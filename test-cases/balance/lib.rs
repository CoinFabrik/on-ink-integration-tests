#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod get_balance {

    #[ink(storage)]
    pub struct GetBalance {}

    impl GetBalance {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }
    }

    impl Default for GetBalance {
        fn default() -> Self {
            Self::new()
        }
    }


    #[cfg(test)]
    const INITIAL_BALANCE_INTEGRATION: u128 = 1000000;
    #[cfg(all(test, feature = "e2e-tests"))]
    const INITIAL_BALANCE_E2E: u128 = 1000000000;


    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        #[ink_e2e::test]
        async fn get_balance(mut client: Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GetBalanceRef::new();
            let contract_to_call_acc_id = client
                .instantiate(
                    "get_balance",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let contract_balance_before = client
                .balance(contract_to_call_acc_id)
                .await
                .expect("Failed to get account balance");


            // Then
            assert_eq!(
                contract_balance_before,
                INITIAL_BALANCE_E2E
            );
            Ok(())
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::set_callee, DefaultEnvironment};
        #[ink::test]
        fn get_balance() {
            // Given
            let callee_id = AccountId::from([0x01; 32]);
            set_callee::<DefaultEnvironment>(callee_id);
            let contract = GetBalance::new();
            // When
            let balance = contract.get_balance();
            // Then
            assert_eq!(balance, INITIAL_BALANCE_INTEGRATION);
        }

        #[ink::test]
        fn ambient_match() {
            assert_eq!(INITIAL_BALANCE_E2E, INITIAL_BALANCE_INTEGRATION);
        }
    }

}
