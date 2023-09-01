#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod custom_default_accounts {

    #[ink(storage)]
    pub struct CustomDefaultAccounts {}

    impl CustomDefaultAccounts {
        /// Creates a new Template contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn message(&self) {}
    }

    impl Default for CustomDefaultAccounts {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::DefaultAccounts;
        use ink_e2e;

        #[ink::test]
        #[should_panic]
        fn test_default_accounts() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_alice_account_id = integration_test_accounts.alice;
            let integration_bob_account_id = integration_test_accounts.bob;
            let integration_charlie_account_id = integration_test_accounts.charlie;
            let integration_django_account_id = integration_test_accounts.django;
            let integration_eve_account_id = integration_test_accounts.eve;
            let integration_frank_account_id = integration_test_accounts.frank;

            let e2e_alice_account_id: AccountId =
                ink_e2e::AccountKeyring::Alice.to_raw_public().into();
            let e2e_bob_account_id: AccountId = ink_e2e::AccountKeyring::Bob.to_raw_public().into();
            let e2e_charlie_account_id: AccountId =
                ink_e2e::AccountKeyring::Charlie.to_raw_public().into();
            let e2e_dave_account_id: AccountId =
                ink_e2e::AccountKeyring::Dave.to_raw_public().into();
            let e2e_eve_account_id: AccountId = ink_e2e::AccountKeyring::Eve.to_raw_public().into();
            let e2e_ferdie_account_id: AccountId =
                ink_e2e::AccountKeyring::Ferdie.to_raw_public().into();

            assert_eq!(integration_alice_account_id, e2e_alice_account_id);
            assert_eq!(integration_bob_account_id, e2e_bob_account_id);
            assert_eq!(integration_charlie_account_id, e2e_charlie_account_id);
            assert_eq!(integration_django_account_id, e2e_dave_account_id); // Django does not exist in e2e, dave does not exist in integration tests
            assert_eq!(integration_eve_account_id, e2e_eve_account_id);
            assert_eq!(integration_frank_account_id, e2e_ferdie_account_id); // Frank does not exist in e2e, ferdie does not exist in integration tests
        }
    }
}
