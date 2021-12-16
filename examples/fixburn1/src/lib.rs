use scrypto::prelude::*;
use scrypto_statictypes::prelude::*;

declare_resource!(FLAM);
declare_resource!(INFLAM);

blueprint! {
    struct FixBurn {
        // Define what resources and data will be managed by Hello components
        flam_vault: VaultOf<FLAM>,
        inflam_vault: VaultOf<INFLAM>,
        auth_def: ResourceDef,
        minter: Vault,
    }

    impl FixBurn {
        pub fn new() -> (Component, Bucket, BucketOf<FLAM>) {
            // create one owner badge for auth for the burn_it functino
            let minter = ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1);
            // create 1 minter badge for 2 resources, FLAM and INFLAM
            let owner = ResourceBuilder::new_fungible(DIVISIBILITY_NONE).initial_supply_fungible(1);
            // create FLAM and mint 1000
            let flamable_bucket: BucketOf<FLAM> = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
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
            let flam_vault = VaultOf::with_bucket(flamable_bucket.take(800)); // FLAM: 800 stay here, 200 are returned
            let c = Self {
                flam_vault: flam_vault,
                inflam_vault: VaultOf::with_bucket(inflammable_bucket), // all 1000 INFLAM stay here
                auth_def: owner.resource_def(), // save this so we can use #[auth(auth_def)]
                minter: Vault::with_bucket(minter), // keep this so we can burn)
            }
            .instantiate();
            (c, owner, flamable_bucket)
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
    }
}
