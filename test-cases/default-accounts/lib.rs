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
        fn test_alice_account() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_alice_account_id = integration_test_accounts.alice;

            let e2e_alice_account_id: AccountId =
                ink_e2e::AccountKeyring::Alice.to_raw_public().into();

            assert_eq!(integration_alice_account_id, e2e_alice_account_id);
        }

        #[ink::test]
        fn test_bob_account() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_bob_account_id = integration_test_accounts.bob;

            let e2e_bob_account_id: AccountId = ink_e2e::AccountKeyring::Bob.to_raw_public().into();

            assert_eq!(integration_bob_account_id, e2e_bob_account_id);
        }

        #[ink::test]
        fn test_charlie_account() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_charlie_account_id = integration_test_accounts.charlie;

            let e2e_charlie_account_id: AccountId =
                ink_e2e::AccountKeyring::Charlie.to_raw_public().into();

            assert_eq!(integration_charlie_account_id, e2e_charlie_account_id);
        }

        #[ink::test]
        fn test_dave_account() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_dave_account_id = integration_test_accounts.django;

            let e2e_dave_account_id: AccountId =
                ink_e2e::AccountKeyring::Dave.to_raw_public().into();

            // There is no Django in the e2e test accounts, there is no Dave in the integration test accounts
            assert_eq!(integration_dave_account_id, e2e_dave_account_id);
        }

        #[ink::test]
        fn test_eve_account() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_eve_account_id = integration_test_accounts.eve;

            let e2e_eve_account_id: AccountId = ink_e2e::AccountKeyring::Eve.to_raw_public().into();

            assert_eq!(integration_eve_account_id, e2e_eve_account_id);
        }

        #[ink::test]
        fn test_frank_account() {
            let integration_test_accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let integration_frank_account_id = integration_test_accounts.frank;

            let e2e_frank_account_id: AccountId =
                ink_e2e::AccountKeyring::Ferdie.to_raw_public().into();

            // There is no Frank in the e2e test accounts, there is no Ferdie in the integration test accounts
            assert_eq!(integration_frank_account_id, e2e_frank_account_id);
        }

        #[ink::test]
        fn test_one_account() {
            let e2e_one_account_id: AccountId = ink_e2e::AccountKeyring::One.to_raw_public().into();

            let integration_one_account_id: AccountId = AccountId::from([0x01; 32]);

            println!(
                "There is no One in Integration test accounts, which is \nOne: {:?}",
                e2e_one_account_id
            );

            assert_eq!(integration_one_account_id, e2e_one_account_id);
        }

        #[ink::test]
        fn test_two_account() {
            let e2e_two_account_id: AccountId = ink_e2e::AccountKeyring::Two.to_raw_public().into();

            let integration_two_account_id: AccountId = AccountId::from([0x02; 32]);

            println!(
                "There is no Two in Integration test accounts, which is \nTwo: {:?}",
                e2e_two_account_id
            );

            assert_eq!(integration_two_account_id, e2e_two_account_id);
        }
    }
}
