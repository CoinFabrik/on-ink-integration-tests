#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod set_code_hash {
    use ink::env::set_code_hash;

    #[ink(storage)]
    pub struct SetCodeHash {}

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidCodeHash,
    }

    impl SetCodeHash {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
            let res = set_code_hash(&value);

            if res.is_err() {
                return res.map_err(|_| Error::InvalidCodeHash);
            }

            Ok(())
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
        #[ink::test]
        fn update_code_works() {
            // Arrange
            let original_contract = SetCodeHash::new();
            let code_hash = [0x42; 32];
            // Act
            let res = original_contract.update_code(code_hash);

            // Assert
            assert_eq!(res, Ok(()));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test(additional_contracts = "../contract_replacement/Cargo.toml")]
        async fn update_code_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let original_contract_contructor = SetCodeHashRef::new();
            let original_contract_acc_id = client
                .instantiate(
                    "set-code-hash",
                    &ink_e2e::alice(),
                    original_contract_contructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let new_code_hash = client
                .upload("contract-replacement", &ink_e2e::alice(), None)
                .await
                .expect("uploading `contract-replacement` failed")
                .code_hash
                .as_ref()
                .try_into()
                .unwrap();

            // Act
            let update_code = build_message::<SetCodeHashRef>(original_contract_acc_id.clone())
                .call(|contract| contract.update_code(new_code_hash));
            let get_res = client.call(&ink_e2e::alice(), update_code, 0, None).await;

            // Assert
            assert!(get_res.is_ok());

            Ok(())
        }
    }
}
