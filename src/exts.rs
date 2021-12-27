use crate::internal::*;
use crate::bucketof::*;
use crate::resourceof::*;
use scrypto::prelude::{Bucket, Account, Decimal, ResourceDef, scrypto_encode, scrypto_decode, call_method, scrypto_unwrap};

//
// Deposit
//

pub trait Deposit {
    fn deposit(&self, bucket: Bucket);
}
impl Deposit for Account {
    #[inline(always)]
    fn deposit(&self, bucket: Bucket) {
        Account::deposit(self, bucket)
    }
}

pub trait DepositExt<RES: Resource>
    where Self: Deposit
{
    #[inline(always)]
    fn deposit(&self, bucket: BucketOf<RES>) {
        <Self as Deposit>::deposit(self, bucket.unwrap())
    }
}

// make deposit_of call require compile time check that RES = RHS
pub trait DepositOf<RES: Resource>
    where Self: DepositExt<RES>
{
    #[inline(always)]
    fn deposit_of<RHS: Resource>(&self, bucket: BucketOf<RHS>)
    where dyn DepositExt<RES>: DepositExt<RHS> // constrain that DepositExt<RES> allows DepositExt<RHS> (also constrains RES)
    //where Self: DepositExt<RHS> // this doesn't quite work, leads to "cannot infer type for type parameter 'RES' declared on the trait 'DepositOf'"
    {
        //<Self as DepositExt<RES>>::deposit(self, bucket.unwrap().unchecked_into())
        <Self as Deposit>::deposit(self, bucket.unwrap()) // potentially less code than the line above
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
        // Account::withdraw(self, amount, resource_def) // BUG in Scrypto implementation missing return Bucket?  Reimplement here for now
        let args = vec![
            scrypto_encode(&amount),
            scrypto_encode(&resource_def.into()),
        ];
        let rtn = call_method(self.address(), "withdraw", args);
        scrypto_unwrap(scrypto_decode(&rtn))
    }
}

// I think because withdraw has generics in both the parameters and return value, the WithdrawExt is unneccessary, and adding "RHS" may not be needed... Need to test this

pub trait WithdrawOf<RES: Resource>
    where Self: Withdraw
{
    #[inline(always)]
    fn withdraw<A: Into<ResourceOf<RES>>>(&self, amount: Decimal, resource_def: A) -> BucketOf<RES> { // change to ResourceOf creates static type check (and BucketOf<RES>)
        let resource_def: ResourceOf<RES> = resource_def.into(); // may do runtime check
        let bucket: Bucket = <Self as Withdraw>::withdraw(self, amount, resource_def.unwrap());
        bucket.unchecked_into() // avoid an extra runtime check
    }
}


// Deposit* for Account

impl<RES: Resource> DepositExt<RES> for Account {}
impl<RES: Resource> DepositOf<RES> for Account {}

// Withdraw* for Account

impl<RES: Resource> WithdrawOf<RES> for Account {}