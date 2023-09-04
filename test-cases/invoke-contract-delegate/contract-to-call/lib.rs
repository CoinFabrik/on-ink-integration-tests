#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::contract_to_call::ContractToCallRef;

#[ink::contract]
mod contract_to_call {
    #[ink(storage)]
    pub struct ContractToCall {
        value: u128,
    }

    impl ContractToCall {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 25 }
        }

        #[ink(message)]
        pub fn get_value(&self) -> u128 {
            self.value
        }
    }
}
