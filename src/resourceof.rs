use scrypto::prelude::*;

use crate::bucketof::BucketOf;
use crate::internal::*;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(ResourceOf<RES>, ResourceAddress);
impl_SBOR_traits!(ResourceOf<RES>, ResourceAddress);
impl SBORable for ResourceAddress {}
impl Container for ResourceAddress {}

impl<RES: Resource> HasResourceAddress for ResourceOf<RES> {
    fn _resource_address(&self) -> ResourceAddress {
        self.inner
    }
}

impl<RES: Resource> ResourceOf<RES> {
    /// Mints fungible resources
    #[inline(always)]
    pub fn mint<T: Into<Decimal>>(&self, amount: T) -> BucketOf<RES> {
        self.borrow_resource_manager().mint(amount).unchecked_into()
    }

    /// Mints non-fungible resources
    #[inline(always)]
    pub fn mint_non_fungible<T: NonFungibleData>(&self, id: &NonFungibleId, data: T) -> BucketOf<RES> {
        self.borrow_resource_manager()
            .mint_non_fungible(id, data)
            .unchecked_into()
    }

    /// Burns a bucket of resources.
    #[inline(always)]
    pub fn burn(&self, bucket: BucketOf<RES>) {
        self.borrow_resource_manager().burn(bucket.inner)
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<ResourceAddress> for ResourceOf<RES> {
    fn from(resource_address: ResourceAddress) -> Self {
        if !runtimechecks::check_address::<RES>(resource_address) {
            // not sure a better error here as with BucketOf and VaultOf
            panic!("ResourceOf mismatch");
        }
        resource_address.unchecked_into()
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
