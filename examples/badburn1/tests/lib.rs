use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_burn_it() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, true);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(include_code!("out")).unwrap();

    // Test the `new` function.
    let transaction = TransactionBuilder::new(&executor)
        .call_function(package, "BadBurn", "new", vec![], None)
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());

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
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());

    // Test the `burn_it` method < 5
    let transaction = TransactionBuilder::new(&executor)
        .call_method(
            component_addr,
            "burn_it",
            vec![format!("3,{}", flam_addr), format!("1,{}", auth_addr)],
            Some(account),
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());

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
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(!receipt.result.is_ok());

    // Test the `burn_it` method < 5 of the WRONG TYPE (use INFLAM)
    // it will succeed (oops)
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
        .call_method_with_all_resources(account, "deposit_batch")
        .build(vec![key])
        .unwrap();
    let receipt = executor.run(transaction).unwrap();
    println!("{:?}\n", receipt);
    assert!(receipt.result.is_ok());
}