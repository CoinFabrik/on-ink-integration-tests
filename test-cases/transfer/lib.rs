#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod transfer {

    #[ink(storage)]
    pub struct Transfer {}

    impl Transfer {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn transfer(&self, value: Balance) {
            self.env()
                .transfer(self.env().caller(), value)
                .unwrap_or_else(|err| panic!("transfer failed: {:?}", err));
        }
    }

    impl Default for Transfer {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{
            test::{get_account_balance, set_account_balance, set_callee, set_caller},
            DefaultEnvironment,
        };

        #[ink::test]
        fn transfer_works() {
            // Given
            let contract = Transfer::new();
            let callee_account_id = AccountId::from([0x01; 32]);
            let caller_account_id = AccountId::from([0x02; 32]);
            set_callee::<DefaultEnvironment>(callee_account_id);
            set_caller::<DefaultEnvironment>(caller_account_id);
            set_account_balance::<DefaultEnvironment>(callee_account_id, 1000);
            set_account_balance::<DefaultEnvironment>(caller_account_id, 0);

            // When
            contract.transfer(100);

            // Then
            let callee_balance =
                get_account_balance::<DefaultEnvironment>(AccountId::from([0x34; 32]))
                    .expect("Cannot get account balance");
            let caller_balance = get_account_balance::<DefaultEnvironment>(caller_account_id)
                .expect("Cannot get account balance");
            assert_eq!(callee_balance, 900);
            assert_eq!(caller_balance, 100);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::{build_message, AccountKeyring};

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn transfer_works(mut client: Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = TransferRef::new();

            let callee_account_id = client
                .instantiate("transfer", &ink_e2e::bob(), constructor, 1000, None)
                .await
                .expect("instantiate failed")
                .account_id;
            let caller_account_id = AccountKeyring::Bob.to_raw_public().into();

            let initial_callee_balance = client
                .balance(callee_account_id)
                .await
                .expect("Failed to get account balance");
            let initial_caller_balance = client
                .balance(caller_account_id)
                .await
                .expect("Failed to get account balance");

            // When
            let transfer_call = build_message::<TransferRef>(callee_account_id)
                .call(|contract| contract.transfer(1));

            let _transfer_call_res = client
                .call(&ink_e2e::bob(), transfer_call, 0, None)
                .await
                .expect("transfer_call failed");

            // Then
            let callee_balance = client
                .balance(callee_account_id)
                .await
                .expect("Failed to get account balance");
            let caller_balance = client
                .balance(caller_account_id)
                .await
                .expect("Failed to get account balance");

            assert_eq!(callee_balance, initial_callee_balance - 1);
            assert!(caller_balance < initial_caller_balance + 1);

            Ok(())
        }
    }
}
