#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hash_encoded {
    use ink::env::hash::{Blake2x128, Blake2x256, Keccak256, Sha2x256};
    use ink::prelude::string::String;
    use scale::Encode;

    #[ink(storage)]
    pub struct HashEncoded {}

    #[derive(Encode)]
    struct Account {
        name: String,
        age: u32,
    }

    impl HashEncoded {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn hash_sha_256(&self, name: String, age: u32) -> [u8; 32] {
            let test_data = Account { name, age };
            self.env().hash_encoded::<Sha2x256, Account>(&test_data)
        }

        #[ink(message)]
        pub fn hash_blake_128(&self, name: String, age: u32) -> [u8; 16] {
            let test_data = Account { name, age };
            self.env().hash_encoded::<Blake2x128, Account>(&test_data)
        }

        #[ink(message)]
        pub fn hash_blake_256(&self, name: String, age: u32) -> [u8; 32] {
            let test_data = Account { name, age };
            self.env().hash_encoded::<Blake2x256, Account>(&test_data)
        }

        #[ink(message)]
        pub fn hash_keccak_256(&self, name: String, age: u32) -> [u8; 32] {
            let test_data = Account { name, age };
            self.env().hash_encoded::<Keccak256, Account>(&test_data)
        }
    }

    impl Default for HashEncoded {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    const NAME: &str = "CoinFabrik";
    #[cfg(test)]
    const AGE: u32 = 9;
    #[cfg(test)]
    const EXPECTED_SHA256_OUTPUT: [u8; 32] = [
        17, 16, 147, 141, 78, 60, 123, 76, 255, 208, 54, 80, 16, 12, 80, 246, 152, 244, 48, 8, 29,
        113, 30, 77, 0, 83, 184, 145, 18, 147, 10, 142,
    ];
    #[cfg(test)]
    const EXPECTED_BLAKE128_OUTPUT: [u8; 16] = [
        147, 117, 200, 111, 255, 7, 223, 69, 213, 16, 102, 228, 232, 103, 228, 229,
    ];
    #[cfg(test)]
    const EXPECTED_BLAKE256_OUTPUT: [u8; 32] = [
        184, 100, 185, 186, 117, 254, 178, 11, 190, 30, 88, 146, 18, 188, 8, 92, 254, 119, 145,
        231, 167, 58, 249, 208, 107, 6, 130, 38, 226, 45, 144, 25,
    ];
    #[cfg(test)]
    const EXPECTED_KECCAK256_OUTPUT: [u8; 32] = [
        63, 29, 47, 151, 56, 88, 65, 75, 141, 236, 242, 218, 220, 140, 220, 136, 251, 23, 244, 122,
        193, 38, 195, 250, 246, 213, 204, 178, 227, 88, 242, 175,
    ];

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn hash_sha_256() {
            // Given
            let contract = HashEncoded::new();

            // When
            let hash_encoded = contract.hash_sha_256(String::from(NAME), AGE);

            // Then
            assert_eq!(hash_encoded, EXPECTED_SHA256_OUTPUT);
        }

        #[ink::test]
        fn hash_blake_128() {
            // Given
            let contract = HashEncoded::new();

            // When
            let hash_encoded = contract.hash_blake_128(String::from(NAME), AGE);

            // Then
            assert_eq!(hash_encoded, EXPECTED_BLAKE128_OUTPUT);
        }

        #[ink::test]
        fn hash_blake_256() {
            // Given
            let contract = HashEncoded::new();

            // When
            let hash_encoded = contract.hash_blake_256(String::from(NAME), AGE);

            // Then
            assert_eq!(hash_encoded, EXPECTED_BLAKE256_OUTPUT);
        }

        #[ink::test]
        fn hash_keccak_256() {
            // Given
            let contract = HashEncoded::new();

            // When
            let hash_encoded = contract.hash_keccak_256(String::from(NAME), AGE);

            // Then
            assert_eq!(hash_encoded, EXPECTED_KECCAK256_OUTPUT);
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
            let constructor = HashEncodedRef::new();

            let contract_acc_id = client
                .instantiate("hash_encoded", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_encoded_call = build_message::<HashEncodedRef>(contract_acc_id)
                .call(|contract| contract.hash_sha_256(String::from(NAME), AGE));
            let hash_encoded_res = client
                .call(&ink_e2e::bob(), hash_encoded_call, 0, None)
                .await
                .expect("hash_encoded failed");

            // Then
            assert_eq!(hash_encoded_res.return_value(), EXPECTED_SHA256_OUTPUT);

            Ok(())
        }

        #[ink_e2e::test]
        async fn hash_blake_128(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashEncodedRef::new();

            let contract_acc_id = client
                .instantiate("hash_encoded", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_encoded_call = build_message::<HashEncodedRef>(contract_acc_id)
                .call(|contract| contract.hash_blake_128(String::from(NAME), AGE));
            let hash_encoded_res = client
                .call(&ink_e2e::bob(), hash_encoded_call, 0, None)
                .await
                .expect("hash_encoded failed");

            // Then
            assert_eq!(hash_encoded_res.return_value(), EXPECTED_BLAKE128_OUTPUT);

            Ok(())
        }

        #[ink_e2e::test]
        async fn hash_blake_256(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashEncodedRef::new();

            let contract_acc_id = client
                .instantiate("hash_encoded", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_encoded_call = build_message::<HashEncodedRef>(contract_acc_id)
                .call(|contract| contract.hash_blake_256(String::from(NAME), AGE));
            let hash_encoded_res = client
                .call(&ink_e2e::bob(), hash_encoded_call, 0, None)
                .await
                .expect("hash_encoded failed");

            // Then
            assert_eq!(hash_encoded_res.return_value(), EXPECTED_BLAKE256_OUTPUT);

            Ok(())
        }

        #[ink_e2e::test]
        async fn hash_keccak_256(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HashEncodedRef::new();

            let contract_acc_id = client
                .instantiate("hash_encoded", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let hash_encoded_call = build_message::<HashEncodedRef>(contract_acc_id)
                .call(|contract| contract.hash_keccak_256(String::from(NAME), AGE));
            let hash_encoded_res = client
                .call(&ink_e2e::bob(), hash_encoded_call, 0, None)
                .await
                .expect("hash_encoded failed");

            // Then
            assert_eq!(hash_encoded_res.return_value(), EXPECTED_KECCAK256_OUTPUT);

            Ok(())
        }
    }
}
