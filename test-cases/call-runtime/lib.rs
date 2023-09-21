#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;
use sp_runtime::MultiAddress;

/// A part of the runtime dispatchable API.
///
/// For now, `ink!` doesn't provide any support for exposing the real `RuntimeCall` enum,
/// which fully describes the composed API of all the pallets present in runtime. Hence,
/// in order to use `call-runtime` functionality, we have to provide at least a partial
/// object, which correctly encodes the target extrinsic.
///
/// You can investigate the full `RuntimeCall` definition by either expanding
/// `construct_runtime!` macro application or by using secondary tools for reading chain
/// metadata, like `subxt`.
#[derive(scale::Encode)]
enum RuntimeCall {
    /// This index can be found by investigating runtime configuration. You can check the
    /// pallet order inside `construct_runtime!` block and read the position of your
    /// pallet (0-based).
    ///
    ///
    /// [See here for more.](https://substrate.stackexchange.com/questions/778/how-to-get-pallet-index-u8-of-a-pallet-in-runtime)
    #[codec(index = 4)]
    Balances(BalancesCall),
}

#[derive(scale::Encode)]
enum BalancesCall {
    /// This index can be found by investigating the pallet dispatchable API. In your
    /// pallet code, look for `#[pallet::call]` section and check
    /// `#[pallet::call_index(x)]` attribute of the call. If these attributes are
    /// missing, use source-code order (0-based).
    #[codec(index = 0)]
    Transfer {
        dest: MultiAddress<AccountId, ()>,
        #[codec(compact)]
        value: u128,
    },
}

#[ink::contract]
mod runtime_call {
    use crate::{BalancesCall, RuntimeCall};

    use ink::env::Error as EnvError;

    /// A trivial contract with a single message, that uses `call-runtime` API for
    /// performing native token transfer.
    #[ink(storage)]
    #[derive(Default)]
    pub struct RuntimeCaller;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RuntimeError {
        CallRuntimeFailed,
    }

    impl From<EnvError> for RuntimeError {
        fn from(e: EnvError) -> Self {
            match e {
                EnvError::CallRuntimeFailed => RuntimeError::CallRuntimeFailed,
                _ => panic!("Unexpected error from `pallet-contracts`."),
            }
        }
    }

    impl RuntimeCaller {
        /// The constructor is `payable`, so that during instantiation it can be given
        /// some tokens that will be further transferred with
        /// `transfer_through_runtime` message.
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Tries to transfer `value` from the contract's balance to `receiver`.
        ///
        /// Fails if:
        ///  - called in the off-chain environment
        ///  - the chain forbids contracts to call `Balances::transfer` (`CallFilter` is
        ///    too restrictive)
        ///  - after the transfer, `receiver` doesn't have at least existential deposit
        ///  - the contract doesn't have enough balance
        #[ink(message)]
        pub fn transfer_through_runtime(
            &mut self,
            receiver: AccountId,
            value: Balance,
        ) -> Result<(), RuntimeError> {
            self.env()
                .call_runtime(&RuntimeCall::Balances(BalancesCall::Transfer {
                    dest: receiver.into(),
                    value,
                }))
                .map_err(Into::into)
        }

        #[ink(message)]
        pub fn balance(&self) -> Balance {
            self.env().balance()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::runtime_call::*;

        // TODO: this test shouldn't panic when ink integration test is improved.
        #[ink::test]
        #[should_panic(
            expected = "not implemented: off-chain environment does not support `call_runtime`"
        )]
        fn test_call_runtime() {
            let mut contract = RuntimeCaller::new();
            let alice = AccountId::from([1u8; 32]);

            contract
                .transfer_through_runtime(alice, 10u128.into())
                .expect("this must never fail");

            // Needs balanace check here.
            assert_eq!(0, 0);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink::{
            env::{test::default_accounts, DefaultEnvironment},
            primitives::AccountId,
        };
        use ink_e2e::build_message;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        /// The base number of indivisible units for balances on the
        /// `substrate-contracts-node`.
        const UNIT: Balance = 1_000_000_000_000;

        /// The contract will be given 1000 tokens during instantiation.
        const CONTRACT_BALANCE: Balance = 1_000 * UNIT;

        /// The receiver will get enough funds to have the required existential deposit.
        ///
        /// If your chain has this threshold higher, increase the transfer value.
        const TRANSFER_VALUE: Balance = 1 / 10 * UNIT;

        /// Positive case scenario:
        ///  - `call_runtime` is enabled
        ///  - the call is valid
        ///  - the call execution succeeds
        #[ink_e2e::test]
        async fn test_call_runtime(mut client: Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = RuntimeCallerRef::new();
            let contract_acc_id = client
                .instantiate(
                    "call-runtime",
                    &ink_e2e::alice(),
                    constructor,
                    CONTRACT_BALANCE,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let receiver: AccountId = default_accounts::<DefaultEnvironment>().bob;

            let contract_balance_before = client
                .balance(contract_acc_id)
                .await
                .expect("Failed to get account balance");
            let receiver_balance_before = client
                .balance(receiver)
                .await
                .expect("Failed to get account balance");

            // when
            let transfer_message = build_message::<RuntimeCallerRef>(contract_acc_id)
                .call(|caller| caller.transfer_through_runtime(receiver, TRANSFER_VALUE));

            let call_res = client
                .call(&ink_e2e::alice(), transfer_message, 0, None)
                .await
                .expect("call failed");

            assert!(call_res.return_value().is_ok());

            // then
            let contract_balance_after = client
                .balance(contract_acc_id)
                .await
                .expect("Failed to get account balance");
            let receiver_balance_after = client
                .balance(receiver)
                .await
                .expect("Failed to get account balance");

            assert_eq!(
                contract_balance_before,
                contract_balance_after + TRANSFER_VALUE
            );
            assert_eq!(
                receiver_balance_before,
                receiver_balance_after - TRANSFER_VALUE
            );

            Ok(())
        }
    }
}
