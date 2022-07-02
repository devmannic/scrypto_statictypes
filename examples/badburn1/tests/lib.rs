use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_burn_it() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, true);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(compile_package!()).unwrap();

    // Test the `new` function.
    let transaction = TransactionBuilder::new()
        .call_function(package, "BadBurn", "new", vec![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());

    // Test the `burn_it` method.
    let component_addr = receipt.new_component_addresses[0];
    let auth_addr = receipt.new_resource_addresses[0]; // should be a better way to pick the right one...
    let flam_addr = receipt.new_resource_addresses[2]; // should be a better way to pick the right one...
    let inflam_addr = receipt.new_resource_addresses[3]; // should be a better way to pick the right one...
    println!("auth_addr:   {}", auth_addr);
    println!("flam_addr:   {}", flam_addr);
    println!("inflam_addr: {}", inflam_addr);

    let transaction = TransactionBuilder::new()
        .create_proof_from_account(auth_addr, account)
        .withdraw_from_account_by_amount(dec!(100), flam_addr, account)
        .take_from_worktop_by_amount(dec!(100), flam_addr, |builder, bucket_id| {
            builder.call_method(
                component_addr,
                "burn_it",
                vec![
                    scrypto_encode(&scrypto::resource::Bucket(bucket_id))
                ]
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());

    // Test the `burn_it` method < 5
    let transaction = TransactionBuilder::new()
        .create_proof_from_account(auth_addr, account)
        .withdraw_from_account_by_amount(dec!(3), flam_addr, account)
        .take_from_worktop_by_amount(dec!(3), flam_addr, |builder, bucket_id| {
            builder.call_method(
                component_addr,
                "burn_it",
                vec![
                    scrypto_encode(&scrypto::resource::Bucket(bucket_id))
                ]
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());

    // Test the `burn_it` method of the WRONG TYPE (use INFLAM)
    // it will FAIL beacuse of the type mismatch on the line: self.flam_vault.put(incoming.take(5));
    let transaction = TransactionBuilder::new()
        .create_proof_from_account(auth_addr, account)
        .withdraw_from_account_by_amount(dec!(10), inflam_addr, account)
        .take_from_worktop_by_amount(dec!(10), inflam_addr, |builder, bucket_id| { // wrong type (inflam_addr)
            builder.call_method(
                component_addr,
                "burn_it",
                vec![
                    scrypto_encode(&scrypto::resource::Bucket(bucket_id))
                ]
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(!receipt.result.is_ok());

    // Test the `burn_it` method < 5 of the WRONG TYPE (use INFLAM)
    // it will succeed (oops)
    let transaction = TransactionBuilder::new()
        .create_proof_from_account(auth_addr, account)
        .withdraw_from_account_by_amount(dec!(3), inflam_addr, account)
        .take_from_worktop_by_amount(dec!(3), inflam_addr, |builder, bucket_id| { // wrong type (inflam_addr)
            builder.call_method(
                component_addr,
                "burn_it",
                vec![
                    scrypto_encode(&scrypto::resource::Bucket(bucket_id))
                ]
            )
        })
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());
}
