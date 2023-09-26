#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod transferred_value {

    #[ink(storage)]
    pub struct TransferredValue {}

    impl TransferredValue {
        /// Creates a new Template contract.
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable)]
        pub fn send_tokens_and_get_transferred_message(&self) -> Balance {
            self.env().transferred_value()
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::{
            test::{
                callee, default_accounts, get_account_balance, set_account_balance, set_caller,
            },
            DefaultEnvironment,
        };

        use super::*;
        use ink::codegen::Env;

        #[ink::test]
        fn transferred_value() {
            // Given
            let contract = create_contract();
            let accounts = default_accounts::<DefaultEnvironment>();
            let contract_account = contract.env().account_id();

            set_account_balance::<DefaultEnvironment>(accounts.eve, 1000);
            set_account_balance::<DefaultEnvironment>(contract_account, 0);
            set_caller::<DefaultEnvironment>(accounts.eve);

            // When
            let res =
                ink::env::pay_with_call!(contract.send_tokens_and_get_transferred_message(), 1000);

            let contract_new_balance = get_balance(contract_account);
            let caller_new_balance = get_balance(accounts.eve);

            // Then
            assert_eq!(caller_new_balance, 0);
            assert_eq!(contract_new_balance, 1000);
            assert_eq!(res, 1000);
        }

        fn get_balance(account_id: AccountId) -> Balance {
            get_account_balance::<DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }

        fn create_contract() -> TransferredValue {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            set_account_balance::<DefaultEnvironment>(callee::<DefaultEnvironment>(), 100);
            TransferredValue::new()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn transferred_value_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = TransferredValueRef::new();

            // When
            let contract_acc_id = client
                .instantiate("transferred_value", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let caller_balance_before: Balance = client
                .balance(bob_account)
                .await
                .expect("getting balance failed");

            let balance_before: Balance = client
                .balance(contract_acc_id.clone())
                .await
                .expect("getting balance failed");

            let transfer = ink_e2e::build_message::<TransferredValueRef>(contract_acc_id)
                .call(|contract| contract.send_tokens_and_get_transferred_message());

            let call_res = client
                .call(&ink_e2e::bob(), transfer, 1000, None)
                .await
                .expect("transfer failed");

            let balance_after: Balance = client
                .balance(contract_acc_id)
                .await
                .expect("getting balance failed");

            let caller_balance_after: Balance = client
                .balance(bob_account)
                .await
                .expect("getting balance failed");

            // Then
            assert_eq!(call_res.return_value(), 1000);
            assert_eq!(balance_after - balance_before, 1000);
            assert_eq!(caller_balance_before - caller_balance_after, 124415155);

            Ok(())
        }
    }
}
