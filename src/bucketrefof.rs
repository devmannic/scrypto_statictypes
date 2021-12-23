use std::cell::{RefCell};
use std::marker::PhantomData;

use scrypto::prelude::*;

use crate::internal::*;
use crate::resourceof::ResourceOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

//impl_wrapper_struct!(BucketRefOf<RES>, Bucket); // can't use this with Drop, so instead custom implementation below

// custom BucketRefOf using RefCell so we can implement Drop
#[derive(Debug)]
pub struct BucketRefOf<RES: Resource> {
    inner: RefCell<Option<BucketRef>>,
    phantom: PhantomData<RES>
}

// the "standard" impl and traits
impl_SBOR_traits!(BucketRefOf<RES>, BucketRef);
impl SBORable for BucketRef {}
impl Container for BucketRef {}


// required for impl_SBOR_traits! and used in forwarder (impl this or impl Deref but not both)
impl<RES: Resource> WithInner<BucketRef> for BucketRefOf<RES> {
    #[inline(always)]
    fn with_inner<F: FnOnce(&BucketRef) -> O, O>(&self, f: F) -> O {
        f(&self.inner.borrow().as_ref().unwrap()) // will panic on already droped BucketRef
    }
}

// "overrides" where in/out types are changed to Something<RES>
impl<RES: Resource> BucketRefOf<RES> {
    /// Returns the resource definition of resources within the bucket.
    #[inline(always)]
    pub fn resource_def(&self) -> ResourceOf<RES> {
        self.with_inner(|inner| inner.resource_def().unchecked_into())
    }
}

// custom impl From<BucketRef> with runtime checks
#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<BucketRef> for BucketRefOf<RES> {
    fn from(bucketref: BucketRef) -> Self {
        if !runtimechecks::check_address::<RES>(bucketref.resource_address()) {
            // not sure a better error here as with BucketOf and VaultOf
            panic!("BucketRef mismatch");
        }
        if bucketref.amount() <= 0.into() { // check() and contains() both check the amount, choosing to keep these semantics
            panic!("Will not create empty BucketRefOf");
        }
        UncheckedIntoBucketRefOf::unchecked_into(bucketref)
    }
}

// custom impl From<BucketRef> since we can't use impl_wrapper_struct! for BucketRefOf
#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: Resource> From<BucketRef> for BucketRefOf<RES> {
    #[inline(always)]
    fn from(_inner: BucketRef) -> Self {
        panic!("Unsafe creation of BucketRefOf from BucketRef.  Enable scrypto_statictypes/runtime_typechecks or use .unchecked_into()");
        //UncheckedIntoBucketRefOf::unchecked_into(inner)
    }
}

// custom Drop to call .drop() the inner BucketRef -- which is for Radix Engine and different from drop(BucketRef)
impl<RES: Resource> Drop for BucketRefOf<RES> {
    fn drop(&mut self) {
        debug!("dropping BucketRefOf {:?}", self);
        let opt = self.inner.borrow_mut().take();
        opt.and_then(|bucketref| Some(bucketref.drop()));
    }
}

// define how to create a BucketRefOf<RES>
pub trait UncheckedIntoBucketRefOf<RES: Resource> {
    fn unchecked_into(self) -> BucketRefOf<RES>;
}
impl<RES: Resource> UncheckedIntoBucketRefOf<RES> for BucketRef {
    #[inline(always)]
    fn unchecked_into(self) -> BucketRefOf<RES> {
        BucketRefOf::<RES> {
            inner: RefCell::new(Some(self)),
            phantom: PhantomData::<RES>
        }
    }
}

// how to get the BucketRef with move semantics
pub trait Mover {
    type Inner;
    fn inner(self) -> Self::Inner;
}
impl<RES: Resource> Mover for BucketRefOf<RES> {
    type Inner = BucketRef;

    #[inline(always)]
    fn inner(self) -> Self::Inner {
        self.inner.borrow_mut().take().unwrap()
    }
}

// "forwarding" implementations because we can't implement Deref while using Drop
impl<RES: Resource> BucketRefOf<RES> {
    /// Checks if the referenced bucket contains the given resource, and aborts if not so.
    pub fn check<A: Into<ResourceDef>>(self, resource_def: A) {
        self.inner().check(resource_def)
    }

    /// Checks if the referenced bucket contains the given resource.
    #[inline(always)]
    pub fn contains<A: Into<ResourceDef>>(&self, resource_def: A) -> bool {
        self.with_inner(|inner| inner.contains(resource_def))
    }

    /// Returns the resource amount within the bucket.
    #[inline(always)]
    pub fn amount(&self) -> Decimal {
        self.with_inner(|inner| inner.amount())
    }

    /// Returns the resource definition of resources within the bucket.
    // pub fn resource_def(&self) -> ResourceDef {
    //     self.deref().resource_def()
    // }

    /// Returns the resource definition address.
    #[inline(always)]
    pub fn resource_address(&self) -> Address {
        self.with_inner(|inner| inner.resource_address())
    }

    /// Get the NFT ids in the referenced bucket.
    #[inline(always)]
    pub fn get_nft_ids(&self) -> Vec<u128> {
        self.with_inner(|inner| inner.get_nft_ids())
    }

    /// Get the NFT id and panic if not singleton.
    #[inline(always)]
    pub fn get_nft_id(&self) -> u128 {
        self.with_inner(|inner| inner.get_nft_id())
    }

    /// Destroys this reference.
    #[inline(always)]
    pub fn drop(self) {
        self.inner().drop()
    }

    /// Checks if the referenced bucket is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.with_inner(|inner| inner.is_empty())
    }
}