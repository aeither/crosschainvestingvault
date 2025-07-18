use drink::prelude::*;

#[test]
fn test_deposit_and_claim() {
    drink::run_test(async {
        let alice = AccountKeyring::Alice.pair();

        let contract = deploy_contract!("vesting_vault.contract");

        call!(contract.deposit(30_000), from: alice, value: 1_000_000_000_000)?;

        advance_block_time(31_000);

        call!(contract.claim(), from: alice)?;

        Ok(())
    });
}
