use scrypto::prelude::*;

use crate::internal::*;
use crate::bucketof::BucketOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

pub type VaultOf<RES> = Of<Vault, RES>;

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> VaultOf<RES> { // runtime_checks requires trait bound on runtimechecks::Resource and use of .into() in new() may have runtime_checks (so we need a different impl block)
    #[inline(always)]
    pub fn new<A: Into<ResourceDef>>(resource_def: A) -> Self {
        Vault::new(resource_def).into()
    }
}

impl<RES> VaultOf<RES> {
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
    pub fn put(&self, other: BucketOf<RES>) {
        //self.vault.put(other.into()) // extra check
        self.inner.put(other.inner) // no extra check
    }

    /// Takes some amount of resources out of this vault, with typed result.
    #[inline(always)]
    pub fn take<A: Into<Decimal>>(&self, amount: A) -> BucketOf<RES> {
        //self.vault.take(amount).into() // extra check
        self.inner.take(amount).unchecked_into() // no extra check
    }

    /// Takes all resourced stored in this vault, with typed result.
    #[inline(always)]
    pub fn take_all(&self) -> BucketOf<RES> {
        //self.vault.take_all().into() // extra check
        self.inner.take_all().unchecked_into() // no extra check
    }
}

// VaultOf<RES>::From<Vault>
#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Vault> for VaultOf<RES> {
    fn from(vault: Vault) -> Self {
        if !runtimechecks::check_address::<RES>(vault.resource_address()) {
            let tmp_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM).initial_supply_fungible(1);
            vault.put(tmp_bucket); // this will trigger resource def mismatch error error: Err(InvokeError(Trap(Trap { kind: Host(VaultError(AccountingError(MismatchingResourceDef))) })))
                                   // shouldn't get here, but just in case (and to help the compiler)
            panic!("VaultOf mismatch");
        }
        vault.unchecked_into()
    }
}
