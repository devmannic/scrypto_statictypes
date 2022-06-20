use scrypto::prelude::*;

use crate::bucketof::BucketOf;
use crate::proofof::*;
use crate::internal::*;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(ResourceOf<RES>, ResourceManager);
impl_SBOR_traits!(ResourceOf<RES>, ResourceManager);
impl SBORable for ResourceManager {}
impl Container for ResourceManager {}

impl<RES: Resource> ResourceOf<RES> {
    /// Mints fungible resources
    #[inline(always)]
    pub fn mint<T: Into<Decimal>>(&self, amount: T) -> BucketOf<RES> {
        self.inner.mint(amount).unchecked_into()
    }

    /// Mints non-fungible resources
    #[inline(always)]
    pub fn mint_non_fungible<T: NonFungibleData>(&self, id: &NonFungibleId, data: T) -> BucketOf<RES> {
        self.inner
            .mint_non_fungible(id, data)
            .unchecked_into()
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn(&self, bucket: BucketOf<RES>) {
        self.inner.burn(bucket.inner)
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<ResourceManager> for ResourceOf<RES> {
    fn from(resource_manager: ResourceManager) -> Self {
        if !runtimechecks::check_address::<RES>(resource_manager.address()) {
            // not sure a better error here as with BucketOf and VaultOf
            panic!("ResourceOf mismatch");
        }
        resource_manager.unchecked_into()
    }
}

/*
//  XXX no longer possible in v0.4.0 because ResourceAddress managed by ResourceManager is pub(crate) only

// Implement == and != between ResourceManager and ResourceOf

impl<RES: Resource> PartialEq<ResourceOf<RES>> for ResourceManager {
    #[inline(always)]
    fn eq(&self, other: &ResourceOf<RES>) -> bool {
        self.address() == other.address()
    }
}

impl<RES: Resource> PartialEq<ResourceManager> for ResourceOf<RES> {
    #[inline(always)]
    fn eq(&self, other: &ResourceManager) -> bool {
        self.address() == other.address()
    }
}

impl<RES: Resource> PartialEq<ResourceOf<RES>> for ResourceOf<RES> {
    #[inline(always)]
    fn eq(&self, other: &ResourceOf<RES>) -> bool {
        self.address() == other.address()
    }
}
*/
