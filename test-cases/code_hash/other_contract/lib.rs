#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::other_contract::{OtherContract, OtherContractRef};

#[ink::contract]
mod other_contract {
    #[ink(storage)]
    pub struct OtherContract {}

    impl OtherContract {
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

    impl Default for OtherContract {
        fn default() -> Self {
            Self::new()
        }
    }
}
