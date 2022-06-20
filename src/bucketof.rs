use scrypto::prelude::*;

use crate::proofof::*;
use crate::internal::*;
use crate::resourceof::ResourceOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(BucketOf<RES>, Bucket);
impl_SBOR_traits!(BucketOf<RES>, Bucket);
impl SBORable for Bucket {}
impl Container for Bucket {}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> BucketOf<RES> {
    // use of .into() when runtime_checks requires trait bound on runtimechecks::Resource because of From trait bound (so we need a different impl block)
    /// Creates a new bucket to hold resources of the given definition.
    #[inline(always)]
    pub fn new<A: Into<ResourceManager>>(resource_manager: A) -> Self {
        Bucket::new(resource_manager).into()
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: ResourceDecl> BucketOf<RES> {
    // use of .into() when not(runtime_checks) requires trait bound on ResourceDecl because of From trait bound (so we need a different impl block)
    /// Creates a new bucket to hold resources of the given definition.
    #[inline(always)]
    pub fn new(resource_address: ResourceAddress) -> Self {
        Bucket::new(resource_address).into()
    }
}

impl<RES: Resource> BucketOf<RES> {
    /// Puts resources from another bucket into this bucket.
    #[inline(always)]
    pub fn put(&mut self, other: Self) {
        self.inner.put(other.inner)
    }

    /// Takes some amount of resources from this bucket.
    #[inline(always)]
    pub fn take<A: Into<Decimal>>(&mut self, amount: A) -> Self {
        self.inner.take(amount).unchecked_into()
    }

    /// Burns resource within this bucket.
    #[inline(always)]
    pub fn burn(self) {
        // must define this instead of leaning on Deref because of self not &self (needs DerefMove which doesn't exist yet)
        self.inner.burn();
    }

    /// Returns the resource definition of resources in this bucket.
    #[inline(always)]
    pub fn resource_address(&self) -> ResourceOf<RES> {
        self.inner.resource_address().unchecked_into()
    }

    /// Creates an immutable reference to this bucket.
    #[inline(always)]
    pub fn create_proof(&self) -> ProofOf<RES> {
        // self.inner.create_proof().unchecked_into()
        UncheckedIntoProofOf::unchecked_into(self.inner.create_proof())
    }

    /// Uses resources in this bucket as authorization for an operation.
    #[inline(always)]
    pub fn authorize<F: FnOnce(ProofOf<RES>) -> O, O>(&self, f: F) -> O {
        f(self.create_proof())
    }

    /// Takes a non-fungible from this bucket, by key.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible bucket or the specified non-fungible resource is not found.
    #[inline(always)]
    pub fn take_non_fungible(&mut self, key: &NonFungibleId) -> BucketOf<RES> {
        self.inner.take_non_fungible(key).unchecked_into()
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Bucket> for BucketOf<RES> {
    fn from(bucket: Bucket) -> Self {
        if !runtimechecks::check_address::<RES>(bucket.resource_address()) {
            // let tmp_bucket =
            //     ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM).initial_supply_fungible(1);
            // bucket.put(tmp_bucket); // this will trigger resource def mismatch error: Err(InvokeError(Trap(Trap { kind: Host(BucketError(MismatchingResourceManager)) })))
                                    // shouldn't get here, but just in case (and to help the compiler)
            panic!("BucketOf mismatch");
        }
        bucket.unchecked_into()
    }
}
