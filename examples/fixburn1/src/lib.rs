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
        pub fn new() -> (ComponentAddress, Bucket, BucketOf<FLAM>) {
            // create one owner badge for auth for the burn_it functino
            let owner = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).initial_supply(1);

            // create 1 minter badge type for 2 resources, FLAM and INFLAM, but quanity 2 to test the take_all_inflam method
            let minter = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).initial_supply(2);

            // create FLAM and mint 1000
            let mut flammable_bucket: BucketOf<FLAM> = ResourceBuilder::new_fungible()
                .metadata("name", "BurnMe")
                .metadata("symbol", "FLAM")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .initial_supply(1000)
                .into();

            // create INFLAM and mint 1000
            let inflammable_bucket = ResourceBuilder::new_fungible()
                .metadata("name", "KeepMe")
                .metadata("symbol", "INFLAM")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED) // this specific accidental burn bug could ALSO be fixed my limiting the burnable authorities, but that doesn't fix this entire class of bug
                .initial_supply(1000)
                .into();

            // setup component storage
            let flam_vault = VaultOf::with_bucket(flammable_bucket.take(800)); // FLAM: 800 stay here, 200 are returned
            let c = Self {
                flam_vault: flam_vault,
                inflam_vault: VaultOf::with_bucket(inflammable_bucket), // all 1000 INFLAM stay here
                auth_def: owner.resource_address().into(), // save this so we can authorize calling burn_it()
                minter: Vault::with_bucket(minter).into(), // keep this so we can burn)
            }
            .instantiate()
            .add_access_check(
                AccessRules::new()
                .method("burn_it", rule!(require("auth_def")))
                .default(rule!(allow_all))
            )
            .globalize();
            (c, owner, flammable_bucket)
        }

        pub fn burn_it(&mut self, mut incoming: BucketOf<FLAM>) -> BucketOf<INFLAM> {
            // burn all but 5, give back same amount of inflam
            if incoming.amount() > dec!(5) {
                self.flam_vault.put(incoming.take(dec!(5)));
            }
            let result = self.inflam_vault.take(incoming.amount());
            self.minter.authorize(|| incoming.burn());
            result
        }

        // auth works the same way here, as long as the runtime type checks feature is enabled. auth will drop without needing the macro
        pub fn take_all_inflam(&mut self, auth: ProofOf<AUTH>) -> BucketOf<INFLAM> {
            assert_eq!(auth.amount(), dec!(2)); // need 2 badges to take everything
            self.inflam_vault.take_all()
        }

        // alternately, if runtime checks are not enabled, it will fail to compile ProofOf<AUTH> as an argument since it purposefully does not allow Decode
        // and if used with .into() (as shown below) it will always panic since it cannot be be checked
        pub fn take_all_inflam_static(&mut self, auth: Proof) -> BucketOf<INFLAM> {
            let auth: ProofOf<AUTH> = auth.into(); // compiles, but panics
            assert_eq!(auth.amount(), dec!(2)); // need 2 badges to take everything
            self.inflam_vault.take_all()
        }
    }
}