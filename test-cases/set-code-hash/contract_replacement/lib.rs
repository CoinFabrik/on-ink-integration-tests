#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::contract_replacement::{ContractReplacement, ContractReplacementRef};

#[ink::contract]
mod contract_replacement {
    use ink::env::set_code_hash;

    #[ink(storage)]
    pub struct ContractReplacement {
        admin: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidCodeHash,
        NotAnAdmin,
    }

    impl ContractReplacement {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
            }
        }

        /// Sets new code hash to contract
        #[ink(message)]
        pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
            if self.admin != Self::env().caller() {
                return Err(Error::NotAnAdmin);
            }

            let res = set_code_hash(&value);

            if res.is_err() {
                return res.map_err(|_| Error::InvalidCodeHash);
            }

            Ok(())
        }

        /// Returns the codehash of the contract
        #[ink(message)]
        pub fn get_code(&self) -> Hash {
            self.env()
                .code_hash(&self.env().account_id())
                .expect("Failed to get code hash")
        }
    }

    impl Default for ContractReplacement {
        fn default() -> Self {
            Self::new()
        }
    }
}
