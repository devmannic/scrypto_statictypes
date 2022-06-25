use scrypto::prelude::*;

use crate::bucketof::BucketOf;
use crate::proofof::*;
use crate::internal::*;
use crate::resourceof::ResourceOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

impl_wrapper_struct!(VaultOf<RES>, Vault);
impl_SBOR_traits!(VaultOf<RES>, Vault);
impl SBORable for Vault {}
impl Container for Vault {}
impl_HasResourceAddress!(Vault);

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> VaultOf<RES> {
    // runtime_checks requires trait bound on runtimechecks::Resource and use of .into() in new() may have runtime_checks (so we need a different impl block)
    /// Creates an empty vault to permanently hold resource of the given definition.
    #[inline(always)]
    pub fn new(resource_address: ResourceAddress) -> Self {
        Vault::new(resource_address).into()
    }
}

impl<RES: Resource> VaultOf<RES> {
    /// Creates an empty vault to permanently hold resource of the given definition.
    #[cfg(not(feature = "runtime_typechecks"))]
    #[inline(always)]
    pub fn new(resource_address: ResourceAddress) -> Self {
        Vault::new(resource_address).into()
    }

    /// Creates an empty vault and fills it with an initial bucket of resource.
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

    /// Takes all resourced stored in this vault, with typed result.
    #[inline(always)]
    pub fn take_all(&mut self) -> BucketOf<RES> {
        // self.vault.take_all().into() // extra check
        self.inner.take_all().unchecked_into() // no extra check
    }

    /// Takes a specific non-fungible from this vault.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible vault or the specified non-fungible resource is not found.
    #[inline(always)]
    pub fn take_non_fungible(&mut self, non_fungible_id: &NonFungibleId) -> BucketOf<RES> {
        self.inner.take_non_fungible(non_fungible_id).unchecked_into()
    }

    /// Takes non-fungibles from this vault.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible vault or the specified non-fungible resource is not found.
    #[inline(always)]
    pub fn take_non_fungibles(&mut self, non_fungible_ids: &BTreeSet<NonFungibleId>) -> BucketOf<RES> {
        self.inner.take_non_fungibles(non_fungible_ids).unchecked_into()
    }

    /// Creates an ownership proof of this vault.
    #[inline(always)]
    pub fn create_proof(&self) -> ProofOf<RES> {
        // self.inner.create_proof().unchecked_into()
        UncheckedIntoProofOf::unchecked_into(self.inner.create_proof())
    }

    /// Creates an ownership proof of this vault, by amount.
    #[inline(always)]
    pub fn create_proof_by_amount(&self, amount: Decimal) -> ProofOf<RES> {
        UncheckedIntoProofOf::unchecked_into(self.inner.create_proof_by_amount(amount))
    }

    /// Creates an ownership proof of this vault, by non-fungible ID set.
    #[inline(always)]
    pub fn create_proof_by_ids(&self, ids: &BTreeSet<NonFungibleId>) -> ProofOf<RES> {
        UncheckedIntoProofOf::unchecked_into(self.inner.create_proof_by_ids(ids))
    }

    /// Returns the resource definition of resources within this vault.
    #[inline(always)]
    pub fn resource_manager(&self) -> ResourceOf<RES> {
        self.inner.resource_address().unchecked_into()
    }
}

impl_TryFrom_Slice!(VaultOf<RES>, ParseVaultError);

// VaultOf<RES>::From<Vault>
#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Vault> for VaultOf<RES> {
    fn from(vault: Vault) -> Self {
        if !runtimechecks::check_address::<RES>(vault.resource_address()) {
            // let tmp_bucket =
            //     ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM).initial_supply_fungible(1);
            // vault.put(tmp_bucket); // this will trigger resource def mismatch error error: Err(InvokeError(Trap(Trap { kind: Host(VaultError(AccountingError(MismatchingResourceManager))) })))
                                   // shouldn't get here, but just in case (and to help the compiler)
            panic!("VaultOf mismatch");
        }
        vault.unchecked_into()
    }
}
