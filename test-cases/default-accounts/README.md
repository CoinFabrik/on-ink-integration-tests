# Function `default_accounts`

## Description

The aim of this function is to provide default test accounts, each given a fantasy name from "A" to "F" (Alice, Bob, etc.). The intention is for these accounts to exhibit determinism, thereby ensuring a predictable test environment. In this sense, it is reasonable to anticipate obtaining the same addresses both in Integration and in End-to-End scenarios.

On Integration, function resides within [`ink_env::test`](https://paritytech.github.io/ink/ink_env/test/index.html)

```rust
pub fn default_accounts<T>() -> DefaultAccounts<T>
```

and each address holds different amounts of value.

On End-to-End there's no such implementation but an [`ink_e2e::AccountKeyring`](https://paritytech.github.io/ink/ink_e2e/enum.AccountKeyring.html) `enum` type which provides the default test accounts.

## Related ink! functions and types

- [`default_accounts`](https://paritytech.github.io/ink/ink_env/test/fn.default_accounts.html)
- [`DefaultAccounts`](https://paritytech.github.io/ink/ink_env/test/struct.DefaultAccounts.html)
- [`AccountKeyring`](https://paritytech.github.io/ink/ink_e2e/enum.AccountKeyring.html)

## Test case

How the test cases actually tests the given functionality.

## Comparison Integration vs E2E

Description if the function called worked on integration, E2E or both.

Differences found in implementation.

## Result

- Can it be implemented? Yes
- How long will it take?
- Ideas for implementation
- Other notes
