#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod invoke_contract {
    use ink::env::{ call::{ Selector, build_call, ExecutionInput }, DefaultEnvironment };

    #[ink(storage)]
    pub struct InvokeContract {}

    impl InvokeContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn delegate_call(&self, contract_to_call: [u8; 32]) -> u64 {
            build_call::<DefaultEnvironment>()
                .call(AccountId::from(contract_to_call))
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("split_profit")))
                        .push_arg(10u64)
                        .push_arg(100u64)
                )
                .returns::<u64>()
                .invoke()
        }
    }

    impl Default for InvokeContract {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        #[should_panic]
        fn invoke_contract_works() {
            // Given
            let contract = InvokeContract::new();
            let other_contract_mock = [0x42; 32];

            // When
            let profit = contract.delegate_call(other_contract_mock);

            // Then
            assert_eq!(profit, 10u64);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;
        use contract_to_call::ContractToCallRef;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        #[should_panic]
        async fn invoke_contract_account_doesnt_exist(mut client: ink_e2e::Client<C, E>) {
            let original_contract_contructor = InvokeContractRef::new();
            let original_contract_acc_id = client
                .instantiate(
                    "invoke_contract",
                    &ink_e2e::alice(),
                    original_contract_contructor,
                    0,
                    None
                ).await
                .expect("instantiate failed").account_id;

            // Then
            let delegate_call = build_message::<InvokeContractRef>(
                original_contract_acc_id.clone()
            ).call(|contract| contract.delegate_call([0x42; 32]));

            client
                .call(&ink_e2e::alice(), delegate_call, 0, None).await
                .expect("Account id doesn't exist");
        }

        #[ink_e2e::test(additional_contracts = "./contract_to_call/Cargo.toml")]
        async fn invoke_contract_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let original_contract_contructor = InvokeContractRef::new();
            let contract_to_call_constructor = ContractToCallRef::new();

            // When

            let original_contract_acc_id = client
                .instantiate(
                    "invoke_contract",
                    &ink_e2e::alice(),
                    original_contract_contructor,
                    0,
                    None
                ).await
                .expect("instantiate failed").account_id;

            let contract_to_call_acc_id = client
                .instantiate(
                    "contract_to_call",
                    &ink_e2e::alice(),
                    contract_to_call_constructor,
                    0,
                    None
                ).await
                .expect("instantiate failed").account_id;

            // Then
            let delegate_call = build_message::<InvokeContractRef>(
                original_contract_acc_id.clone()
            ).call(|contract| contract.delegate_call(*contract_to_call_acc_id.as_ref()));
            let profit_res = client.call(&ink_e2e::alice(), delegate_call, 0, None).await;

            assert_eq!(profit_res.expect("This call always returns a value").return_value(), 10u64);
            Ok(())
        }
    }
}
