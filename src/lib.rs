use ink::storage::Mapping;
use ink::prelude::*;

#[ink(storage)]
pub struct VestingVault {
    pub deposits: Mapping<AccountId, DepositInfo>,
    pub emergency_mode: bool,
    pub admin: AccountId,
}

#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct DepositInfo {
    pub amount: Balance,
    pub unlock_timestamp: Timestamp,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Deposited {
    pub user: AccountId,
    pub amount: Balance,
    pub unlock_time: Timestamp,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ClaimInitiated {
    pub to_chain: String,
    pub amount: Balance,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct EmergencyTriggered {
    pub timestamp: Timestamp,
}

impl VestingVault {
    #[ink(message, payable)]
    pub fn deposit(&mut self, lock_secs: u64) {
        let caller = self.env().caller();
        let current_time = self.env().block_timestamp();
        let unlock_time = current_time + lock_secs;

        assert!(lock_secs >= 60000, "Minimum lock time is 60s");

        let deposited = self.env().transferred_balance();
        assert!(deposited > 0, "You must deposit some balance");

        let info = DepositInfo {
            amount: deposited,
            unlock_timestamp: unlock_time,
        };
        self.deposits.insert(caller, &info);
        
        self.env().emit_event(Deposited {
            user: caller,
            amount: deposited,
            unlock_time,
        });
    }

    #[ink(message)]
    pub fn claim(&mut self) {
        let caller = self.env().caller();
        let current_time = self.env().block_timestamp();

        let Some(mut info) = self.deposits.get(caller).clone() else {
            panic!("No deposit found");
        };

        assert!(current_time >= info.unlock_timestamp || self.emergency_mode, "Tokens are still locked");

        let amount = info.amount;
        self.env().emit_event(ClaimInitiated {
            to_chain: "RemoteParachainXYZ".into(),
            amount,
        });

        self.deposits.remove(caller);
    }

    #[ink(message)]
    pub fn emergency_unlock(&mut self) {
        assert_eq!(self.env().caller(), self.admin, "Only admin");
        self.emergency_mode = true;

        self.env().emit_event(EmergencyTriggered {
            timestamp: self.env().block_timestamp(),
        });
    }
}
