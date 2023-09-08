#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod instantiate_contract {
    use ink::env::call::ExecutionInput;
    use ink::env::DefaultEnvironment;

    #[ink(storage)]
    pub struct InstantiateContract {}

    impl InstantiateContract {
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

        #[ink(message)]
        pub fn instantiate_other_contract(&self) -> InstantiateContractRef {
            let my_contract = ink::env::call::build_create::<InstantiateContractRef>()
                .code_hash(Hash::from([0x42; 32]))
                .gas_limit(4000)
                .endowment(25)
                .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                    ink::selector_bytes!("constructor"),
                )))
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .returns::<InstantiateContractRef>()
                .params();

            self.env()
                .instantiate_contract(&my_contract)
                .unwrap_or_else(|error| {
                    panic!(
                        "Received an error from the Contracts pallet while instantiating: {:?}",
                        error
                    )
                })
                .unwrap_or_else(|error| {
                    panic!("Received a `LangError` while instatiating: {:?}", error)
                })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn split_profit_precision() {
            // Given
            let contract = InstantiateContract::new();

            // When
            let profit = contract.split_profit(33, 100);

            // Then
            assert_eq!(profit, 0);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn split_profit_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = InstantiateContractRef::new();

            // When
            let contract_acc_id = client
                .instantiate(
                    "instantiate_contract",
                    &ink_e2e::bob(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let instantiate_other_contract =
                build_message::<InstantiateContractRef>(contract_acc_id.clone())
                    .call(|contract| contract.instantiate_other_contract());
            let instantiate_other_contract_res = client
                .call(&ink_e2e::bob(), instantiate_other_contract, 0, None)
                .await
                .expect("split_profit failed");

            assert_eq!(split_profit_res.return_value(), 0);

            Ok(())
        }
    }
}
