# Function `terminate_contract`

```rust
pub fn terminate_contract<E>(beneficiary: E::AccountId) -> !
```

## Description

The `terminate_contract` function is designed to end the existence of the currently executed smart contract. By invoking this method, the calling account is removed, and all its remaining balance is transferred to the given beneficiary.
This function never returns. Either the termination was successful, and the execution of the destroyed contract is halted, or it failed during the termination. A failure during termination is considered fatal and results in a trap and rollback.

## Related ink! functions

- [`terminate_contract`](https://paritytech.github.io/ink/ink_env/fn.terminate_contract.html)

## Test case

The integration test and end-to-end test both verify the termination functionality of the contract. Both tests ensure that after the `terminate_contract` function is called, the contract's balance is transferred to the caller and the contract's execution is terminated. Both tests passed successfully, demonstrating the correct functionality of the `terminate_contract` method.

## Comparison Integration vs E2E

Both the integration test and the end-to-end test for the terminate method passed successfully. However, it's worth noting that during the execution of the integration test, a Rust panic is observed:

```rust
thread 'terminate_contract::tests::terminate_works' panicked at 'Box<dyn Any>'
```

This panic is expected behaviour and enables both testing for the proper result and makes sure this method returns `Never`.

| \#  | Test                                         | Integration | E2E |
| --- | -------------------------------------------- | :---------: | :-: |
| 1   | Attempts to terminate the executing contract |     ✅      | ✅  |

## Result

- This functionality is implemented in both environments and works as expected.
