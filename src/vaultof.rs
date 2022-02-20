use scrypto::prelude::*;

use crate::bucketof::BucketOf;
use crate::bucketrefof::*;
use crate::internal::*;
use crate::resourceof::ResourceOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(VaultOf<RES>, Vault);
impl_SBOR_traits!(VaultOf<RES>, Vault);
impl SBORable for Vault {}
impl Container for Vault {}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> VaultOf<RES> {
    // runtime_checks requires trait bound on runtimechecks::Resource and use of .into() in new() may have runtime_checks (so we need a different impl block)
    #[inline(always)]
    pub fn new<A: Into<ResourceDef>>(resource_def: A) -> Self {
        Vault::new(resource_def).into()
    }
}

impl<RES: Resource> VaultOf<RES> {
    #[cfg(not(feature = "runtime_typechecks"))]
    #[inline(always)]
    pub fn new<A: Into<ResourceDef>>(resource_def: A) -> Self {
        Vault::new(resource_def).into()
    }

    #[inline(always)]
    pub fn with_bucket(bucketof: BucketOf<RES>) -> VaultOf<RES> {
        Vault::with_bucket(bucketof.inner).unchecked_into()
    }

    /// Puts a typed bucket of resources into this vault.
    #[inline(always)]
    pub fn put(&mut self, other: BucketOf<RES>) {
        // self.vault.put(other.into()) // extra check
        self.inner.put(other.inner) // no extra check
    }

    /// Takes some amount of resources out of this vault, with typed result.
    #[inline(always)]
    pub fn take<A: Into<Decimal>>(&mut self, amount: A) -> BucketOf<RES> {
        // self.vault.take(amount).into() // extra check
        self.inner.take(amount).unchecked_into() // no extra check
    }

    /// Takes some amount of resource from this vault into a bucket.
    ///
    /// This variant of `take` accepts an additional auth parameter to support resources
    /// with or without `RESTRICTED_TRANSFER` flag on.
    #[inline(always)]
    pub fn take_with_auth<A: Into<Decimal>, AUTH: Resource>(
        &mut self,
        amount: A,
        auth: BucketRefOf<AUTH>,
    ) -> BucketOf<RES> {
        self.inner
            .take_with_auth(amount, auth.unwrap())
            .unchecked_into()
    }

    /// Takes all resourced stored in this vault, with typed result.
    #[inline(always)]
    pub fn take_all(&mut self) -> BucketOf<RES> {
        // self.vault.take_all().into() // extra check
        self.inner.take_all().unchecked_into() // no extra check
    }

    /// Takes all resource stored in this vault.
    ///
    /// This variant of `take_all` accepts an additional auth parameter to support resources
    /// with or without `RESTRICTED_TRANSFER` flag on.
    #[inline(always)]
    pub fn take_all_with_auth<AUTH: Resource>(&mut self, auth: BucketRefOf<AUTH>) -> BucketOf<RES> {
        self.inner
            .take_all_with_auth(auth.unwrap())
            .unchecked_into()
    }

    /// Takes a non-fungible from this vault, by id.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible vault or the specified non-fungible is not found.
    #[inline(always)]
    pub fn take_non_fungible(&self, key: &NonFungibleKey) -> BucketOf<RES> {
        self.inner.take_non_fungible(key).unchecked_into()
    }

    /// Takes a non-fungible from this vault, by id.
    ///
    /// This variant of `take_non_fungible` accepts an additional auth parameter to support resources
    /// with or without `RESTRICTED_TRANSFER` flag on.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible vault or the specified non-fungible is not found.
    #[inline(always)]
    pub fn take_non_fungible_with_auth<AUTH: Resource>(&self, key: &NonFungibleKey, auth: BucketRefOf<AUTH>) -> BucketOf<RES> {
        self.inner
            .take_non_fungible_with_auth(key, auth.unwrap())
            .unchecked_into()
    }

    /// Returns the resource definition of resources within this vault.
    #[inline(always)]
    pub fn resource_def(&self) -> ResourceOf<RES> {
        self.inner.resource_def().unchecked_into()
    }

    /// This is a convenience method for using the contained resource for authorization.
    ///
    /// It conducts the following actions in one shot:
    /// 1. Takes `1` resource from this vault into a bucket;
    /// 2. Creates a `BucketRef`.
    /// 3. Applies the specified function `f` with the created bucket reference;
    /// 4. Puts the `1` resource back into this vault.
    pub fn authorize<F: FnOnce(BucketRefOf<RES>) -> O, O>(&mut self, f: F) -> O {
        let bucket = self.take(1);
        let output = f(bucket.present());
        self.put(bucket);
        output
    }

    /// This is a convenience method for using the contained resource for authorization.
    ///
    /// It conducts the following actions in one shot:
    /// 1. Takes `1` resource from this vault into a bucket;
    /// 2. Creates a `BucketRef`.
    /// 3. Applies the specified function `f` with the created bucket reference;
    /// 4. Puts the `1` resource back into this vault.
    ///
    /// This variant of `authorize` accepts an additional auth parameter to support resources
    /// with or without `RESTRICTED_TRANSFER` flag on.
    pub fn authorize_with_auth<F: FnOnce(BucketRefOf<RES>) -> O, O, AUTH: Resource>(
        &mut self,
        f: F,
        auth: BucketRefOf<AUTH>,
    ) -> O {
        let bucket = self.take_with_auth(1, auth);
        let output = f(bucket.present());
        self.put(bucket);
        output
    }
}

// VaultOf<RES>::From<Vault>
#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Vault> for VaultOf<RES> {
    fn from(vault: Vault) -> Self {
        if !runtimechecks::check_address::<RES>(vault.resource_address()) {
            // let tmp_bucket =
            //     ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM).initial_supply_fungible(1);
            // vault.put(tmp_bucket); // this will trigger resource def mismatch error error: Err(InvokeError(Trap(Trap { kind: Host(VaultError(AccountingError(MismatchingResourceDef))) })))
                                   // shouldn't get here, but just in case (and to help the compiler)
            panic!("VaultOf mismatch");
        }
        vault.unchecked_into()
    }
}
