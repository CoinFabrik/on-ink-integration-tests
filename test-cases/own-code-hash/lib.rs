#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod own_code_hash {

    #[ink(storage)]
    pub struct OwnCodeHash {}

    impl OwnCodeHash {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn own_code_hash(&self) -> Hash {
            self.env().own_code_hash().unwrap()
        }
    }

    impl Default for OwnCodeHash {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_own_code_hash() {
            let own_code_hash = OwnCodeHash::new();
            assert_eq!(own_code_hash.own_code_hash(), Hash::from([0x0; 32]));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_own_code_hash_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = OwnCodeHashRef::new();
            let contract_acc_id = client
                .instantiate("own_code_hash", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            let own_code_hash = build_message::<OwnCodeHashRef>(contract_acc_id)
                .call(|contract| contract.own_code_hash());
            client
                .call(&ink_e2e::bob(), own_code_hash, 0, None)
                .await
                .expect("split_profit failed");
            Ok(())
        }
    }
}
