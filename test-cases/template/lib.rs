#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod template {

    #[ink(storage)]
    pub struct Template {}

    impl Template {
        /// Creates a new Template contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calculates the profit for a given percentage of the total profit.
        #[ink(message)]
        pub fn split_profit(&self, percentage: u64, total_profit: u64) -> u64 {
            (percentage / 100) * total_profit
        }
    }

    /*

    When writing tests, include comments when necessary to ensure that the code
    remains self-explanatory and easy to comprehend.

    Consider using "given/when/then" constructs where appropriate.

    Example: https://github.com/paritytech/ink/blob/master/integration-tests/contract-transfer/lib.rs

    */

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn split_profit_works() {
            // given
            let contract = Template::new();

            // when
            let profit = contract.split_profit(33, 100);

            // then
            assert_eq!(profit, 0);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn split_profit_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = TemplateRef::new();

            // when
            let contract_acc_id = client
                .instantiate("template", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // then
            let split_profit = build_message::<TemplateRef>(contract_acc_id.clone())
                .call(|contract| contract.split_profit(33, 100));
            let split_profit_res = client
                .call(&ink_e2e::bob(), split_profit, 0, None)
                .await
                .expect("split_profit failed");
            assert_eq!(split_profit_res.return_value(), 0);

            Ok(())
        }
    }
}
