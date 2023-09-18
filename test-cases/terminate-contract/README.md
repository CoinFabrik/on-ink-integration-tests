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

The integration test verifies the termination functionality of the contract. Based on the test results, it seems that there might be an issue with the integration of the terminate method, as it did not execute as expected during the test. The end-to-end test has passed successfully, indicating that the terminate method functions correctly in a full environment.

## Comparison Integration vs E2E

The integration test for the terminate method failed during execution, while the end-to-end test passed successfully.

| \#  | Test                                         | Integration | E2E |
| --- | -------------------------------------------- | :---------: | :-: |
| 1   | Attempts to terminate the executing contract |     ❌      | ✅  |

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
