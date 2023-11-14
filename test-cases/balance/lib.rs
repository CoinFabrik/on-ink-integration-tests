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
    const CORRECTED_BALANCE_INTEGRATION: u128 = 1000000000;

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        #[ink_e2e::test]
        async fn get_balance(mut client: Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = GetBalanceRef::new();
            let contract_to_call_acc_id = client
                .instantiate("get_balance", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let contract_balance_before = client
                .balance(contract_to_call_acc_id)
                .await
                .expect("Failed to get account balance");

            // Then
            assert_eq!(contract_balance_before, INITIAL_BALANCE_E2E);
            Ok(())
        }

        #[ink_e2e::test]
        async fn default_balances(mut client: Client<C, E>) -> E2EResult<()> {
            //get the balance from alice, bob and all the default accounts

            let alice_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Alice))
                .await
                .expect("Failed to get account balance");

            let bob_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Bob))
                .await
                .expect("Failed to get account balance");

            let charlie_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie))
                .await
                .expect("Failed to get account balance");

            let dave_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Dave))
                .await
                .expect("Failed to get account balance");

            let eve_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Eve))
                .await
                .expect("Failed to get account balance");

            let ferdie_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Ferdie))
                .await
                .expect("Failed to get account balance");

            let one_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::One))
                .await
                .expect("Failed to get account balance");

            let two_balance_before = client
                .balance(ink_e2e::account_id(ink_e2e::AccountKeyring::Two))
                .await
                .expect("Failed to get account balance");

            let arr = [
                alice_balance_before,
                bob_balance_before,
                charlie_balance_before,
                dave_balance_before,
                eve_balance_before,
                ferdie_balance_before,
                one_balance_before,
                two_balance_before,
            ];

            assert_eq!(arr, [CORRECTED_BALANCE_INTEGRATION; 8]);

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
