use scrypto::prelude::*;

use crate::bucketof::BucketOf;
use crate::bucketrefof::*;
use crate::internal::*;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(ResourceOf<RES>, ResourceDef);
impl_SBOR_traits!(ResourceOf<RES>, ResourceDef);
impl SBORable for ResourceDef {}
impl Container for ResourceDef {}

impl<RES: Resource> ResourceOf<RES> {
    /// Mints fungible resources
    #[inline(always)]
    pub fn mint<T: Into<Decimal>, AUTH: Resource>(
        &mut self,
        amount: T,
        auth: BucketRefOf<AUTH>,
    ) -> BucketOf<RES> {
        self.inner.mint(amount, auth.unwrap()).unchecked_into()
    }

    /// Mints non-fungible resources
    #[inline(always)]
    pub fn mint_non_fungible<T: NonFungibleData, AUTH: Resource>(
        &mut self,
        key: &NonFungibleKey,
        data: T,
        auth: BucketRefOf<AUTH>,
    ) -> BucketOf<RES> {
        self.inner
            .mint_non_fungible(key, data, auth.unwrap())
            .unchecked_into()
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn(&mut self, bucket: BucketOf<RES>) {
        self.inner.burn(bucket.inner)
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn_with_auth<AUTH: Resource>(&mut self, bucket: BucketOf<RES>, auth: BucketRefOf<AUTH>) {
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

// Implement == and != between ResourceDef and ResourceOf

impl<RES: Resource> PartialEq<ResourceOf<RES>> for ResourceDef {
    #[inline(always)]
    fn eq(&self, other: &ResourceOf<RES>) -> bool {
        self.address() == other.address()
    }
}

impl<RES: Resource> PartialEq<ResourceDef> for ResourceOf<RES> {
    #[inline(always)]
    fn eq(&self, other: &ResourceDef) -> bool {
        self.address() == other.address()
    }
}

impl<RES: Resource> PartialEq<ResourceOf<RES>> for ResourceOf<RES> {
    #[inline(always)]
    fn eq(&self, other: &ResourceOf<RES>) -> bool {
        self.address() == other.address()
    }
}