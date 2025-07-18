#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vesting_vault {
    use ink::storage::Mapping;
    use ink::prelude::*;
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    #[ink(storage)]
    pub struct VestingVault {
        pub deposits: Mapping<AccountId, DepositInfo>,
        pub emergency_mode: bool,
        pub admin: AccountId,
        pub total_locked: Balance,
        pub supported_assets: Vec<AssetId>,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct DepositInfo {
        pub amount: Balance,
        pub unlock_timestamp: Timestamp,
        pub asset_id: AssetId,
        pub destination_parachain: u32,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct AssetId(pub u32);

    #[ink(event)]
    pub struct Deposited {
        pub user: AccountId,
        pub amount: Balance,
        pub asset_id: AssetId,
        pub unlock_time: Timestamp,
    }

    #[ink(event)]
    pub struct ClaimInitiated {
        pub user: AccountId,
        pub amount: Balance,
        pub destination_parachain: u32,
        pub xcm_hash: [u8; 32],
    }

    #[ink(event)]
    pub struct EmergencyTriggered {
        pub timestamp: Timestamp,
        pub admin: AccountId,
    }

    #[ink(event)]
    pub struct XCMExecuted {
        pub user: AccountId,
        pub amount: Balance,
        pub destination: u32,
        pub success: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum VestingError {
        InsufficientBalance,
        TokensStillLocked,
        NoDepositFound,
        UnauthorizedAccess,
        AssetNotSupported,
        XCMExecutionFailed,
    }

    impl VestingVault {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut supported_assets = Vec::new();
            supported_assets.push(AssetId(1)); // DOT
            supported_assets.push(AssetId(2)); // USDT
            
            Self {
                deposits: Default::default(),
                emergency_mode: false,
                admin,
                total_locked: 0,
                supported_assets,
            }
        }

        // Asset Precompile Integration
        #[ink(message, payable)]
        pub fn deposit_with_asset(
            &mut self, 
            asset_id: AssetId, 
            amount: Balance, 
            lock_secs: u64,
            destination_parachain: u32,
        ) -> Result<(), VestingError> {
            let caller = self.env().caller();
            let current_time = self.env().block_timestamp();
            let unlock_time = current_time + lock_secs;

            // Validate asset support
            if !self.supported_assets.contains(&asset_id) {
                return Err(VestingError::AssetNotSupported);
            }

            assert!(lock_secs >= 60000, "Minimum lock time is 60 seconds");
            assert!(amount > 0, "Amount must be greater than zero");

            // In a real implementation, this would call the Assets precompile
            // to transfer tokens from the user to the contract
            self.call_assets_precompile_transfer(caller, amount, asset_id.clone())?;

            let info = DepositInfo {
                amount,
                unlock_timestamp: unlock_time,
                asset_id: asset_id.clone(),
                destination_parachain,
            };

            self.deposits.insert(caller, &info);
            self.total_locked += amount;
            
            self.env().emit_event(Deposited {
                user: caller,
                amount,
                asset_id,
                unlock_time,
            });

            Ok(())
        }

        // XCM Cross-Chain Claim
        #[ink(message)]
        pub fn claim_cross_chain(&mut self) -> Result<(), VestingError> {
            let caller = self.env().caller();
            let current_time = self.env().block_timestamp();

            let info = self.deposits.get(caller)
                .ok_or(VestingError::NoDepositFound)?;

            if current_time < info.unlock_timestamp && !self.emergency_mode {
                return Err(VestingError::TokensStillLocked);
            }

            // Execute XCM cross-chain transfer
            let xcm_hash = self.execute_xcm_transfer(
                caller,
                info.amount,
                info.destination_parachain,
                info.asset_id.clone(),
            )?;

            self.total_locked -= info.amount;
            self.deposits.remove(caller);

            self.env().emit_event(ClaimInitiated {
                user: caller,
                amount: info.amount,
                destination_parachain: info.destination_parachain,
                xcm_hash,
            });

            Ok(())
        }

        // Circuit Breaker - Emergency Withdraw
        #[ink(message)]
        pub fn emergency_unlock(&mut self) -> Result<(), VestingError> {
            if self.env().caller() != self.admin {
                return Err(VestingError::UnauthorizedAccess);
            }

            self.emergency_mode = true;

            self.env().emit_event(EmergencyTriggered {
                timestamp: self.env().block_timestamp(),
                admin: self.admin,
            });

            Ok(())
        }

        // Assets Precompile Integration (simulated)
        fn call_assets_precompile_transfer(
            &self,
            from: AccountId,
            amount: Balance,
            asset_id: AssetId,
        ) -> Result<(), VestingError> {
            // In a real implementation, this would call the Assets precompile
            // using something like:
            // 
            // let call = build_call::<DefaultEnvironment>()
            //     .call_type(Call::new(ASSETS_PRECOMPILE_ADDRESS))
            //     .exec_input(ExecutionInput::new(Selector::new([0x84, 0xa1, 0x5d, 0xa1])))
            //     .returns::<()>()
            //     .params();
            //
            // self.env().invoke_contract(&call).unwrap();
            
            // For demonstration, we'll just simulate the transfer
            ink::env::debug_println!("Assets precompile transfer: {} tokens of asset {:?} from {:?}", amount, asset_id, from);
            
            Ok(())
        }

        // XCM Execution (using ink! v5.1.0+ XCM functions)
        fn execute_xcm_transfer(
            &self,
            beneficiary: AccountId,
            amount: Balance,
            destination_parachain: u32,
            asset_id: AssetId,
        ) -> Result<[u8; 32], VestingError> {
            // In ink! v5.1.0+, you can use xcm_execute and xcm_send
            // This is a simplified example
            
            // Create XCM message for cross-chain transfer
            let xcm_message = self.build_xcm_message(beneficiary, amount, destination_parachain, asset_id);
            
            // Execute XCM (simulated)
            let xcm_hash = self.calculate_xcm_hash(&xcm_message);
            
            self.env().emit_event(XCMExecuted {
                user: beneficiary,
                amount,
                destination: destination_parachain,
                success: true,
            });
            
            Ok(xcm_hash)
        }

        fn build_xcm_message(
            &self,
            beneficiary: AccountId,
            amount: Balance,
            destination_parachain: u32,
            asset_id: AssetId,
        ) -> Vec<u8> {
            // Build XCM message (simplified for demo)
            let mut message = Vec::new();
            message.extend_from_slice(&beneficiary.encode());
            message.extend_from_slice(&amount.to_le_bytes());
            message.extend_from_slice(&destination_parachain.to_le_bytes());
            message.extend_from_slice(&asset_id.0.to_le_bytes());
            message
        }

        fn calculate_xcm_hash(&self, message: &[u8]) -> [u8; 32] {
            // Simple hash calculation for demo
            let mut hash = [0u8; 32];
            for (i, byte) in message.iter().enumerate() {
                hash[i % 32] ^= byte;
            }
            hash
        }

        #[ink(message)]
        pub fn get_deposit_info(&self, account: AccountId) -> Option<DepositInfo> {
            self.deposits.get(account)
        }

        #[ink(message)]
        pub fn get_total_locked(&self) -> Balance {
            self.total_locked
        }

        #[ink(message)]
        pub fn is_emergency_mode(&self) -> bool {
            self.emergency_mode
        }

        #[ink(message)]
        pub fn get_supported_assets(&self) -> Vec<AssetId> {
            self.supported_assets.clone()
        }
    }
}
