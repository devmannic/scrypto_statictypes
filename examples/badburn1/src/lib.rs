use scrypto::prelude::*;

blueprint! {
    struct BadBurn {
        // Define what resources and data will be managed by Hello components
        flam_vault: Vault,
        inflam_vault: Vault,
        auth_def: ResourceDef,
        minter: Vault,
    }

    impl BadBurn {
        pub fn new() -> (Component, Bucket, Bucket) {
            // create one owner badge for auth for the burn_it functino
            let minter = ResourceBuilder::new().new_badge_fixed(1);
            // create 1 minter badge for 2 resources, FLAM and INFLAM
            let owner = ResourceBuilder::new().new_badge_fixed(1);
            // create FLAM and mint 1000
            let flamable_bucket = ResourceBuilder::new()
                .metadata("name", "BurnMe")
                .metadata("symbol", "FLAM")
                .new_token_mutable(minter.resource_def())
                .mint(1000, minter.borrow());
            // create INFLAM and mint 1000
            let inflammable_bucket = ResourceBuilder::new()
                .metadata("name", "KeepMe")
                .metadata("symbol", "INFLAM")
                .new_token_mutable(minter.resource_def())
                .mint(1000, minter.borrow());

            // setup component storage
            let flam_vault = Vault::with_bucket(flamable_bucket.take(800)); // FLAM: 800 stay here, 200 are returned
            let c = Self {
                flam_vault: flam_vault,
                inflam_vault: Vault::with_bucket(inflammable_bucket), // all 1000 INFLAM stay here
                auth_def: owner.resource_def(), // save this so we can use #[auth(auth_def)]
                minter: Vault::with_bucket(minter), // keep this so we can burn)
            }
            .instantiate();
            (c, owner, flamable_bucket)
        }

        #[auth(auth_def)]
        pub fn burn_it(&mut self, incoming: Bucket) -> Bucket {
            // burn all but 5, give back same amount of inflam
            if incoming.amount() > 5.into() {
                self.flam_vault.put(incoming.take(5));
            }
            let result = self.inflam_vault.take(incoming.amount());
            self.minter.authorize(|auth| incoming.burn(auth));
            result
        }
    }
}
