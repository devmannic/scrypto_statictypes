use scrypto::prelude::*;

use crate::internal::*;
use crate::bucketof::BucketOf;
use crate::bucketrefof::*;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(ResourceOf<RES>, ResourceDef);
impl_SBOR_traits!(ResourceOf<RES>, ResourceDef);
impl SBORable for ResourceDef {}
impl Container for ResourceDef {}

impl<RES: Resource> ResourceOf<RES> {
    /// Mints fungible resources
    #[inline(always)]
    pub fn mint<T: Into<Decimal>, AUTH: Resource>(&self, amount: T, auth: BucketRefOf<AUTH>) -> BucketOf<RES> {
        self.inner.mint(amount, auth.unwrap()).unchecked_into()
    }

    /// Mints non-fungible resources
    #[inline(always)]
    pub fn mint_nft<T: NftData, AUTH: Resource>(&self, id: u128, data: T, auth: BucketRefOf<AUTH>) -> BucketOf<RES> {
        self.inner.mint_nft(id, data, auth.unwrap()).unchecked_into()
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn(&self, bucket: BucketOf<RES>) {
        self.inner.burn(bucket.inner)
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn_with_auth<AUTH: Resource>(&self, bucket: BucketOf<RES>, auth: BucketRefOf<AUTH>) {
        self.inner.burn_with_auth(bucket.inner, auth.unwrap())
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