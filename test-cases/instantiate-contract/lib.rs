#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod instantiate_contract {
    use other_contract::OtherContractRef;

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
        pub fn instantiate_other_contract(&self, code_hash: Hash) -> OtherContractRef {
            // let my_contract = ink::env::call::build_create::<OtherContractRef>()
            //     .code_hash(Hash::from([0x42; 32]))
            //     .gas_limit(4000)
            //     .endowment(total_balance / 4)
            //     .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
            //         ink::selector_bytes!("new"),
            //     )))
            //     .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
            //     .returns::<OtherContractRef>()
            //     .params();

            let my_contract = OtherContractRef::new()
                .code_hash(code_hash)
                .gas_limit(4000)
                .endowment(0)
                .salt_bytes([0x0; 4])
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
            let contract = InstantiateContract::new();

            contract.instantiate_other_contract([0x42; 32].into());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test(additional_contracts = "other_contract/Cargo.toml")]
        async fn split_profit_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = InstantiateContractRef::new();

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

            let other_contract_code_hash = client
                .upload("other_contract", &ink_e2e::bob(), None)
                .await
                .expect("instantiate failed")
                .code_hash;

            let instantiate_other_contract =
                build_message::<InstantiateContractRef>(contract_acc_id.clone())
                    .call(|contract| contract.instantiate_other_contract(other_contract_code_hash));

            let _instantiate_other_contract_res = client
                .call_dry_run(&ink_e2e::bob(), &instantiate_other_contract, 0, None)
                .await
                .return_value();

            Ok(())
        }
    }
}
