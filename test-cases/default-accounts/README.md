# Function `default_accounts`

## Description

The aim of this function is to provide default test accounts, each given a fantasy name from "A" to "F" (Alice, Bob, etc.). The intention is for these accounts to exhibit determinism, thereby ensuring a predictable test environment. In this sense, it is reasonable to anticipate obtaining the same addresses both in Integration and in End-to-End scenarios.

On Integration, function resides within [`ink_env::test`](https://paritytech.github.io/ink/ink_env/test/index.html)

```rust
pub fn default_accounts<T>() -> DefaultAccounts<T>
```

and each address holds different amounts of value. There are in total 6 test default accounts ("A" to "F").

On End-to-End there's no such implementation but an [`ink_e2e::AccountKeyring`](https://paritytech.github.io/ink/ink_e2e/enum.AccountKeyring.html) `enum` type which provides the default test accounts. There are in total 8 test default accounts ("A" to "F" and two more extra).

## Related ink! functions and types

- [`default_accounts`](https://paritytech.github.io/ink/ink_env/test/fn.default_accounts.html)
- [`DefaultAccounts`](https://paritytech.github.io/ink/ink_env/test/struct.DefaultAccounts.html)
- [`AccountKeyring`](https://paritytech.github.io/ink/ink_e2e/enum.AccountKeyring.html)

## Test case

There is a test case corresponding to each default account in the End-to-End environment, checking that the address is the same in Integration one. After this exploration-instance, it is desibrable to also examine balance consistency.

| \#  | Test                                                            | Integration | E2E |
| --- | --------------------------------------------------------------- | :---------: | :-: |
| 1   | The "A" default account in Integration is the same as in E2E.   |     ❌      | ✅  |
| 2   | The "B" default account in Integration is the same as in E2E.   |     ❌      | ✅  |
| 2   | The "C" default account in Integration is the same as in E2E.   |     ❌      | ✅  |
| 2   | The "D" default account in Integration is the same as in E2E.   |     ❌      | ✅  |
| 2   | The "E" default account in Integration is the same as in E2E.   |     ❌      | ✅  |
| 2   | The "F" default account in Integration is the same as in E2E.   |     ❌      | ✅  |
| 2   | The "One" default account in Integration is the same as in E2E. |     ❌      | ✅  |
| 2   | The "Two" default account in Integration is the same as in E2E. |     ❌      | ✅  |

## Comparison Integration vs E2E

All default accounts addresses are different. Furthermore, there is a disparity in the fantasy names assigned to accounts "D" and "F." Additionally, the default accounts "One" and "Two" are two additional accounts present in E2E but absent in the Integration environment.

## Result

This is relatively trivial. It can be implemented in a single day. For backwards compatibility django and frank should retain their current names, and dave and ferdie added as their respective aliases.

## Update on Correcting this Issue

Now the integration tests mimic the account setup in e2e tests. We changed the name of the accounts "Django" to "Dave" and "Frank" to "Ferdie". On the other hand, there were two accounts in e2e that did not exist in integration tests, accounts “one” and “two”. We added these accounts to integration tests.


Moreover, since e2e tests were drawing these accounts from the library `sp_keyring::sr25519::Keyring`, we made integration tests depend on the same library in order to account for future changes in this lib.

