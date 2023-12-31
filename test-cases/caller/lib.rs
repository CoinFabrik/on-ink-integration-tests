#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod caller {

    #[ink(storage)]
    pub struct Caller {}

    impl Caller {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn caller(&self) -> AccountId {
            self.env().caller()
        }
    }

    impl Default for Caller {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::set_caller, DefaultEnvironment};

        #[ink::test]
        fn get_caller() {
            // Given
            let e2e_bob_account_id: AccountId = ink_e2e::AccountKeyring::Bob.to_raw_public().into();
            set_caller::<DefaultEnvironment>(e2e_bob_account_id);
            let contract = Caller::new();

            // When
            let caller = contract.caller();

            // Then
            assert_eq!(caller, e2e_bob_account_id);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_caller(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = CallerRef::new();
            let contract_acc_id = client
                .instantiate("caller", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let caller =
                build_message::<CallerRef>(contract_acc_id).call(|contract| contract.caller());

            let caller_res = client
                .call(&ink_e2e::bob(), caller, 0, None)
                .await
                .expect("caller failed")
                .return_value();

            let e2e_bob_account_id: AccountId = ink_e2e::AccountKeyring::Bob.to_raw_public().into();

            // Then
            assert_eq!(caller_res, e2e_bob_account_id);
            Ok(())
        }
    }
}
