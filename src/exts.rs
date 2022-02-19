/// DepositOf and WithdrawOf
use crate::internal::*;
use crate::bucketof::*;
use crate::resourceof::*;
use scrypto::prelude::{
    call_method, scrypto_decode, scrypto_encode, scrypto_unwrap, Bucket, Decimal, ResourceDef,
};

#[cfg(feature = "runtime_typechecks")]
use crate::runtime::runtimechecks;

/// marker to force equivalence between 2 resources (type parameters) at compile time
pub trait ResourceIs<RES: Resource> {}
impl<RES: Resource> ResourceIs<RES> for RES {}

/// Proxy for an Account taking the place of the removed scrypto::core::Account API
struct Account {
    component: Address,
}
impl Account {
    fn address(&self) -> Address {
        self.component
    }
}

//
// Deposit
//

pub trait Deposit {
    fn deposit(&self, bucket: Bucket);
}
impl Deposit for Account {
    #[inline(always)]
    fn deposit(&self, bucket: Bucket) {
        // Account::deposit(self, bucket)
        // Account API removed in Scrypto v0.3.0, use this
        // dynamic implementation instead
        let args = vec![scrypto_encode(&bucket)];
        let rtn = call_method(self.address(), "deposit", args);
        scrypto_unwrap(scrypto_decode(&rtn))
    }
}

pub trait DepositOf
    where Self: Deposit
{
    #[inline(always)]
    fn deposit_of<RHS: Resource>(&self, bucket: BucketOf<RHS>) // RHS allows for specifying the resource with the function, or eliding it with the correct BucketOf
    {
        <Self as Deposit>::deposit(self, bucket.unwrap())
    }
}

/// Explicitly requires deposit_of::<RES> syntax instead of of automatically allowing any BucketOf<_> parameter
pub trait DepositOfExplicit<RES: Resource>
    where Self: Deposit
{
    #[inline(always)]
    fn deposit_of<RHS: Resource>(&self, bucket: BucketOf<RES>)
    where RHS: ResourceIs<RES>
    {
        <Self as Deposit>::deposit(self, bucket.unwrap())
    }
}

//
// Withdraw
//

pub trait Withdraw {
    fn withdraw<A: Into<ResourceDef>>(&self, amount: Decimal, resource_def: A) -> Bucket;
}
impl Withdraw for Account {
    // #[inline(always)] // put this back if the bug is fixed
    fn withdraw<A: Into<ResourceDef>>(&self, amount: Decimal, resource_def: A) -> Bucket {
        // Account::withdraw(self, amount, resource_def) // BUG in Scrypto implementation missing return Bucket?  Reimplement here for now -- https://github.com/radixdlt/radixdlt-scrypto/issues/107
        let args = vec![
            scrypto_encode(&amount),
            scrypto_encode(&resource_def.into()),
        ];
        let rtn = call_method(self.address(), "withdraw", args);
        scrypto_unwrap(scrypto_decode(&rtn))
    }
}

pub trait WithdrawOf
    where Self: Withdraw
{
    #[cfg(feature = "runtime_typechecks")]
    #[inline(always)]
    // RHS allows for specifying the resource with the function, or eliding it with the correct ResourceOf
    fn withdraw_of<RHS: runtimechecks::Resource>(&self, amount: Decimal, resource_of: ResourceOf<RHS>) -> BucketOf<RHS>
    {
        <Self as Withdraw>::withdraw(self, amount, resource_of).into() // do checked into here since external method call could return any type of bucket
    }

    #[cfg(not(feature = "runtime_typechecks"))]
    #[inline(always)]
    // RHS allows for specifying the resource with the function, or eliding it with the correct ResourceOf
    fn withdraw_of<RHS: ResourceDecl>(&self, amount: Decimal, resource_of: ResourceOf<RHS>) -> BucketOf<RHS>
    {
        <Self as Withdraw>::withdraw(self, amount, resource_of).into() // do checked into here since external method call could return any type of bucket
    }
}

#[cfg(not(feature = "runtime_typechecks"))]
/// Explicitly requires withdraw_of::<RES> syntax instead of of automatically allowing any ResourceOf<_> parameter
pub trait WithdrawOfExplicit<RES: ResourceDecl>
    where Self: Withdraw
{
    #[inline(always)]
    fn withdraw_of<RHS: ResourceDecl>(&self, amount: Decimal, resource_of: ResourceOf<RES>) -> BucketOf<RES>
    where RHS: ResourceIs<RES>
    {
        <Self as Withdraw>::withdraw(self, amount, resource_of).into() // do checked into here since external method call could return any type of bucket
    }
}

#[cfg(feature = "runtime_typechecks")]
/// Explicitly requires withdraw_of::<RES> syntax instead of of automatically allowing any ResourceOf<_> parameter
pub trait WithdrawOfExplicit<RES: runtimechecks::Resource>
    where Self: Withdraw
{
    #[inline(always)]
    fn withdraw_of<RHS: runtimechecks::Resource>(&self, amount: Decimal, resource_of: ResourceOf<RES>) -> BucketOf<RES>
    where RHS: ResourceIs<RES>
    {
        <Self as Withdraw>::withdraw(self, amount, resource_of).into() // do checked into here since external method call could return any type of bucket
    }
}

// Apply to Account

// impl DepositOf for Account {} // prefer Explicit, TODO make this configurable with feature flag?
impl<RES: Resource> DepositOfExplicit<RES> for Account {}
//impl WithdrawOf for Account {} // prefer Explicit, TODO make this configurable with feature flag?
#[cfg(not(feature = "runtime_typechecks"))]
impl<RES: ResourceDecl> WithdrawOfExplicit<RES> for Account {}
#[cfg(feature = "runtime_typechecks")]
impl<RES: runtimechecks::Resource> WithdrawOfExplicit<RES> for Account {}