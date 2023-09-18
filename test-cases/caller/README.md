# Function `caller`

```rust
impl Caller {
    #[ink(message)]
    pub fn caller(&self) -> AccountId {
```

## Description

The caller function retrieves the account ID of the entity that called the currently executed contract.

## Related ink! functions

- [`caller`](https://paritytech.github.io/ink/ink_env/fn.caller.html)

## Test case

The contract possesses a function named caller that allows querying the account ID of the caller. For example:

```rust
let contract = Caller::new();
let caller_id = contract.caller();
```

## Comparison Integration vs E2E

Both the integration and end-to-end tests work as expected. Notably, the caller in the integration test has been updated to match the e2e test caller. Specifically, both tests now use the `ink_e2e::AccountKeyring::Bob` as the caller, showcasing that in both integration and e2e environments, the `self.env().caller()` function performs consistently and returns the same result. In the integration tests, the contract caller can be set using the `set_caller` function, after that we ensure that the actual contract was called with this id. In the end-to-end test, the Bob account is used to call the contract, which also returns the correct caller's account ID.

| \#  | Test                                                  | Integration | E2E |
| --- | ----------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the account id of the contract caller |     ✅      | ✅  |

## Result

This functionality is correctly implemented in both integration and end-to-end scenarios.
