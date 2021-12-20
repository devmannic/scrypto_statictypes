use scrypto::prelude::*;

use crate::internal::*;
use crate::resourceof::ResourceOf;
use crate::bucketrefof::BucketRefOf;

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

pub type BucketOf<RES> = Of<Bucket, RES>;

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
        self.inner.put(other.inner)
    }

    /// Takes some amount of resources from this bucket.
    #[inline(always)]
    pub fn take<A: Into<Decimal>>(&self, amount: A) -> Self {
        self.inner.take(amount).unchecked_into()
    }

    /// Burns resource within this bucket.
    #[inline(always)]
    pub fn burn(self) {
        // must define this instead of leaning on Deref because of self not &self (needs DerefMove which doesn't exist yet)
        self.inner.burn();
    }

    /// Burns resource within this bucket.
    #[inline(always)]
    pub fn burn_with_auth<AUTH>(self, auth: BucketRefOf<AUTH>) {
        // must define this instead of leaning on Deref because of self not &self (needs DerefMove which doesn't exist yet)
        self.inner.burn_with_auth(auth.inner);
    }

    /// Returns the resource definition of resources in this bucket.
    #[inline(always)]
    pub fn resource_def(&self) -> ResourceOf<RES> {
        self.inner.resource_def().unchecked_into()
    }

    /// Creates an immutable reference to this bucket.
    #[inline(always)]
    pub fn present(&self) -> BucketRefOf<RES> {
        self.inner.present().unchecked_into()
    }

    /// Uses resources in this bucket as authorization for an operation.
    #[inline(always)]
    pub fn authorize<F: FnOnce(BucketRefOf<RES>) -> O, O>(&self, f: F) -> O {
        f(self.present())
    }

    /// Takes an NFT from this bucket, by id.
    ///
    /// # Panics
    /// Panics if this is not an NFT bucket or the specified NFT is not found.
    #[inline(always)]
    pub fn take_nft(&self, id: u128) -> BucketOf<RES> {
        self.inner.take_nft(id).unchecked_into()
    }
}

#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> From<Bucket> for BucketOf<RES> {
    fn from(bucket: Bucket) -> Self {
        if !runtimechecks::check_address::<RES>(bucket.resource_address()) {
            let tmp_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM).initial_supply_fungible(1);
            bucket.put(tmp_bucket); // this will trigger resource def mismatch error: Err(InvokeError(Trap(Trap { kind: Host(BucketError(MismatchingResourceDef)) })))
                                    // shouldn't get here, but just in case (and to help the compiler)
            panic!("BucketOf mismatch");
        }
        bucket.unchecked_into()
    }
}
