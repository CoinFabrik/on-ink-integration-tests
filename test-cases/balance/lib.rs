#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod get_balance {

    #[ink(storage)]
    pub struct GetBalance {}

    impl GetBalance {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }
    }

    impl Default for GetBalance {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::set_account_balance, test::set_callee, DefaultEnvironment};
        #[ink::test]
        fn get_balance() {
            // Given
            let initial_balance = 100;
            let callee_id = AccountId::from([0x01; 32]);
            set_callee::<DefaultEnvironment>(callee_id);
            set_account_balance::<DefaultEnvironment>(callee_id, initial_balance);
            let contract = GetBalance::new();
            // When
            let balance = contract.get_balance();
            // Then
            assert_eq!(balance, initial_balance);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_balance(mut client: Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GetBalanceRef::new();
            let initial_balance = 50;
            let contract_to_call_acc_id = client
                .instantiate(
                    "get_balance",
                    &ink_e2e::alice(),
                    constructor,
                    initial_balance,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let contract_balance_before = client
                .balance(contract_to_call_acc_id)
                .await
                .expect("Failed to get account balance");

            // When
            let balance_to_pay = 10;
            let call = build_message::<GetBalanceRef>(contract_to_call_acc_id)
                .call(|contract| contract.get_balance());
            let call_res = client
                .call(&ink_e2e::alice(), call, balance_to_pay, None)
                .await
                .expect("call failed");

            // Then
            assert_eq!(
                call_res.return_value(),
                contract_balance_before + balance_to_pay
            );
            Ok(())
        }
    }
}
