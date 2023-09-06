#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod code_hash {

    #[ink(storage)]
    pub struct CodeHash {}

    impl CodeHash {
        /// Creates a new Template contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_own_code_hash(&self) -> Hash {
            let hash = self.get_code_hash(self.env().account_id());
            ink::env::debug_println!("{:?}", hash);
            return hash;
        }

        #[ink(message)]
        pub fn get_code_hash(&self, account_id: AccountId) -> Hash {
            self.env().code_hash(&account_id).unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        #[should_panic]
        fn get_own_code_hash() {
            let contract = CodeHash::new();

            contract.get_own_code_hash();
        }

        #[ink::test]
        #[should_panic]
        fn get_other_code_hash() {
            let contract = CodeHash::new();

            contract.get_code_hash([0x42; 32].into());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;
        use other_contract::OtherContractRef;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_own_code_hash_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = CodeHashRef::new();

            let contract_acc_id = client
                .instantiate("code_hash", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let contract_hash = build_message::<CodeHashRef>(contract_acc_id.clone())
                .call(|contract| contract.get_own_code_hash());

            let contract_hash_res = client.call(&ink_e2e::bob(), contract_hash, 0, None).await;

            assert!(contract_hash_res.is_ok());

            Ok(())
        }

        #[ink_e2e::test(additional_contracts = "./other_contract/Cargo.toml")]
        async fn get_other_contract_code_hash_e2e(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = CodeHashRef::new();
            let other_contract_constructor = OtherContractRef::new();

            let contract_acc_id = client
                .instantiate("code_hash", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let other_contract_acc_id = client
                .instantiate(
                    "other_contract",
                    &ink_e2e::alice(),
                    other_contract_constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let contract_hash = build_message::<CodeHashRef>(contract_acc_id.clone())
                .call(|contract| contract.get_code_hash(other_contract_acc_id));

            let contract_hash_res = client.call(&ink_e2e::alice(), contract_hash, 0, None).await;

            assert!(contract_hash_res.is_ok());

            Ok(())
        }
    }
}
