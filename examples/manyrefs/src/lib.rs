use scrypto::prelude::*;
use scrypto_statictypes::prelude::*;

declare_resource!(T);
declare_resource!(Q);

blueprint! {
    struct ManyRefs {
        vault: VaultOf<T>
    }

    impl ManyRefs {

        pub fn new() -> (ComponentAddress, BucketOf<T>, BucketOf<Q>) {
            let mut my_bucket: BucketOf<T> = ResourceBuilder::new_fungible()
                .metadata("name", "Token")
                .metadata("symbol", "T")
                .initial_supply(1000)
                .into();

            let other_bucket: BucketOf<Q> = ResourceBuilder::new_fungible()
                .metadata("name", "Qoken")
                .metadata("symbol", "Q")
                .initial_supply(1000)
                .into();

            debug!("T: {}", my_bucket.resource_address());
            debug!("Q: {}", other_bucket.resource_address());

            let c = Self {
                vault: VaultOf::with_bucket(my_bucket.take(900))
            }
            .instantiate()
            .globalize();
            (c, my_bucket, other_bucket)
        }

        pub fn free_token(&mut self) -> BucketOf<T> {
            self.vault.take(1)
        }
        pub fn check_amount(&self) {
            assert_eq!(self.vault.amount(), 900.into());
        }

        // good type checking on auth and on result matching vault
        pub fn double_tokens(&mut self, auth: ProofOf<T>) -> BucketOf<T> {
            self.vault.take(auth.amount() * 2)
        }
        // only checking on auth, could fail ok checking T
        pub fn double_tokens_unwrapped(&mut self, auth: ProofOf<T>) -> Bucket {
            self.vault.take(auth.amount() * 2).unwrap()
        }
        // only checking on auth, since Q isn't in self, any passed in resource will be created as Q.  Any resource could be used EXCEPT another declared one that is in self (such as T) *DANGEROUS*
        pub fn double_tokens_q_unwrapped(&mut self, auth: ProofOf<Q>) -> Bucket {
            self.vault.take(auth.amount() * 2).unwrap()
        }

        // tests using Proofs and ProofOfs

        pub fn mirror_old(&self, auth: Proof) -> Proof {
            auth
        }
        pub fn mirror_new(&self, auth: ProofOf<T>) -> ProofOf<T> {
            auth
        }
        pub fn check_amount_old(&self, auth: Proof, a: Decimal) -> bool {
            let br = self.mirror_old(auth);
            let r = br.amount() >= a;
            br.drop(); // required otherwise Proof is left dangling (actually shouldn't be needed anymore with v0.3.0)
            r
        }
        // ProofOf has Drop so this concise implementation is possible
        pub fn check_amount_new(&self, auth: ProofOf<T>, a: Decimal) -> bool {
            self.mirror_new(auth).amount() >= a
        }
        // must use check_amount_old style, or implement explicit drop when calling mirror_old in the same way
        pub fn check_vault_amount_old(&mut self, a: Decimal) -> bool {
            let bucketof: BucketOf<T> = self.vault.take(a);
            let bucket: &Bucket = &bucketof;
            let auth = bucket.create_proof();
            //let r = self.mirror_old(auth).amount() >= a; // can't use this would need the same br.drop() as in check_amount_old
            let r = self.check_amount_old(auth, a);
            self.vault.put(bucketof);
            r
        }
        // can call either check_amount_new or mirror_new since drop manages the Proofs better
        // this example uses mirror
        pub fn check_vault_amount_new_mirror(&mut self, a: Decimal) -> bool {
            let bucket: BucketOf<T> = self.vault.take(a);
            //let bucket: &Bucket = &bucket;
            let auth = bucket.create_proof();
            let r = self.mirror_new(auth).amount() >= a;
            //let r = self.check_amount_new(auth, a);
            self.vault.put(bucket);
            r
        }
        // can call either check_amount_new or mirror_new since drop manages the Proofs better
        // this example uses check_ammount
        pub fn check_vault_amount_new_check(&mut self, a: Decimal) -> bool {
            let bucket: BucketOf<T> = self.vault.take(a);
            //let bucket: &Bucket = &bucket;
            let auth = bucket.create_proof();
            //let r = self.mirror_new(auth).amount() >= a;
            let r = self.check_amount_new(auth, a);
            self.vault.put(bucket);
            r
        }

        // this fails with BucketNotFound, there's no way (as of v0.2.0) to return a Proof from a vault and keep the asset in the Vault
        pub fn bad_proof_old(&mut self, a: Decimal) -> Proof {
            let bucket: Bucket = self.vault.take(a).unwrap();
            let bref = bucket.create_proof();
            let old_vault: &mut Vault = &mut self.vault;
            old_vault.put(bucket);
            bref
        }
        // this fails with BucketNotFound, there's no way (as of v0.2.0) to return a Proof from a vault and keep the asset in the Vault
        pub fn bad_proof_new(&mut self, a: Decimal) -> ProofOf<T> {
            let bucket: BucketOf<T> = self.vault.take(a);
            let bref = bucket.create_proof();
            self.vault.put(bucket);
            bref
        }

        // this fails with dangling Bucket (atfer the bug fix found in  #102 fixed in a890ba7, after v0.2.0)
        pub fn also_bad_proof_old(&mut self, a: Decimal) -> Proof {
            let bucket: Bucket = self.vault.take(a).unwrap();
            let bref = bucket.create_proof();
            //let old_vault: &Vault = &self.vault;
            //old_vault.put(bucket);
            bref
        }
        // this fails with dangling Bucket (atfer the bug fix found in  #102 fixed in a890ba7, after v0.2.0)
        pub fn also_bad_proof_new(&mut self, a: Decimal) -> ProofOf<T> {
            let bucket: BucketOf<T> = self.vault.take(a);
            let bref = bucket.create_proof();
            //self.vault.put(bucket);
            bref
        }
    }
}
