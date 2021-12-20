use scrypto::prelude::*;

use crate::internal::*;
use crate::bucketof::BucketOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

pub type ResourceOf<RES> = Of<ResourceDef, RES>;

impl<RES> ResourceOf<RES> {
    /// Mints fungible resources
    #[inline(always)]
    pub fn mint<T: Into<Decimal>>(&self, amount: T, auth: BucketRef) -> BucketOf<RES> {
        self.inner.mint(amount, auth).unchecked_into()
    }

    /// Mints non-fungible resources
    #[inline(always)]
    pub fn mint_nft<T: NftData>(&self, id: u128, data: T, auth: BucketRef) -> BucketOf<RES> {
        self.inner.mint_nft(id, data, auth).unchecked_into()
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn(&self, bucket: BucketOf<RES>) {
        self.inner.burn(bucket.inner)
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn_with_auth(&self, bucket: BucketOf<RES>, auth: BucketRef) {
        self.inner.burn_with_auth(bucket.inner, auth)
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<ResourceDef> for ResourceOf<RES> {
    fn from(resource_def: ResourceDef) -> Self {
        if !runtimechecks::check_address::<RES>(resource_def.address()) {
            // not sure a better error here as with BucketOf and VaultOf
            panic!("ResourceOf mismatch");
        }
        resource_def.unchecked_into()
    }
}