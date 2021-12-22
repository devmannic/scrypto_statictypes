use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_burn_it() {
    // Set up environment.
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(include_code!("out"));

    // Test the `new` function.
    let transaction = TransactionBuilder::new(&executor)
        .call_function(package, "FixBurn", "new", vec![], None)
        .drop_all_bucket_refs()
        .deposit_all_buckets(account) // needed when returning a bucket
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.success);

    // Test the `burn_it` method.
    let component_addr = receipt.component(0).unwrap();
    let auth_addr = receipt.resource_def(1).unwrap(); // should be a better way to pick the right one...
    let flam_addr = receipt.resource_def(2).unwrap(); // should be a better way to pick the right one...
    let inflam_addr = receipt.resource_def(3).unwrap(); // should be a better way to pick the right one...

    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "burn_it",
            vec![format!("100,{}", flam_addr), format!("1,{}", auth_addr)],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.success);

    // Test the `burn_it` method < 5
    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "burn_it",
            vec![format!("3,{}", flam_addr), format!("1,{}", auth_addr)],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.success);

    // Test the `burn_it` method of the WRONG TYPE (use INFLAM)
    // it will FAIL beacuse of the type mismatch on the line: self.flam_vault.put(incoming.take(5));
    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "burn_it",
            vec![
                format!("10,{}", inflam_addr), // wrong type (inflam addr)
                format!("1,{}", auth_addr),
            ],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(!receipt.success);

    // Test the `burn_it` method < 5 of the WRONG TYPE (use INFLAM)
    // it will also fail (now that we use scrypto_statictypes with the runtime_typechecks enabled !!!)
    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "burn_it",
            vec![
                format!("3,{}", inflam_addr), // wrong type (inflam addr)
                format!("1,{}", auth_addr),
            ],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(!receipt.success);


    // Test the `alt_burn_it` method < 5 of the RIGHT TYPE (use FLAM) but bad auth
    // it will also fail even without the macro in use, because it uses BucketRefOf and runtime checks are enabled
    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "alt_burn_it",
            vec![
                format!("3,{}", flam_addr), // use the correct type this time
                format!("1,{}", flam_addr), // but use wrong resource address
            ],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(!receipt.success);

    // Again but bad auth with empty BucketRef but correct address Test the `alt_burn_it` method < 5 of the RIGHT TYPE (use FLAM) but bad auth
    // it will also fail even without the macro in use, because it uses BucketRefOf and runtime checks are enabled
    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "alt_burn_it",
            vec![
                format!("3,{}", flam_addr), // use the correct type this time
                format!("0,{}", auth_addr), // but wrong amount but right address
            ],
            Some(account),
        )
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction, true).unwrap();
    println!("{:?}\n", receipt);
    assert!(!receipt.success);
}
