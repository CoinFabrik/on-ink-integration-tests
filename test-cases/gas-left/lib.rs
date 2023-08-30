#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gas_left {

    #[ink(storage)]
    pub struct GasLeft {}

    impl GasLeft {
        /// Creates a new Template contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calculates the profit for a given percentage of the total profit.
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
            let gas_left = GasLeft::new();
            assert_eq!(gas_left.get_gas_left(), 1000000000000000000);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_gas_left_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GasLeftRef::new();

            // When
            let contract_acc_id = client
                .instantiate("gas-left", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get_gas_left = build_message::<GasLeftRef>(contract_acc_id.clone())
                .call(|contract| contract.get_gas_left());
            let _get_gas_left_res = client
                .call(&ink_e2e::bob(), get_gas_left, 0, None)
                .await
                .expect("get_gas_left failed");

            Ok(())
        }
    }
}
