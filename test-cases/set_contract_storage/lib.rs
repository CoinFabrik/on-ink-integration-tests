#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod set_contract_storage {
    use ink::env::set_contract_storage;

    const SIZE_LIMIT: usize = (1 << 14) - 4;

    #[ink(storage)]
    pub struct SetContractStorage {}

    impl SetContractStorage {
        /// Creates a new SetContractStorage contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Stores an array that is JUST big enough to be validly allocated.
        #[ink(message)]
        pub fn set_storage_big(&self){
            set_contract_storage(&42, &[42u8; SIZE_LIMIT]);
        }

        /// Tries to store the smallest array that is too big to be validly
        /// allocated. This function should always fail.
        #[ink(message)]
        pub fn set_storage_very_big(&self){
            set_contract_storage(&42, &[42u8; SIZE_LIMIT + 1]);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn contract_storage_big() {
            // Given
            let contract = SetContractStorage::new();

            // When
            contract.set_storage_big();

            // Then
            assert_eq!(0, 0);
        }

        #[ink::test]
        #[should_panic]
        fn contract_storage_too_big() {
            // Given
            let contract = SetContractStorage::new();

            // When
            contract.set_storage_very_big();
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn contract_storage_big_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = SetContractStorageRef::new();

            // When
            let contract_acc_id = client
                .instantiate("set-contract-storage", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let message = build_message::<SetContractStorageRef>(contract_acc_id.clone())
                .call(|contract| contract.set_storage_big());

            client
                .call(&ink_e2e::alice(), message, 0, None)
                .await
                .expect("set_storage_big failed");

            Ok(())
        }

        #[ink_e2e::test]
        #[should_panic]
        async fn contract_storage_too_big_e2e(mut client: ink_e2e::Client<C, E>){
            // Given
            let constructor = SetContractStorageRef::new();

            // When
            let contract_acc_id = client
                .instantiate("set-contract-storage", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let message = build_message::<SetContractStorageRef>(contract_acc_id.clone())
                .call(|contract| contract.set_storage_very_big());

            client
                .call(&ink_e2e::bob(), message, 0, None)
                .await
                .expect("set_storage_very_big failed");
        }
        
    }
}
