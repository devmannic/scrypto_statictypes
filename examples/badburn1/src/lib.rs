use scrypto::prelude::*;

blueprint! {
    struct BadBurn {
        // Define what resources and data will be managed by Hello components
        flam_vault: Vault,
        inflam_vault: Vault,
        auth_def: ResourceAddress,
        minter: Vault,
    }

    impl BadBurn {
        pub fn new() -> (ComponentAddress, Bucket, Bucket) {
            // create one owner badge for auth for the burn_it functino
            let owner = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).initial_supply(1);
            debug!("owner  raddr: {}", owner.resource_address());
            // create 1 minter badge for 2 resources, FLAM and INFLAM
            let minter = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).initial_supply(1);
            debug!("minter raddr: {}", minter.resource_address());
            // create FLAM and mint 1000
            let mut flammable_bucket = ResourceBuilder::new_fungible()
                .metadata("name", "BurnMe")
                .metadata("symbol", "FLAM")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .initial_supply(1000);
            debug!("flam   raddr: {}", flammable_bucket.resource_address());
            // create INFLAM and mint 1000
            let inflammable_bucket = ResourceBuilder::new_fungible()
                .metadata("name", "KeepMe")
                .metadata("symbol", "INFLAM")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .initial_supply(1000);
            debug!("inflam raddr: {}", inflammable_bucket.resource_address());

            // setup component storage
            let flam_vault = Vault::with_bucket(flammable_bucket.take(800)); // FLAM: 800 stay here, 200 are returned
            let c = Self {
                flam_vault: flam_vault,
                inflam_vault: Vault::with_bucket(inflammable_bucket), // all 1000 INFLAM stay here
                auth_def: owner.resource_address(), // save this so we can authorize calling burn_it()
                minter: Vault::with_bucket(minter), // keep this so we can burn)
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

        pub fn burn_it(&mut self, mut incoming: Bucket) -> Bucket {
            debug!("incoming raddr: {}", incoming.resource_address());
            // burn all but 5, give back same amount of inflam
            if incoming.amount() > dec!(5) {
                self.flam_vault.put(incoming.take(dec!(5)));
            }
            let result = self.inflam_vault.take(incoming.amount());
            self.minter.authorize(|| incoming.burn());
            result
        }
    }
}
