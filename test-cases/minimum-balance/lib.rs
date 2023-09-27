#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod minimum_balance {

    #[ink(storage)]
    pub struct MinimumBalance {}

    impl MinimumBalance {
        /// Creates a new Template contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_minimum_balance(&self) -> u128 {
            self.env().minimum_balance()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_minimum_balance_works() {
            // given
            let contract = MinimumBalance::new();

            // when
            let profit = contract.get_minimum_balance();

            // then
            assert_eq!(profit, 1000000);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_minimum_balance_works_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = MinimumBalanceRef::new();

            // when
            let contract_acc_id = client
                .instantiate("minimum_balance", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // then
            let split_profit = build_message::<MinimumBalanceRef>(contract_acc_id.clone())
                .call(|contract| contract.get_minimum_balance());

            let split_profit_res = client
                .call(&ink_e2e::bob(), split_profit, 0, None)
                .await
                .expect("split_profit failed");

            assert_eq!(split_profit_res.return_value(), 1000000000);

            Ok(())
        }
    }
}
