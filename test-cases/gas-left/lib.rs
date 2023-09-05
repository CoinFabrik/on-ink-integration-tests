#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gas_left {

    #[ink(storage)]
    pub struct GasLeft {}

    impl GasLeft {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_gas_left(&self) -> u64 {
            self.env().gas_left()
        }
    }

    impl Default for GasLeft {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        #[should_panic]
        fn get_gas_left() {
            let contract = GasLeft::new();
            assert!(contract.get_gas_left() > 0);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_gas_left_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = GasLeftRef::new();
            let contract_acc_id = client
                .instantiate("gas-left", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            let get_gas_left = build_message::<GasLeftRef>(contract_acc_id)
                .call(|contract| contract.get_gas_left());
            let get_gas_left_res = client
                .call(&ink_e2e::bob(), get_gas_left, 0, None)
                .await
                .expect("get_gas_left failed")
                .return_value();
            // Assert that the gas left is greater than zero
            assert!(get_gas_left_res > 0);
            Ok(())
        }
    }
}
