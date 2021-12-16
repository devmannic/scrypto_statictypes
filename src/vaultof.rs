use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use scrypto::prelude::*;

use crate::internal::*;
use crate::bucketof::BucketOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

/// VaultOf

pub struct VaultOf<RES> {
    vault: Vault,
    phantom: PhantomData<RES>,
}

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
        VaultOf::<RES> {
            vault: Vault::with_bucket(bucketof.bucket),
            phantom: PhantomData,
        }
    }

    /// Puts a typed bucket of resources into this vault.
    #[inline(always)]
    pub fn put(&self, other: BucketOf<RES>) {
        //self.vault.put(other.into()) // extra check
        self.vault.put(other.bucket) // no extra check
    }

    /// Takes some amount of resources out of this vault, with typed result.
    #[inline(always)]
    pub fn take<A: Into<Decimal>>(&self, amount: A) -> BucketOf<RES> {
        //self.vault.take(amount).into() // extra check
        BucketOf::<RES> {
            bucket: self.vault.take(amount),
            phantom: PhantomData,
        } // no extra check
    }

    /// Takes all resourced stored in this vault, with typed result.
    #[inline(always)]
    pub fn take_all(&self) -> BucketOf<RES> {
        //self.vault.take_all().into() // extra check
        BucketOf::<RES> {
            bucket: self.vault.take_all(),
            phantom: PhantomData,
        } // no extra check
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
        VaultOf::<RES> {
            vault,
            phantom: PhantomData,
        }
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<RES> From<Vault> for VaultOf<RES> {
    #[inline(always)]
    fn from(vault: Vault) -> Self {
        VaultOf::<RES> {
            vault,
            phantom: PhantomData,
        }
    }
}

// VaultOf <-> Vault

impl<RES> From<VaultOf<RES>> for Vault {
    #[inline(always)]
    fn from(vaultof: VaultOf<RES>) -> Self {
        vaultof.vault
    }
}

impl<RES> Deref for VaultOf<RES> {
    type Target = Vault;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.vault
    }
}
impl<RES> DerefMut for VaultOf<RES> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vault
    }
}

//=====
// SBOR
//=====

use sbor::describe::Type;
use sbor::{Decode, DecodeError, Decoder, TypeId};
use sbor::{Describe, Encode, Encoder};

//=============
// VaultOf SBOR
//=============

impl<RES> TypeId for VaultOf<RES> {
    #[inline(always)]
    fn type_id() -> u8 {
        // look like a Vault
        Vault::type_id()
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: ResourceDecl> Decode for VaultOf<RES> {
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let r = Vault::decode_value(decoder);
        r.map(|vault| vault.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `Vault`'s ResourceDef (Address) with this type "RES"
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: ResourceDecl + 'static> Decode for VaultOf<RES> { // 'static is required only when doing runtime checks because of the static storage used
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let r = Vault::decode_value(decoder);
        r.map(|vault| vault.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `Vault`'s ResourceDef (Address) with this type "RES"
    }
}

impl<RES> Encode for VaultOf<RES> {
    #[inline(always)]
    fn encode_value(&self, encoder: &mut Encoder) {
        self.vault.encode_value(encoder);
    }
}

impl<RES> Describe for VaultOf<RES> {
    fn describe() -> Type {
        Vault::describe()
    }
}