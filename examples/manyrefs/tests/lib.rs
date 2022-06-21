use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

macro_rules! setup {
    ($ledger:ident, $debug:ident) => {{
        // Set up environment.
        let mut executor = TransactionExecutor::new(&mut $ledger, $debug);
        let (pk, sk, account) = executor.new_account();
        let package = executor.publish_package(compile_package!()).unwrap();
        // Test the `new` function.
        let transaction1 = TransactionBuilder::new()
            .call_function(package, "ManyRefs", "new", vec![])
            .call_method_with_all_resources(account, "deposit_batch")
            .build(executor.get_nonce([pk]))
            .sign([&sk]);
        let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
        println!("{:?}\n", receipt1);
        assert!(receipt1.result.is_ok());
        (receipt1, executor, account, pk, sk)
    }};
}

// convenience methods to call methods with bucket
trait TransactionBuilderExt {
    fn with_account_bucket_call_method(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self;
    fn with_account_bucket_call_method_pre(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self;
    fn with_account_proof_call_method(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self;
    fn with_account_proof_call_method_pre(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self;
}

impl TransactionBuilderExt for TransactionBuilder {
    fn with_account_bucket_call_method(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self {
        self
        .withdraw_from_account_by_amount(amount, resource_address, account)
        .take_from_worktop_by_amount(amount, resource_address, | builder, bucket_id| {
            let mut args = args.clone();
            args.extend([scrypto_encode(&scrypto::resource::Bucket(bucket_id))]);
            builder.call_method(
                component_addr,
                method_name,
                args
            )
        })
    }
    fn with_account_bucket_call_method_pre(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self {
        self
        .withdraw_from_account_by_amount(amount, resource_address, account)
        .take_from_worktop_by_amount(amount, resource_address, | builder, bucket_id| {
            let mut new_args = vec![scrypto_encode(&scrypto::resource::Bucket(bucket_id))];
            new_args.extend(args);
            builder.call_method(
                component_addr,
                method_name,
                new_args
            )
        })
    }
    fn with_account_proof_call_method(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self {
        self
        .create_proof_from_account_by_amount(amount, resource_address, account)
        .pop_from_auth_zone(| builder, proof_id| {
            let mut args = args.clone();
            args.extend([scrypto_encode(&scrypto::resource::Proof(proof_id))]);
            builder.call_method(
                component_addr,
                method_name,
                args
            )
        })
    }
    fn with_account_proof_call_method_pre(&mut self,
        amount: Decimal,
        resource_address: ResourceAddress,
        account: ComponentAddress,
        component_addr: ComponentAddress,
        method_name: &str,
        args: Vec<Vec<u8>>
    ) -> &mut Self {
        self
        .create_proof_from_account_by_amount(amount, resource_address, account)
        .pop_from_auth_zone(| builder, proof_id| {
            let mut new_args = vec![scrypto_encode(&scrypto::resource::Proof(proof_id))];
            new_args.extend(args);
            builder.call_method(
                component_addr,
                method_name,
                new_args
            )
        })
    }
}

#[test]
fn test_hello() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    // Test the `free_token` method.
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(component, "free_token", vec![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_double_tokens() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let rdef = receipt1.new_resource_addresses[0];
    //
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
    /*
        .call_method(
            component,
            "double_tokens",
            vec![format!("10,{}", rdef)],
        )
    */
        .with_account_proof_call_method(dec!(10), rdef, account, component, "double_tokens", vec![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_sploit_double_tokens() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let qrdef = receipt1.new_resource_addresses[1];
    //
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .with_account_proof_call_method(dec!(10), qrdef, account, component, "double_tokens", vec![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // should fail tx because Q != T
}

#[test]
fn test_sploit_double_tokens_unwrapped() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let qrdef = receipt1.new_resource_addresses[1];
    //
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .with_account_proof_call_method(dec!(10), qrdef, account, component, "double_tokens_unwrapped", vec![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // should fail tx because Q != T
}

#[test]
fn test_sploit_double_tokens_q_unwrapped() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let rdef = receipt1.new_resource_addresses[0];
    //
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        // T which is not what is expected, should fail
        .with_account_proof_call_method(dec!(10), rdef, account, component, "double_tokens_q_unwrapped", vec![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // correctly fails, because of address in use check
}

#[test]
fn test_mirror_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let rdef = receipt1.new_resource_addresses[0];
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .with_account_proof_call_method(dec!(10), rdef, account, component, "mirror_old", vec![])
        //.drop_proof(Rid(514u32)) // an option that is ugly, and doesn't work
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // this fails without drop_all_proofs or similar, and there's no (obvious) way to specify the correct single bucket ref to use with the available drop_proof(rid)
}

#[test]
fn test_mirror_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let rdef = receipt1.new_resource_addresses[0];
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .with_account_proof_call_method(dec!(10), rdef, account, component, "mirror_new", vec![])
        //.drop_proof(Rid(514u32)) // an option that is ugly, and doesn't work
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // this fails without drop_all_proofs or similar, and there's no (obvious) way to specify the correct single bucket ref to use with the available drop_proof(rid)
}

#[test]
fn test_check_amount_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let rdef = receipt1.new_resource_addresses[0];
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .with_account_proof_call_method_pre(dec!(10), rdef, account, component, "check_amount_old", args![dec!(1)])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_amount_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let rdef = receipt1.new_resource_addresses[0];
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .with_account_proof_call_method_pre(dec!(10), rdef, account, component, "check_amount_new", args![dec!(1)])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_vault_amount_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, true);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "check_vault_amount_old",
            args![dec!(1)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_vault_amount_new_mirror() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "check_vault_amount_new_mirror",
            args![dec!(1)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_vault_amount_new_check() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "check_vault_amount_new_check",
            args![dec!(1)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_bad_proof_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "bad_proof_old",
            args![dec!(1)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // expected to fail, can't return Proof after putting the bucket back into the Vault
}

#[test]
fn test_bad_proof_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "bad_proof_new",
            args![dec!(1)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // expected to fail, can't return Proof after putting the bucket back into the Vault
}

#[test]
fn test_also_bad_proof_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "also_bad_proof_old",
            args![dec!(25)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // will fail
}

#[test]
fn test_also_bad_proof_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, pk, sk) = setup!(ledger, false);

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(
            component,
            "also_bad_proof_new",
            args![dec!(25)],
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // will fail
}
