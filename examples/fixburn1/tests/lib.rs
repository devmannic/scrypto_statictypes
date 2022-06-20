use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[cfg(feature = "runtime_typechecks")]
const RUNTIME_CHECKS: bool = true;

#[cfg(not(feature = "runtime_typechecks"))]
const RUNTIME_CHECKS: bool = false;

#[test]
fn test_burn_it() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, true);
    let (pk, sk, account) = executor.new_account();
    let package = executor.publish_package(include_package!("fixburn1")).unwrap(); // include_package instead of compile_package so we can control feature flags for testing

    // Test the `new` function.
    let transaction = TransactionBuilder::new()
        .call_function(package, "FixBurn", "new", vec![])
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
    // it will also fail (now that we use scrypto_statictypes with the runtime_typechecks enabled !!!)
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
    if RUNTIME_CHECKS {
        println!("RUNTIME_CHECKS ARE ON");
        assert!(!receipt.result.is_ok());
    } else {
        println!("RUNTIME_CHECKS ARE OFF");
        assert!(receipt.result.is_ok()); // would succeed without runtime checks
    }

/*
    // Test the `take_all_inflam` method with right amount wrong address
    // it will also fail even without the macro in use, because it uses ProofOf and runtime checks are enabled
    let transaction = TransactionBuilder::new()
        .call_method(
            component_addr,
            "take_all_inflam",
            vec![
                format!("2,{}", flam_addr), // but use wrong resource address
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    if RUNTIME_CHECKS {
        assert!(!receipt.result.is_ok());
    } else {
        assert!(!receipt.result.is_ok()); // would still fail, because cannot Decode ProofOf
    }

    // Again but bad auth with empty Proof but correct address
    // it will also fail even without the macro in use, because it uses ProofOf and runtime checks are enabled
    let transaction = TransactionBuilder::new()
        .call_method(
            component_addr,
            "take_all_inflam",
            vec![
                format!("0,{}", auth_addr), // but wrong amount but right address
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    if RUNTIME_CHECKS {
        assert!(!receipt.result.is_ok());
    } else {
        assert!(!receipt.result.is_ok()); // would still fail, because cannot Decode ProofOf
    }

    // Again but bad auth with not enough in  Proof but correct address
    // it will also fail even without the macro in use, because it uses ProofOf and runtime checks are enabled
    let transaction = TransactionBuilder::new()
        .call_method(
            component_addr,
            "take_all_inflam",
            vec![
                format!("1,{}", auth_addr), // but wrong amount but right address
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    if RUNTIME_CHECKS {
        assert!(!receipt.result.is_ok());
    } else {
        assert!(!receipt.result.is_ok()); // would still fail, because cannot Decode ProofOf
    }

    // Again but correct
    // it will also fail even without the macro in use, because it uses ProofOf and runtime checks are enabled
    let transaction = TransactionBuilder::new()
        .call_method(
            component_addr,
            "take_all_inflam",
            vec![
                format!("2,{}", auth_addr), // correct
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    println!("{:?}\n", receipt);
    if RUNTIME_CHECKS {
        assert!(receipt.result.is_ok()); // correct
    } else {
        assert!(!receipt.result.is_ok()); // would fail, because cannot Decode ProofOf
    }

*/
}
