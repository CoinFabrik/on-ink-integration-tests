#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hash_bytes {
    use ink::env::hash::Blake2x256;

    #[ink(storage)]
    pub struct HashBytes {}

    impl HashBytes {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_hash_bytes(&self, input: [u8; 32]) -> [u8; 32] {
            self.env().hash_bytes::<Blake2x256>(&input)
        }
    }

    impl Default for HashBytes {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn hash_bytes_works() {
            // Given
            let contract = HashBytes::new();

            // When
            let hash_bytes = contract.get_hash_bytes([0x01; 32]);

            // Then
            assert!(hash_bytes.len() == 32);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn hash_bytes_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashBytesRef::new();
            let contract_acc_id = client
                .instantiate("hash_bytes", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_bytes_call = build_message::<HashBytesRef>(contract_acc_id)
                .call(|contract| contract.get_hash_bytes([0x01; 32]));
            let hash_bytes_res = client
                .call(&ink_e2e::bob(), hash_bytes_call, 0, None)
                .await
                .expect("hash_bytes failed");

            // Then
            assert!(!hash_bytes_res.dry_run.is_err());
            assert!(hash_bytes_res.return_value().len() == 32);

            Ok(())
        }
    }
}
