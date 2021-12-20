pub use scrypto::prelude::{Address};

pub trait ResourceDecl {
    const ADDRESS: Option<Address>;
}

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)] // i think this derive is ok since it SHOULD depend on what "C" really is, so not try to derive Clone if C is not Clone, similarly with PartialEq and Eq
pub struct Of<C, RES> {
    pub(crate) inner: C,
    pub(crate) phantom: PhantomData<RES>
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<C, RES> From<C> for Of<C,RES> {
    #[inline(always)]
    fn from(inner: C) -> Self {
        Self {
            inner,
            phantom: PhantomData
        }
    }
}

pub trait UncheckedInto<C, RES> {
    fn unchecked_into(self) -> Of<C, RES>;
}

impl<C, RES> UncheckedInto<C, RES> for C {
    #[inline(always)]
    fn unchecked_into(self) -> Of<C, RES> {
        Of::<C, RES> {
            inner: self,
            phantom: PhantomData
        }
    }
}

// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210
impl<C, RES> From<Of<C,RES>> for (C,) {
    #[inline(always)]
    fn from(of: Of<C,RES>) -> Self {
        (of.inner,)
    }
}

impl<C, RES> Deref for Of<C, RES> {
    type Target = C;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<C, RES> DerefMut for Of<C, RES> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

//=====
// SBOR
//=====

use sbor::describe::Type;
use sbor::{Decode, DecodeError, Decoder, TypeId};
use sbor::{Describe, Encode, Encoder};

//==============
// Of SBOR
//==============

impl<C: TypeId, RES> TypeId for Of<C, RES> {
    #[inline(always)]
    fn type_id() -> u8 {
        // look like a "C"
        C::type_id()
    }
}

impl<C: Encode, RES> Encode for Of<C, RES> {
    #[inline(always)]
    fn encode_value(&self, encoder: &mut Encoder) {
        self.inner.encode_value(encoder);
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<C: Decode + Into<Self>, RES: ResourceDecl> Decode for Of<C, RES> {
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let r = C::decode_value(decoder);
        r.map(|inner| inner.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `C`'s ResourceDef (Address) with this type "RES"
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<C: Decode + Into<Self>, RES: ResourceDecl + 'static> Decode for Of<C, RES> { // 'static is required only when doing runtime checks because of the static storage used
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let r = C::decode_value(decoder);
        r.map(|inner| inner.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `C`'s ResourceDef (Address) with this type "RES"
    }
}

impl<C: Describe, RES> Describe for Of<C, RES> {
    #[inline(always)]
    fn describe() -> Type {
        C::describe()
    }
}
