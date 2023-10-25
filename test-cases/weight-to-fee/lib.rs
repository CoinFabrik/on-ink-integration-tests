#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod weight_to_fee {
    #[ink(storage)]
    pub struct WeightToFee {}

    impl WeightToFee {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn weight_to_fee(&self, gas: u64) -> Balance {
            self.env().weight_to_fee(gas)
        }
    }

    impl Default for WeightToFee {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    const GAS_AMOUNT: u64 = 42;

    #[cfg(test)]
    const EXPECTED: u128 = 100; // as seen in integration tests

    #[cfg(test)]
    mod tests {

        use super::*;

        #[ink::test]
        fn integration_weight_to_fee() {
            // Given
            let contract = WeightToFee::new();

            // When
            let fee = contract.weight_to_fee(GAS_AMOUNT);

            // Then
            assert_eq!(fee.clone(), EXPECTED * GAS_AMOUNT as u128);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn end_to_end_weigth_to_fee(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = WeightToFeeRef::new();

            let contract_acc_id = client
                .instantiate("weight-to-fee", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let fee = client
                .call(
                    &ink_e2e::bob(),
                    build_message::<WeightToFeeRef>(contract_acc_id)
                        .call(|contract| contract.weight_to_fee(GAS_AMOUNT)), // calling weight_to_fee with GAS_AMOUNT
                    0,
                    None,
                )
                .await
                .expect("weight-to-fee failed")
                .return_value();

            // Then
            assert_eq!(fee, EXPECTED * GAS_AMOUNT as u128);

            Ok(())
        }
    }
}
