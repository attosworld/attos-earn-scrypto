use scrypto_test::prelude::*;

use attos_earn_scrypto::attos_earn_test::*;

#[test]
fn test_hello() {
    // Setup the environment
    let mut ledger = LedgerSimulatorBuilder::new().build();

    // Create an account
    let (public_key, _private_key, account_address) = ledger.new_allocated_account();

    // Publish package
    let package_address = ledger.compile_and_publish(this_package!());

    // Test the `instantiate_hello` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "AttosEarn",
            "instantiate",
            manifest_args!(XRD),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    let balance_before = ledger.get_component_balance(account_address, XRD);
    // Test the `free_token` method.
    let manifest = ManifestBuilder::new()
        .lock_fee(account_address, dec!(110))
        .call_method(component, "charge_royalty", manifest_args!())
        .call_method(
            account_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();

    let balance_after = ledger.get_component_balance(account_address, XRD);

    // check it tx has charged royalties correctly
    assert!(balance_before - balance_after >= 100.into());
}

#[test]
fn test_hello_with_test_environment() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let instantiation = AttosEarn::instantiate(XRD, package_address, &mut env)?;

    // Act
    instantiation.charge_royalty(&mut env)?;

    Ok(())
}
