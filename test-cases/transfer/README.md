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

- Integration Tests: These tests focus on the contract's internal logic, verifying that the `transfer` function behaves as expected under various conditions.

- E2E Tests: In end-to-end tests, the exact balance of the smart contract after the `transfer` is known. However, when a user or another contract calls the `transfer` function, they pay a gas fee. The exact gas required isn't known in advance. For this reason, we initiate a transaction with a value of `1` and then assert that the resulting balance is less than the initial balance due to the gas fee.

| \#  | Test                                  | Integration | E2E |
| --- | ------------------------------------- | :---------: | :-: |
| 1   | Transfers funds to the caller account |     ✅      | ✅  |

## Result

This functionality works as expected.
