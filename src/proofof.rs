use std::cell::RefCell;
use std::marker::PhantomData;

use scrypto::prelude::*;

use crate::internal::*;
use crate::resourceof::ResourceOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

// impl_wrapper_struct!(ProofOf<RES>, Proof); // can't use this with Drop, so instead custom implementation below
impl_wrapper_common!(ProofOf<RES>, Proof); // still want the common implementation

// custom ProofOf using RefCell so we can implement Drop
#[derive(Debug)]
pub struct ProofOf<RES: Resource> {
    inner: RefCell<Option<Proof>>,
    phantom: PhantomData<RES>,
}

// the "standard" impl and traits (but we may not have Decode, and we always have a custom Encode)
impl_SBOR_traits_without_Encode_Decode!(ProofOf<RES>, Proof);
#[cfg(feature = "runtime_typechecks")]
impl_SBOR_Decode!(ProofOf<RES>, Proof);

impl SBORable for Proof {}
impl Container for Proof {}
impl_HasResourceAddress!(Proof);

// required for impl_SBOR_traits! and used in forwarder (impl this or impl Deref but not both)
impl<RES: Resource> WithInner<Proof> for ProofOf<RES> {
    #[inline(always)]
    fn with_inner<F: FnOnce(&Proof) -> O, O>(&self, f: F) -> O {
        f(&self.inner.borrow().as_ref().unwrap()) // will panic on already droped Proof
    }
}

// "overrides" where in/out types are changed to Something<RES>
impl<RES: Resource> ProofOf<RES> {
    /// Returns the resource definition of resources within the bucket.
    #[inline(always)]
    pub fn resource_manager(&self) -> ResourceOf<RES> {
        self.with_inner(|inner| inner.resource_address().unchecked_into())
    }
}

// custom impl From<Proof> with runtime checks
#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Proof> for ProofOf<RES> {
    fn from(Proof: Proof) -> Self {
        if !runtimechecks::check_address::<RES>(Proof.resource_address()) {
            // not sure a better error here as with BucketOf and VaultOf
            panic!("Proof mismatch");
        }
        if Proof.amount() <= 0.into() {
            // check() and contains() both check the amount, choosing to keep these semantics
            panic!("Will not create empty ProofOf");
        }
        UncheckedIntoProofOf::unchecked_into(Proof)
    }
}

// choosing to implement this with panic! instead of unchecked_into because Proof is used for autnentication and silently converting at runtime is worse than
// with other types like Vault and Bucket where there is more benefit to allow gradual typing
// custom impl From<Proof> since we can't use impl_wrapper_struct! for ProofOf
#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: Resource> From<Proof> for ProofOf<RES> {
    #[inline(always)]
    fn from(_inner: Proof) -> Self {
        panic!("Unsafe creation of ProofOf from Proof.  Enable scrypto_statictypes/runtime_typechecks or use .unchecked_into()");
        // UncheckedIntoProofOf::unchecked_into(inner)
    }
}

// custom Drop to call .drop() the inner Proof -- which is for Radix Engine and different from drop(Proof)
impl<RES: Resource> Drop for ProofOf<RES> {
    fn drop(&mut self) {
        let opt = self.inner.borrow_mut().take();
        opt.and_then(|proof| {
            debug!("Drop ProofOf {:?}", proof);
            Some(proof.drop())
        });
    }
}

// define how to create a ProofOf<RES>
pub trait UncheckedIntoProofOf<RES: Resource> {
    fn unchecked_into(self) -> ProofOf<RES>;
}
impl<RES: Resource> UncheckedIntoProofOf<RES> for Proof {
    #[inline(always)]
    fn unchecked_into(self) -> ProofOf<RES> {
        ProofOf::<RES> {
            inner: RefCell::new(Some(self)),
            phantom: PhantomData::<RES>,
        }
    }
}

// how to get the Proof with move semantics
impl<RES: Resource> Unwrap for ProofOf<RES> {
    type Value = Proof;

    #[inline(always)]
    fn unwrap(self) -> Self::Value {
        self.inner.borrow_mut().take().unwrap()
    }
}

// "forwarding" implementations because we can't implement Deref while using Drop
impl<RES: Resource> ProofOf<RES> {
    /// Whether this proof includes an ownership proof of any of the given resource.
    #[inline(always)]
    pub fn contains(&self, resource: ResourceAddress) -> bool {
        self.with_inner(|inner| inner.contains(resource))
    }

    /// Returns the resource amount within the bucket.
    #[inline(always)]
    pub fn amount(&self) -> Decimal {
        self.with_inner(|inner| inner.amount())
    }

    /// Returns the resource definition address.
    #[inline(always)]
    pub fn resource_address(&self) -> ResourceAddress {
        self.with_inner(|inner| inner.resource_address())
    }

    /// Returns the keys of all non-fungibles in this bucket.
    ///
    /// # Panics
    /// If the bucket is not a non-fungible bucket.
    #[inline(always)]
    pub fn non_fungible_ids(&self) -> BTreeSet<NonFungibleId> {
        self.with_inner(|inner| inner.non_fungible_ids())
    }

    /// Destroys this reference.
    #[inline(always)]
    pub fn drop(self) {
        self.unwrap().drop()
    }

    /// Checks if the referenced bucket is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.with_inner(|inner| inner.is_empty())
    }
}

// custom Encode that takes the value so it can't be dropped twice (semantics are Encode should own/move the Proof)
impl<RES: Resource> sbor::Encode for ProofOf<RES>
where ProofOf<RES>: WithInner<Proof>
{
    // Encode
    #[inline(always)]
    fn encode_value(&self, encoder: &mut sbor::Encoder) {
        // self.with_inner(|inner| <$t as sbor::Encode>::encode_value(inner, encoder))
        let br: Proof = self.inner.borrow_mut().take().unwrap(); // take so the Drop trait can't drop the Proof
        debug!("Encode ProofOf {:?}", br);
        <Proof as sbor::Encode>::encode_value(&br, encoder) // encode here
                                                                // let Proof go out of scope (it doesn't have Drop)
    }
}
