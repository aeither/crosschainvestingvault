use drink::prelude::*;
use vesting_vault::{VestingVault, VestingError, AssetId};

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test]
fn test_full_vesting_cycle(mut session: Session) -> Result<(), Box<dyn std::error::Error>> {
    let admin = AccountId32::new([1u8; 32]);
    let alice = AccountId32::new([2u8; 32]);
    let bob = AccountId32::new([3u8; 32]);

    // Deploy the contract
    let contract_address = session.deploy_bundle_and(
        BundleProvider::local(),
        "new",
        &[admin.to_string()],
        NO_SALT,
        NO_ENDOWMENT,
    )?;

    // Test deposit with asset
    let deposit_result = session.call_and(
        contract_address,
        "deposit_with_asset",
        &[
            AssetId(1).encode(),
            (1000u128).encode(),
            (120u64).encode(),  // 2 minutes lock
            (2000u32).encode(), // destination parachain
        ],
        NO_ENDOWMENT,
    )?;

    println!("Deposit result: {:?}", deposit_result);

    // Test getting deposit info
    let deposit_info = session.call_and(
        contract_address,
        "get_deposit_info",
        &[alice.encode()],
        NO_ENDOWMENT,
    )?;

    println!("Deposit info: {:?}", deposit_info);

    // Test emergency unlock (should fail for non-admin)
    let emergency_result = session.call_with_address(
        contract_address,
        "emergency_unlock",
        &[],
        NO_ENDOWMENT,
        bob, // Non-admin user
    );

    assert!(emergency_result.is_err(), "Emergency unlock should fail for non-admin");

    // Test emergency unlock as admin
    let emergency_result = session.call_with_address(
        contract_address,
        "emergency_unlock",
        &[],
        NO_ENDOWMENT,
        admin,
    )?;

    println!("Emergency unlock result: {:?}", emergency_result);

    // Test claim after emergency unlock
    let claim_result = session.call_with_address(
        contract_address,
        "claim_cross_chain",
        &[],
        NO_ENDOWMENT,
        alice,
    )?;

    println!("Claim result: {:?}", claim_result);

    // Verify total locked amount decreased
    let total_locked = session.call_and(
        contract_address,
        "get_total_locked",
        &[],
        NO_ENDOWMENT,
    )?;

    println!("Total locked after claim: {:?}", total_locked);

    Ok(())
}

#[drink::test]
fn test_time_locked_claim(mut session: Session) -> Result<(), Box<dyn std::error::Error>> {
    let admin = AccountId32::new([1u8; 32]);
    let alice = AccountId32::new([2u8; 32]);

    // Deploy the contract
    let contract_address = session.deploy_bundle_and(
        BundleProvider::local(),
        "new",
        &[admin.to_string()],
        NO_SALT,
        NO_ENDOWMENT,
    )?;

    // Test deposit
    session.call_with_address(
        contract_address,
        "deposit_with_asset",
        &[
            AssetId(1).encode(),
            (1000u128).encode(),
            (120u64).encode(),  // 2 minutes lock
            (2000u32).encode(), // destination parachain
        ],
        NO_ENDOWMENT,
        alice,
    )?;

    // Try to claim immediately (should fail)
    let claim_result = session.call_with_address(
        contract_address,
        "claim_cross_chain",
        &[],
        NO_ENDOWMENT,
        alice,
    );

    assert!(claim_result.is_err(), "Claim should fail when tokens are still locked");

    // Advance time by 121 seconds
    session.advance_time(121_000);

    // Try to claim again (should succeed)
    let claim_result = session.call_with_address(
        contract_address,
        "claim_cross_chain",
        &[],
        NO_ENDOWMENT,
        alice,
    )?;

    println!("Claim after time advance: {:?}", claim_result);

    Ok(())
}

#[drink::test]
fn test_asset_support(mut session: Session) -> Result<(), Box<dyn std::error::Error>> {
    let admin = AccountId32::new([1u8; 32]);
    let alice = AccountId32::new([2u8; 32]);

    // Deploy the contract
    let contract_address = session.deploy_bundle_and(
        BundleProvider::local(),
        "new",
        &[admin.to_string()],
        NO_SALT,
        NO_ENDOWMENT,
    )?;

    // Test supported assets
    let supported_assets = session.call_and(
        contract_address,
        "get_supported_assets",
        &[],
        NO_ENDOWMENT,
    )?;

    println!("Supported assets: {:?}", supported_assets);

    // Test deposit with unsupported asset (should fail)
    let deposit_result = session.call_with_address(
        contract_address,
        "deposit_with_asset",
        &[
            AssetId(999).encode(), // Unsupported asset
            (1000u128).encode(),
            (120u64).encode(),
            (2000u32).encode(),
        ],
        NO_ENDOWMENT,
        alice,
    );

    assert!(deposit_result.is_err(), "Deposit with unsupported asset should fail");

    Ok(())
}
