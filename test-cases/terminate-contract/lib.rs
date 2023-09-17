#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod terminate_contract {

    #[ink(storage)]
    pub struct TerminateContract {}

    impl TerminateContract {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn terminate(&self) {
            self.env().terminate_contract(self.env().caller());
        }
    }

    impl Default for TerminateContract {
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
        fn terminate_works() {
            // Given
            let caller_account_id = AccountId::from([0x01; 32]);
            let callee_account_id = AccountId::from([0x02; 32]);
            set_caller::<DefaultEnvironment>(caller_account_id);
            set_callee::<DefaultEnvironment>(callee_account_id);
            let contract = TerminateContract::new();
            set_account_balance::<DefaultEnvironment>(callee_account_id, 100);

            // When
            contract.terminate();

            // Then
            let caller_balance = get_account_balance::<DefaultEnvironment>(caller_account_id)
                .unwrap_or_else(|err| {
                    panic!("Cannot get caller balance: {:?}", err);
                });
            assert_eq!(caller_balance, 100);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn terminate_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = TerminateContractRef::new();
            let initial_caller_balance = client
                .balance(ink_e2e::AccountKeyring::Bob.to_raw_public().into())
                .await
                .expect("balance failed");

            let contract_acc_id = client
                .instantiate("terminate_contract", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            let initial_callee_balance = client
                .balance(contract_acc_id)
                .await
                .expect("balance failed");

            // When
            let terminate_call = build_message::<TerminateContractRef>(contract_acc_id)
                .call(|contract| contract.terminate());

            let terminate_call_res = client
                .call(&ink_e2e::bob(), terminate_call, 0, None)
                .await
                .expect("split_profit failed");

            // Then
            let final_caller_balance = client
                .balance(ink_e2e::AccountKeyring::Bob.to_raw_public().into())
                .await
                .expect("balance failed");

            let final_callee_balance = client
                .balance(contract_acc_id)
                .await
                .expect("balance failed");

            println!(
                "caller balance: {} -> {}",
                initial_callee_balance, final_caller_balance
            );
            Ok(())
        }
    }
}
