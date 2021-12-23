//! This is the same as lib.rs except it explicitly does not include BucketRefOf in a Decode location so it can compile without runtime typechecks
//! 
//! But first, here's a short doctest showing how BucketRefOf wont compile when used as an argument
//! ```compile_fail
//! # #[macro_use] extern crate scrypto_statictypes;
//! # fn main() {}
//! use scrypto::prelude::*;
//! use scrypto_statictypes::prelude::*;
//!
//! declare_resource!(AUTH);
//!
//! blueprint! {
//!
//!    struct StaticComponent {
//!         auth_def: ResourceOf<AUTH>,
//!     }
//!
//!     impl StaticComponent {
//!         pub fn do_with_auth(&mut self, _auth: BucketRefOf<AUTH>) { /* ... */ }
//!     }
//! }
//! ```
use scrypto::prelude::*;
use scrypto_statictypes::prelude::*;

declare_resource!(FLAM);
declare_resource!(INFLAM);
declare_resource!(AUTH);
declare_resource!(MINTER);

blueprint! {
    struct FixBurn {
        // Define what resources and data will be managed by Hello components
        flam_vault: VaultOf<FLAM>,
        inflam_vault: VaultOf<INFLAM>,
        auth_def: ResourceOf<AUTH>,
        minter: VaultOf<MINTER>,
    }

    impl FixBurn {
        pub fn new() -> (Component, Bucket, BucketOf<FLAM>) {
            // create one owner badge for auth for the burn_it functino
            let minter = ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1);
            // create 1 minter badge type for 2 resources, FLAM and INFLAM, but quanity 2 to test the take_all_inflam method
            let owner = ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(2);
            // create FLAM and mint 1000
            let flammable_bucket: BucketOf<FLAM> = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "BurnMe")
                .metadata("symbol", "FLAM")
                .flags(MINTABLE | BURNABLE)
                .badge(minter.resource_address(), MAY_MINT | MAY_BURN)
                .initial_supply_fungible(1000)
                .into();

            // create INFLAM and mint 1000
            let inflammable_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "KeepMe")
                .metadata("symbol", "INFLAM")
                .flags(MINTABLE | BURNABLE) // this specific accidental burn bug could ALSO be fixed my removing the BURNABLE flag, but that doesn't fix this entire class of bug
                .badge(minter.resource_address(), MAY_MINT | MAY_BURN)
                .initial_supply_fungible(1000)
                .into();

            // setup component storage
            let flam_vault = VaultOf::with_bucket(flammable_bucket.take(800)); // FLAM: 800 stay here, 200 are returned
            let c = Self {
                flam_vault: flam_vault,
                inflam_vault: VaultOf::with_bucket(inflammable_bucket), // all 1000 INFLAM stay here
                auth_def: owner.resource_def().into(), // save this so we can use #[auth(auth_def)] or BucketRefOf<AUTH> as an argument
                minter: Vault::with_bucket(minter).into(), // keep this so we can burn)
            }
            .instantiate();
            (c, owner, flammable_bucket)
        }

        #[auth(auth_def)]
        pub fn burn_it(&mut self, incoming: BucketOf<FLAM>) -> BucketOf<INFLAM> {
            // burn all but 5, give back same amount of inflam
            if incoming.amount() > 5.into() {
                self.flam_vault.put(incoming.take(5));
            }
            let result = self.inflam_vault.take(incoming.amount());
            self.minter.authorize(|auth| incoming.burn_with_auth(auth));
            result
        }

        // alternately, if runtime checks are not enabled, it will fail to compile BucketRefOf<AUTH> as an argument since it purposefully does not allow Decode
        // and if used with .into() (as shown below) it will always panic since it cannot be be checked
        #[auth(auth_def, keep_auth)]
        pub fn take_all_inflam(&mut self, /*auth: BucketRefOf<AUTH>*/) -> BucketOf<INFLAM> {
            let auth: BucketRefOf<AUTH> = auth.into(); // compiles, but panics
            assert_eq!(auth.amount(), 2.into()); // need 2 badges to take everything
            self.inflam_vault.take_all()
        }
    }
}