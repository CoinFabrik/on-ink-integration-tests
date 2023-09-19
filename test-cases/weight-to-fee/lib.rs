#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod weight_to_fee {

    // Fixed gas amount
    const _GAS_AMOUNT: u64 = 5000;

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
    mod tests {
        use super::*;

        #[ink::test]
        fn weight_to_fee_works() {
            // Given
            let contract = WeightToFee::new();

            // When
            let gas_price = contract.weight_to_fee(_GAS_AMOUNT);

            // Then
            println!("Gas price in Integration: {:?}", gas_price);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn weight_to_fee_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = WeightToFeeRef::new();

            let contract_acc_id = client
                .instantiate("weight-to-fee", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let weight_to_fee_call = build_message::<WeightToFeeRef>(contract_acc_id.clone())
                .call(|contract| contract.weight_to_fee(_GAS_AMOUNT));

            let weight_to_fee_res = client
                .call(&ink_e2e::bob(), weight_to_fee_call, 0, None)
                .await
                .expect("weight-to-fee failed");

            // Then
            println!("Gas price in E2E: {:?}", weight_to_fee_res.return_value());

            Ok(())
        }
    }
}
