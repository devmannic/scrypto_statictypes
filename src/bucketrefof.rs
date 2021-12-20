use scrypto::prelude::*;

use crate::internal::*;
use crate::resourceof::ResourceOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

pub type BucketRefOf<RES> = Of<BucketRef, RES>;


impl<RES> BucketRefOf<RES> {
    /// Returns the resource definition of resources within the bucket.
    #[inline(always)]
    pub fn resource_def(&self) -> ResourceOf<RES> {
        self.inner.resource_def().unchecked_into()
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<BucketRef> for BucketRefOf<RES> {
    fn from(bucketref: BucketRef) -> Self {
        if !runtimechecks::check_address::<RES>(bucketref.resource_address()) {
            // not sure a better error here as with BucketOf and VaultOf
            panic!("BucketRef mismatch");
        }
        if bucketref.amount() <= 0.into() { // check() and contains() both check the amount, choosing to keep these semantics
            panic!("Empty BucketRef");
        }
        bucketref.unchecked_into()
    }
}