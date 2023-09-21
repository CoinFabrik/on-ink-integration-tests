#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod set_code_hash {
    use ink::env::set_code_hash;

    #[ink(storage)]
    pub struct SetCodeHash {
        admin: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidCodeHash,
        NotAnAdmin,
    }

    impl SetCodeHash {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
            }
        }

        /// Sets new code hash to contract (doesn't check caller is admin)
        #[ink(message)]
        pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
            let res = set_code_hash(&value);

            if res.is_err() {
                return res.map_err(|_| Error::InvalidCodeHash);
            }

            Ok(())
        }

        /// Returns the code hash of the contract
        #[ink(message)]
        pub fn get_code(&self) -> Hash {
            self.env()
                .code_hash(&self.env().account_id())
                .expect("Failed to get code hash")
        }
    }

    impl Default for SetCodeHash {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // TODO: this test shouldn't panic when ink integration test is improved.
        #[ink::test]
        #[should_panic(
            expected = "not implemented: off-chain environment does not support `set_code_hash`"
        )]
        fn update_code_works() {
            let original_contract = SetCodeHash::new();
            let code_hash = [0x42; 32];

            let res = original_contract.update_code(code_hash);

            assert_eq!(res, Ok(()));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test(additional_contracts = "./contract_replacement/Cargo.toml")]
        async fn update_code_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Instantiate contract
            let constructor = SetCodeHashRef::new();
            let contract_acc_id = client
                .instantiate("set-code-hash", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Get original code hash
            let get_code = build_message::<SetCodeHashRef>(contract_acc_id)
                .call(|contract| contract.get_code());
            let original_code_hash = client
                .call_dry_run(&ink_e2e::alice(), &get_code, 0, None)
                .await
                .return_value();

            // Get contract's replacement code hash
            let new_code_hash = client
                .upload("contract-replacement", &ink_e2e::alice(), None)
                .await
                .expect("uploading `contract-replacement` failed")
                .code_hash;

            // Check code hashes initially are different
            assert_ne!(new_code_hash, original_code_hash);

            // Update contract's code hash
            let new_code_hash_ref: [u8; 32] = new_code_hash.as_ref().try_into().unwrap();
            let update_code = build_message::<SetCodeHashRef>(contract_acc_id)
                .call(|contract| contract.update_code(new_code_hash_ref));
            let update_code_res = client.call(&ink_e2e::alice(), update_code, 0, None).await;

            // Assert
            assert!(update_code_res.is_ok());

            // Compare codes
            let get_code = build_message::<SetCodeHashRef>(contract_acc_id)
                .call(|contract| contract.get_code());
            let updated_code_hash = client
                .call_dry_run(&ink_e2e::alice(), &get_code, 0, None)
                .await
                .return_value();

            assert_eq!(new_code_hash.as_ref(), updated_code_hash.as_ref());

            Ok(())
        }
    }
}
