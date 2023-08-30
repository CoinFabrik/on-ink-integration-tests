#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::contract_to_call::{ ContractToCall, ContractToCallRef };

#[ink::contract]
mod contract_to_call {
    #[ink(storage)]
    pub struct ContractToCall {}

    impl ContractToCall {
        /// Creates a new Template contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calculates the profit for a given percentage of the total profit.
        #[ink(message)]
        pub fn split_profit(&self, percentage: u64, total_profit: u64) -> u64 {
            total_profit / percentage
        }
    }

    impl Default for ContractToCall {
        fn default() -> Self {
            Self::new()
        }
    }
}
