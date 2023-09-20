#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hash_bytes {
    use ink::env::hash::{Blake2x128, Blake2x256, Keccak256, Sha2x256};

    #[ink(storage)]
    pub struct HashBytes {}

    impl HashBytes {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn hash_sha_256(&self, input: [u8; 32]) -> [u8; 32] {
            self.env().hash_bytes::<Sha2x256>(&input)
        }

        #[ink(message)]
        pub fn hash_blake_128(&self, input: [u8; 32]) -> [u8; 16] {
            self.env().hash_bytes::<Blake2x128>(&input)
        }

        #[ink(message)]
        pub fn hash_blake_256(&self, input: [u8; 32]) -> [u8; 32] {
            self.env().hash_bytes::<Blake2x256>(&input)
        }

        #[ink(message)]
        pub fn hash_keccak_256(&self, input: [u8; 32]) -> [u8; 32] {
            self.env().hash_bytes::<Keccak256>(&input)
        }
    }

    impl Default for HashBytes {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    const HASH_INPUT: [u8; 32] = [0x01; 32];
    #[cfg(test)]
    const EXPECTED_SHA256_OUTPUT: [u8; 32] = [
        114, 205, 110, 132, 34, 196, 7, 251, 109, 9, 134, 144, 241, 19, 11, 125, 237, 126, 194,
        247, 245, 225, 211, 11, 217, 213, 33, 240, 21, 54, 55, 147,
    ];
    #[cfg(test)]
    const EXPECTED_BLAKE128_OUTPUT: [u8; 16] = [
        192, 53, 248, 83, 252, 208, 240, 88, 158, 48, 201, 226, 220, 26, 15, 87,
    ];
    #[cfg(test)]
    const EXPECTED_BLAKE256_OUTPUT: [u8; 32] = [
        244, 12, 234, 248, 110, 87, 118, 146, 51, 50, 184, 216, 253, 59, 239, 132, 156, 173, 177,
        156, 105, 150, 188, 39, 42, 241, 246, 72, 217, 86, 106, 76,
    ];
    #[cfg(test)]
    const EXPECTED_KECCAK256_OUTPUT: [u8; 32] = [
        206, 188, 136, 130, 254, 203, 236, 127, 184, 13, 44, 244, 179, 18, 190, 192, 24, 136, 76,
        45, 102, 102, 124, 103, 169, 5, 8, 33, 75, 216, 186, 252,
    ];

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn hash_sha_256() {
            // Given
            let contract = HashBytes::new();

            // When
            let hash_bytes = contract.hash_sha_256(HASH_INPUT);

            // Then
            assert_eq!(hash_bytes, EXPECTED_SHA256_OUTPUT);
        }

        #[ink::test]
        fn hash_blake_128() {
            // Given
            let contract = HashBytes::new();

            // When
            let hash_bytes = contract.hash_blake_128(HASH_INPUT);

            // Then
            assert_eq!(hash_bytes, EXPECTED_BLAKE128_OUTPUT);
        }

        #[ink::test]
        fn hash_blake_256() {
            // Given
            let contract = HashBytes::new();

            // When
            let hash_bytes = contract.hash_blake_256(HASH_INPUT);

            // Then
            assert_eq!(hash_bytes, EXPECTED_BLAKE256_OUTPUT);
        }

        #[ink::test]
        fn hash_keccak_256() {
            // Given
            let contract = HashBytes::new();

            // When
            let hash_bytes = contract.hash_keccak_256(HASH_INPUT);

            // Then
            assert_eq!(hash_bytes, EXPECTED_KECCAK256_OUTPUT);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn hash_sha_256(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashBytesRef::new();

            let contract_acc_id = client
                .instantiate("hash_bytes", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_bytes_call = build_message::<HashBytesRef>(contract_acc_id)
                .call(|contract| contract.hash_sha_256(HASH_INPUT));
            let hash_bytes_res = client
                .call(&ink_e2e::bob(), hash_bytes_call, 0, None)
                .await
                .expect("hash_bytes failed");

            // Then
            assert_eq!(hash_bytes_res.return_value(), EXPECTED_SHA256_OUTPUT);

            Ok(())
        }

        #[ink_e2e::test]
        async fn hash_blake_128(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashBytesRef::new();

            let contract_acc_id = client
                .instantiate("hash_bytes", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_bytes_call = build_message::<HashBytesRef>(contract_acc_id)
                .call(|contract| contract.hash_blake_128(HASH_INPUT));
            let hash_bytes_res = client
                .call(&ink_e2e::bob(), hash_bytes_call, 0, None)
                .await
                .expect("hash_bytes failed");

            // Then
            assert_eq!(hash_bytes_res.return_value(), EXPECTED_BLAKE128_OUTPUT);

            Ok(())
        }

        #[ink_e2e::test]
        async fn hash_blake_256(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashBytesRef::new();

            let contract_acc_id = client
                .instantiate("hash_bytes", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_bytes_call = build_message::<HashBytesRef>(contract_acc_id)
                .call(|contract| contract.hash_blake_256(HASH_INPUT));
            let hash_bytes_res = client
                .call(&ink_e2e::bob(), hash_bytes_call, 0, None)
                .await
                .expect("hash_bytes failed");

            // Then
            assert_eq!(hash_bytes_res.return_value(), EXPECTED_BLAKE256_OUTPUT);

            Ok(())
        }

        #[ink_e2e::test]
        async fn hash_keccak_256(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashBytesRef::new();

            let contract_acc_id = client
                .instantiate("hash_bytes", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_bytes_call = build_message::<HashBytesRef>(contract_acc_id)
                .call(|contract| contract.hash_keccak_256(HASH_INPUT));
            let hash_bytes_res = client
                .call(&ink_e2e::bob(), hash_bytes_call, 0, None)
                .await
                .expect("hash_bytes failed");

            // Then
            assert_eq!(hash_bytes_res.return_value(), EXPECTED_KECCAK256_OUTPUT);

            Ok(())
        }
    }
}
