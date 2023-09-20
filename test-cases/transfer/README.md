# Function `transfer`

```rust
pub fn transfer<E>(destination: E::AccountId, value: E::Balance) -> Result<()>
```

## Description

The `transfer` function serves as the core of this contract. When invoked, it initiates a balance transfer of the specified `value` from the contract's balance to the `destination` account. This function is especially useful in scenarios where users or other contracts need to transfer funds.

## Related ink! functions

- [`transfer`](https://paritytech.github.io/ink/ink_env/fn.transfer.html)

## Test case

The contract's functionality can be demonstrated with a simple test:

```rust
let contract = Transfer::new();
contract.transfer(1000);
```

This test showcases the process of instantiating the contract and requesting a balance transfer. In real-world scenarios, the results would also account for transaction fees and other network considerations.

## Comparison Integration vs E2E

Ensuring the reliability and security of balance transfers is crucial. As such, rigorous testing is performed on both integration and end-to-end levels:

- Integration Tests: In the integration test `transfer_works`, a testing environment is set up with a specific account balance. The `transfer` function is then invoked to transfer a specified value from the contract's balance to the caller. After the transfer, the test asserts that the balances of the accounts have changed as expected.

- E2E Tests: In the end-to-end test `transfer_works`, the contract is set up with a given balance. The `transfer` function is then called to transfer a value of `1` to the caller. The test then verifies that the contract's balance has decreased by the transferred amount.

| \#  | Test                                  | Integration | E2E |
| --- | ------------------------------------- | :---------: | :-: |
| 1   | Transfers funds to the caller account |     ✅      | ✅  |

## Result

This functionality works as expected.
