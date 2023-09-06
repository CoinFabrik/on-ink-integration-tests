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

        /// Returns the hash code of the contract through the function 'own_code_hash'.
        #[ink(message)]
        pub fn own_code_hash(&self) -> Hash {
            self.env().own_code_hash().unwrap()
        }
    }

    impl Default for OtherContract {
        fn default() -> Self {
            Self::new()
        }
    }
}
