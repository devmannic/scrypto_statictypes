use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

macro_rules! setup {
    ($ledger:ident, $debug:ident) => {{
        // Set up environment.
        let mut executor = TransactionExecutor::new(&mut $ledger, $debug);
        let key = executor.new_public_key();
        let account = executor.new_account(key);
        let package = executor.publish_package(include_code!("manyrefs")).unwrap();
        // Test the `new` function.
        let transaction1 = TransactionBuilder::new(&executor)
            .call_function(package, "ManyRefs", "new", vec![], None)
            .call_method_with_all_resources(account, "deposit_batch")
            .build(vec![key])
            .unwrap();
        let receipt1 = executor.run(transaction1).unwrap();
        println!("{:?}\n", receipt1);
        assert!(receipt1.result.is_ok());
        (receipt1, executor, account, key)
    }};
}

#[test]
fn test_hello() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let _rdef = receipt1.resource_def(0).unwrap();

    // Test the `free_token` method.
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(component, "free_token", vec![], Some(account))
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_double_tokens() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let rdef = receipt1.resource_def(0).unwrap();
    //
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "double_tokens",
            vec![format!("10,{}", rdef)],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_sploit_double_tokens() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //let rdef = receipt1.resource_def(0).unwrap();
    let qrdef = receipt1.resource_def(1).unwrap();
    //
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "double_tokens",
            vec![
                format!("10,{}", qrdef), // Q
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // should fail tx because Q != T
}

#[test]
fn test_sploit_double_tokens_unwrapped() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let qrdef = receipt1.resource_def(1).unwrap();
    //
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "double_tokens_unwrapped",
            vec![
                format!("10,{}", qrdef), // Q which is not what is expected, should fail
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // should fail tx because Q != T
}

#[test]
fn test_sploit_double_tokens_q_unwrapped() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let rdef = receipt1.resource_def(0).unwrap();
    //    let qrdef = receipt1.resource_def(1).unwrap();
    //
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "double_tokens_q_unwrapped",
            vec![
                format!("10,{}", rdef), // T which is not what is expected, should fail
            ],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // correctly fails, because of address in use check
}

#[test]
fn test_mirror_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "mirror_old",
            vec![format!("10,{}", rdef)],
            Some(account),
        )
        //.drop_bucket_ref(Rid(514u32)) // an option that is ugly, and doesn't work
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // this fails without drop_all_bucket_refs or similar, and there's no (obvious) way to specify the correct single bucket ref to use with the available drop_bucket_ref(rid)
}

#[test]
fn test_mirror_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "mirror_new",
            vec![format!("10,{}", rdef)],
            Some(account),
        )
        //.drop_bucket_ref(Rid(514u32)) // an option that is ugly, and doesn't work
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // this fails without drop_all_bucket_refs or similar, and there's no (obvious) way to specify the correct single bucket ref to use with the available drop_bucket_ref(rid)
}

#[test]
fn test_check_amount_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "check_amount_old",
            vec![format!("10,{}", rdef), format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_amount_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "check_amount_new",
            vec![format!("10,{}", rdef), format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_vault_amount_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, true);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "check_vault_amount_old",
            vec![format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_vault_amount_new_mirror() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "check_vault_amount_new_mirror",
            vec![format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_check_vault_amount_new_check() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "check_vault_amount_new_check",
            vec![format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_bad_proof_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "bad_proof_old",
            vec![format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // expected to fail, can't return BucketRef after putting the bucket back into the Vault
}

#[test]
fn test_bad_proof_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "bad_proof_new",
            vec![format!("1")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // expected to fail, can't return BucketRef after putting the bucket back into the Vault
}

#[test]
fn test_also_bad_proof_old() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "also_bad_proof_old",
            vec![format!("25")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // will fail
}

#[test]
fn test_also_bad_proof_new() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let (receipt1, mut executor, account, key) = setup!(ledger, false);

    //    let rdef = receipt1.resource_def(0).unwrap();
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(
            component,
            "also_bad_proof_new",
            vec![format!("25")],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // will fail
}