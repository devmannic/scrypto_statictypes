use crate::internal::*;
use crate::bucketof::*;
use scrypto::prelude::{Bucket, Account};

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


// Deposit* for Account

impl<RES: Resource> DepositExt<RES> for Account {}
impl<RES: Resource> DepositOf<RES> for Account {}