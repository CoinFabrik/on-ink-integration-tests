#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod weight_to_fee {

    #[ink(storage)]
    pub struct WeightToFee {}

    impl WeightToFee {
        /// Creates a new WeightToFee contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calculates the price of a specified amount of gas.
        #[ink(message)]
        pub fn weight_to_fee(&self, gas: u64) -> Balance {
            self.env().weight_to_fee(gas)
        }
    }

    /// Fixed amount of gas for both test environments.
    const _GAS_AMOUNT: u64 = 5000;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn weight_to_fee_works() {
            let contract = WeightToFee::new();
            let gas: u64 = _GAS_AMOUNT;

            let gas_price = contract.weight_to_fee(gas);

            // TODO:
            // assert!(gas_cost > 0);

            println!("Gas price in Integration: {:?}", gas_price);
            println!(
                "Gas price per unit in Integration: {:?}",
                gas_price / <u64 as Into<u128>>::into(gas)
            );
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn weight_to_fee_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = WeightToFeeRef::new();
            let gas: u64 = _GAS_AMOUNT;

            let contract_acc_id = client
                .instantiate("weight-to-fee", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let weight_to_fee = build_message::<WeightToFeeRef>(contract_acc_id.clone())
                .call(|contract| contract.weight_to_fee(gas));

            let gas_price = client
                .call(&ink_e2e::bob(), weight_to_fee, 0, None)
                .await
                .expect("split_profit failed")
                .return_value();

            // TODO:
            // assert!(gas_cost > 0);

            println!("Gas price in E2E: {:?}", gas_price);
            println!(
                "Gas price per unit in E2E: {:?}",
                gas_price / <u64 as Into<u128>>::into(gas)
            );

            Ok(())
        }
    }
}
