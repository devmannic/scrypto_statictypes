use std::ops::Deref;

pub use scrypto::prelude::{ResourceAddress, ResourceManager};

pub trait Resource: std::fmt::Debug {} // supertrait to ensure the correct traits propate to all of the templates

pub trait ResourceDecl: Resource {
    const ADDRESS: Option<ResourceAddress>;
}

pub trait Container: SBORable {}

pub trait HasResourceAddress {
    fn _resource_address(&self) -> ResourceAddress;
    fn borrow_resource_manager(&self) -> &ResourceManager {
        use scrypto::prelude::*; // make sure the macro works
        borrow_resource_manager!(HasResourceAddress::_resource_address(self))
    }
}

macro_rules! impl_HasResourceAddress {
    ( $w:ty ) => {
        impl HasResourceAddress for $w {
            #[inline(always)]
            fn _resource_address(&self) -> ResourceAddress {
                self.resource_address()
            }
        }
    }
}
pub(crate) use impl_HasResourceAddress; // export for use within crate

//=====
// SBOR
//=====

use sbor::describe::Type;
use sbor::{Decode, DecodeError, Decoder, TypeId};
use sbor::{Describe, Encode, Encoder};

//==============
// Generic SBOR for Wrapper
//==============

// trait grouping
pub trait SBORable: TypeId + Encode + Decode + Describe {
    // TypeId
    #[inline(always)]
    fn type_id() -> u8 {
        <Self as TypeId>::type_id()
    }

    // Encode
    #[inline(always)]
    fn encode_value(&self, encoder: &mut Encoder) {
        <Self as Encode>::encode_value(self, encoder)
    }

    // Decode
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        <Self as Decode>::decode_value(decoder)
    }

    // Describe
    #[inline(always)]
    fn describe() -> Type {
        <Self as Describe>::describe()
    }
}

// abstract how the inner value is retrieved for use in encode_value (or elsewhere)
pub trait WithInner<T> {
    type Inner;
    fn with_inner<F: FnOnce(&T) -> O, O>(&self, f: F) -> O;
}
// for anything that supports Deref, just use that (but only implement on our Containers)
impl<T: Container, W: Deref<Target = T>> WithInner<T> for W {
    type Inner = T;
    #[inline(always)]
    fn with_inner<F: FnOnce(&T) -> O, O>(&self, f: F) -> O {
        f(self)
    }
}

macro_rules! impl_SBOR_traits {
    ( $w:ty, $t:ident ) => {
        impl_SBOR_traits_without_Encode_Decode!($w, $t);
        impl_SBOR_Encode!($w, $t);
        impl_SBOR_Decode!($w, $t);
    };
}
pub(crate) use impl_SBOR_traits; // export for use within crate

// generate the SBOR traits with $w wrapping type $t
// requires $w: WithInner<$t> and $t: From<$w>
macro_rules! impl_SBOR_traits_without_Encode_Decode {
    ( $w:ty, $t:ident ) => {
        // use std::ops::{Deref};
        // use sbor::describe::Type;
        // use sbor::{Decode, DecodeError, Decoder, TypeId};
        // use sbor::{Describe, Encode, Encoder};
        impl<RES: Resource> sbor::TypeId for $w {
            // TypeId
            #[inline(always)]
            fn type_id() -> u8 {
                // look like a "T"
                <$t as sbor::TypeId>::type_id()
            }
        }

        impl<RES: Resource> sbor::Describe for $w {
            // Describe
            #[inline(always)]
            fn describe() -> sbor::describe::Type {
                <$t as sbor::Describe>::describe()
            }
        }
    };
}

pub(crate) use impl_SBOR_traits_without_Encode_Decode; // export for use within crate

//==============
// Main Wrapper implementation
//==============

pub trait UncheckedInto<RES: Resource, W> {
    fn unchecked_into(self) -> W;
}

pub trait Unwrap {
    type Value;
    fn unwrap(self) -> Self::Value;
}

macro_rules! impl_wrapper_struct {
    ( $w:ident<RES>, $t:ty, noderef ) => {
        #[derive(Debug, PartialEq, Eq, Hash)] // Bucket, Proof, Vault are inconsistent, deriving superset (and Proof doesn't use this macro)
        pub struct $w<RES> {
            pub(crate) inner: $t,
            pub(crate) phantom: std::marker::PhantomData<RES>,
        }
        impl<RES: Resource> Unwrap for $w<RES> {
            type Value = $t;

            #[inline(always)]
            fn unwrap(self) -> Self::Value {
                self.inner
            }
        }
        impl<RES: Resource> UncheckedInto<RES, $w<RES>> for $t {
            #[inline(always)]
            fn unchecked_into(self) -> $w<RES> {
                $w::<RES> {
                    inner: self,
                    phantom: std::marker::PhantomData::<RES>,
                }
            }
        }
        #[cfg(not(feature = "runtime_typechecks"))]
        impl<RES: Resource> From<$t> for $w<RES> {
            #[inline(always)]
            fn from(inner: $t) -> Self {
                inner.unchecked_into()
            }
        }

        impl_wrapper_common!($w<RES>, $t);
    };
    ( $w:ident<RES>, $t:ty) => {
        impl_wrapper_struct!($w<RES>, $t, noderef);
        impl_wrapper_deref!($w<RES>, $t);
    };
}
pub(crate) use impl_wrapper_struct; // export for use within crate

// seperate out deref
macro_rules! impl_wrapper_deref {
    ( $w:ident<RES>, $t:ty ) => {
        impl<RES: Resource> std::ops::Deref for $w<RES> {
            type Target = $t;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<RES: Resource> std::ops::DerefMut for $w<RES> {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}
pub(crate) use impl_wrapper_deref; // export for use within crate

// common implementations that only depend on traits of $t or $w
// and are not expected to need custom implementations (like From<$t> for $w<RES> when not(feature = "runtime_typechecks"))
macro_rules! impl_wrapper_common {
    ( $w:ident<RES>, $t:ty ) => {
        impl<RES: Resource> From<$w<RES>> for $t {
            #[inline(always)]
            fn from(wrapped: $w<RES>) -> Self {
                wrapped.unwrap()
            }
        }
    };
}

pub(crate) use impl_wrapper_common; // export for use within crate

macro_rules! impl_SBOR_Encode {
    ( $w:ty, $t:ident ) => {
        // use std::ops::{Deref};
        // use sbor::describe::Type;
        // use sbor::{Decode, DecodeError, Decoder, TypeId};
        // use sbor::{Describe, Encode, Encoder};
        impl<RES: Resource> sbor::Encode for $w
        where $w: WithInner<$t>
        {
            // Encode
            #[inline(always)]
            fn encode_value(&self, encoder: &mut sbor::Encoder) {
                self.with_inner(|inner| <$t as sbor::Encode>::encode_value(inner, encoder))
            }
        }
    };
}

pub(crate) use impl_SBOR_Encode; // export for use within crate

macro_rules! impl_SBOR_Decode {
    ( $w:ty, $t:ident ) => {
        // use std::ops::{Deref};
        // use sbor::describe::Type;
        // use sbor::{Decode, DecodeError, Decoder, TypeId};
        // use sbor::{Describe, Encode, Encoder};
        // .into implementation needs a ResourceDecl or a runtimechecks::Resource
        #[cfg(not(feature = "runtime_typechecks"))]
        impl<RES: ResourceDecl> sbor::Decode for $w {
            // Decode
            #[inline(always)]
            fn decode_value(decoder: &mut sbor::Decoder) -> Result<Self, sbor::DecodeError> {
                let r = <$t as sbor::Decode>::decode_value(decoder);
                r.map(|inner| inner.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `T`'s ResourceManager (Address) with this type "RES"
            }
        }

        // .into implementation needs a ResourceDecl or a runtimechecks::Resource
        #[cfg(feature = "runtime_typechecks")]
        impl<RES: runtimechecks::Resource> sbor::Decode for $w {
            // Decode
            #[inline(always)]
            fn decode_value(decoder: &mut sbor::Decoder) -> Result<Self, sbor::DecodeError> {
                let r = <$t as sbor::Decode>::decode_value(decoder);
                r.map(|inner| inner.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `T`'s ResourceManager (Address) with this type "RES"
            }
        }
    };
}

pub(crate) use impl_SBOR_Decode; // export for use within crate

macro_rules! impl_TryFrom_Slice {
    ( $w:ty, $e:ident ) => {
        // runtime_checks requires trait bound on runtimechecks::Resource and use of .into() may have runtime_checks (so we need a different impl block)
        #[cfg(feature = "runtime_typechecks")]
        impl<RES: runtimechecks::Resource> TryFrom<&[u8]> for $w {
            type Error = $e;

            fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
                Ok(<$w as WithInner<_>>::Inner::try_from(slice)?.into())
            }
        }

        #[cfg(not(feature = "runtime_typechecks"))]
        impl<RES: Resource> TryFrom<&[u8]> for $w {
            type Error = $e;

            fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
                Ok(<$w as WithInner<_>>::Inner::try_from(slice)?.into())
            }
        }
    };
}

pub(crate) use impl_TryFrom_Slice; // export for use within crate