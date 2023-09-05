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
            self.get_code_hash(self.env().account_id())
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

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn code_hash_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = CodeHashRef::new();

            let contract_acc_id = client
                .instantiate("code_hash", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let split_profit = build_message::<TemplateRef>(contract_acc_id.clone())
                .call(|contract| contract.split_profit(33, 100));
            let split_profit_res = client
                .call(&ink_e2e::bob(), split_profit, 0, None)
                .await
                .expect("split_profit failed");
            assert_eq!(split_profit_res.return_value(), 0);

            Ok(())
        }
    }
}
