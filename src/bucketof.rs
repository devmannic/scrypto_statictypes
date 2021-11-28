use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use scrypto::prelude::*;

use crate::internal::*;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

/// BucketOf

pub struct BucketOf<RES> {
    pub(crate) bucket: Bucket,
    pub(crate) phantom: PhantomData<RES>,
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> BucketOf<RES> { // use of .into() when runtime_checks requires trait bound on runtimechecks::Resource because of From trait bound (so we need a different impl block)
    /// Creates a new bucket to hold resources of the given definition.
    #[inline(always)]
    pub fn new<A: Into<ResourceDef>>(resource_def: A) -> Self {
        Bucket::new(resource_def).into()
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: ResourceDecl> BucketOf<RES> { // use of .into() when not(runtime_checks) requires trait bound on ResourceDecl because of From trait bound (so we need a different impl block)
    /// Creates a new bucket to hold resources of the given definition.
    #[inline(always)]
    pub fn new<A: Into<ResourceDef>>(resource_def: A) -> Self {
        Bucket::new(resource_def).into()
    }
}

impl<RES> BucketOf<RES> {
    /// Puts resources from another bucket into this bucket.
    #[inline(always)]
    pub fn put(&self, other: Self) {
        self.bucket.put(other.bucket)
    }
    /// Takes some amount of resources from this bucket.
    #[inline(always)]
    pub fn take<A: Into<Decimal>>(&self, amount: A) -> Self {
        BucketOf::<RES> {
            bucket: self.bucket.take(amount),
            phantom: PhantomData,
        }
    }
    /// Burns resource within this bucket.
    #[inline(always)]
    pub fn burn(self, minter: BucketRef) { // must define this instead of using leaning on Deref because of self not &self (needs DerefMove which doesn't exist yet)
        self.bucket.burn(minter);
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Bucket> for BucketOf<RES> {
    fn from(bucket: Bucket) -> Self {
        if !runtimechecks::check_address::<RES>(bucket.resource_address()) {
            let tmp_bucket = ResourceBuilder::new().new_token_fixed(1);
            bucket.put(tmp_bucket); // this will trigger resource def mismatch error: Err(InvokeError(Trap(Trap { kind: Host(BucketError(MismatchingResourceDef)) })))
                                    // shouldn't get here, but just in case (and to help the compiler)
            panic!("BucketOf mismatch");
        }
        BucketOf::<RES> {
            bucket,
            phantom: PhantomData,
        }
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: ResourceDecl> From<Bucket> for BucketOf<RES> {
    #[inline(always)]
    fn from(bucket: Bucket) -> Self {
        BucketOf::<RES> {
            bucket,
            phantom: PhantomData,
        }
    }
}

impl<RES> From<BucketOf<RES>> for Bucket {
    #[inline(always)]
    fn from(bucketof: BucketOf<RES>) -> Self {
        bucketof.bucket
    }
}

impl<RES> Deref for BucketOf<RES> {
    type Target = Bucket;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bucket
    }
}
impl<RES> DerefMut for BucketOf<RES> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bucket
    }
}

//=====
// SBOR
//=====

use sbor::describe::Type;
use sbor::{Decode, DecodeError, Decoder, TypeId};
use sbor::{Describe, Encode, Encoder};

//==============
// BucketOf SBOR
//==============

impl<RES> TypeId for BucketOf<RES> {
    #[inline(always)]
    fn type_id() -> u8 {
        // look like a Bucket
        Bucket::type_id()
    }
}

impl<RES> Encode for BucketOf<RES> {
    #[inline(always)]
    fn encode_value(&self, encoder: &mut Encoder) {
        self.bucket.encode_value(encoder);
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: ResourceDecl> Decode for BucketOf<RES> {
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let r = Bucket::decode_value(decoder);
        r.map(|bucket| bucket.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `Bucket`'s ResourceDef (Address) with this type "RES"
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: ResourceDecl + 'static> Decode for BucketOf<RES> { // 'static is required only when doing runtime checks because of the static storage used
    #[inline(always)]
    fn decode_value(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let r = Bucket::decode_value(decoder);
        r.map(|bucket| bucket.into()) // the .into() saves duplicate code and ensures optional runtime type checks bind the decoded `Bucket`'s ResourceDef (Address) with this type "RES"
    }
}

impl<RES> Describe for BucketOf<RES> {
    #[inline(always)]
    fn describe() -> Type {
        Bucket::describe()
    }
}
