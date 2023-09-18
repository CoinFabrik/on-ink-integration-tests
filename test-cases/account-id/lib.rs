#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod account_id {

    #[ink(storage)]
    pub struct GetAccountId {}

    impl GetAccountId {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            self.env().account_id()
        }
    }

    impl Default for GetAccountId {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::{test::set_callee, DefaultEnvironment};

        use super::*;

        #[ink::test]
        fn account_id_works() {
            // Given
            let callee_account_id = AccountId::from([0x03; 32]);
            set_callee::<DefaultEnvironment>(callee_account_id);
            let contract = GetAccountId::new();

            // When
            let account_id = contract.get_account_id();

            // Then
            assert_eq!(account_id, callee_account_id);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn account_id_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GetAccountIdRef::new();
            let contract_acc_id = client
                .instantiate("account_id", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let account_id_call = build_message::<GetAccountIdRef>(contract_acc_id)
                .call(|contract| contract.get_account_id());
            let account_id = client
                .call(&ink_e2e::bob(), account_id_call, 0, None)
                .await
                .expect("account_id failed")
                .return_value();

            // Then
            assert_eq!(account_id, contract_acc_id);
            Ok(())
        }
    }
}
