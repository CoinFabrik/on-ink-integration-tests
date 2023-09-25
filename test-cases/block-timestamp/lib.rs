#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod block_timestamp {

    #[ink(storage)]
    pub struct BlockTimestamp {}

    impl BlockTimestamp {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_block_timestamp(&self) -> u64 {
            self.env().block_timestamp()
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::{test::advance_block, DefaultEnvironment};

        use super::*;

        #[ink::test]
        fn split_profit_works() {
            // Given
            let contract = BlockTimestamp::new();

            // When
            let timestamp = contract.get_block_timestamp();
            advance_block::<DefaultEnvironment>();

            let timestamp_2 = contract.get_block_timestamp();
            advance_block::<DefaultEnvironment>();

            // Then
            assert!(timestamp < timestamp_2);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn split_profit_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = BlockTimestampRef::new();

            let contract_acc_id = client
                .instantiate("block_timestamp", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let timestamp = build_message::<BlockTimestampRef>(contract_acc_id.clone())
                .call(|contract| contract.get_block_timestamp());

            let timestamp_res = client
                .call(&ink_e2e::bob(), timestamp.clone(), 0, None)
                .await
                .expect("block_timestamp failed");

            let timestamp_res_2 = client
                .call(&ink_e2e::bob(), timestamp, 0, None)
                .await
                .expect("block_timestamp failed");

            // Then
            assert!(timestamp_res.return_value() < timestamp_res_2.return_value());

            Ok(())
        }
    }
}
