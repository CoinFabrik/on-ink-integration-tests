#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod block_number {

    #[ink(storage)]
    pub struct GetBlockNumber {}

    impl GetBlockNumber {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_block_number(&self) -> BlockNumber {
            self.env().block_number()
        }
    }

    impl Default for GetBlockNumber {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::advance_block, DefaultEnvironment};
        #[ink::test]
        fn get_block_number() {
            // Given
            let get_block_number = GetBlockNumber::new();

            // When
            let first_block_number = get_block_number.get_block_number();
            advance_block::<DefaultEnvironment>();
            let second_block_number = get_block_number.get_block_number();

            // Then
            assert_eq!(first_block_number, 0);
            assert_eq!(second_block_number, 1);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn get_block_number(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GetBlockNumberRef::new();
            let contract_to_call_acc_id = client
                .instantiate("get_block_number", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let call = build_message::<GetBlockNumberRef>(contract_to_call_acc_id)
                .call(|contract| contract.get_block_number());
            let first_block_number = client
                .call(&ink_e2e::alice(), call.clone(), 0, None)
                .await
                .expect("call failed")
                .return_value();

            let second_block_number = client
                .call(&ink_e2e::alice(), call, 0, None)
                .await
                .expect("call failed")
                .return_value();

            // Then
            assert_eq!(first_block_number, 1);
            assert_eq!(second_block_number, 2);
            Ok(())
        }
    }
}
