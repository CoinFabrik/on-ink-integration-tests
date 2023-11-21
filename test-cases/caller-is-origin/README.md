# Function `caller_is_origin`

```rust
pub fn caller_is_origin<E>() -> bool
```

## Description

`caller_is_origin` returns true if the caller is the original contract that was called by the extrinsic, and false otherwise. For example, if contract A invokes contract B and B calls `caller_is_origin`, `caller_is_origin` will return false in that context.

## Related ink! functions

- [`caller_is_origin`](https://paritytech.github.io/ink/ink_env/fn.caller_is_origin.html)

## Test case

Tests that the Ink! Integration tests correctly implement `caller_is_origin` checks.

| \#  | Test                                                                                                                       | Integration | E2E |
| --- | -------------------------------------------------------------------------------------------------------------------------- | :---------: | :-: |
| 1   | Calls a contract that calls `caller_is_origin`. `caller_is_origin` is expected to return true.                             |     ❌      | ✅  |
| 2   | Calls a contract that invokes another contract that calls `caller_is_origin`. `caller_is_origin` expected to return false. |     ❌      | ✅  |

Note: Currently Integration is missing critical functionality to properly write some of the corresponding tests. They will need to be updated once that functionality is put in place.

In particular, note that Test 2 in Integration is incompletely implemented, as Integration does not have the functionality required to deploy or invoke secondary contracts.

## Comparison Integration vs E2E

Tests 1 and 2 passed on End-to-End. On the other hand, did not on Integration.
In this scenario, Test 1 fails because the `caller_is_origin` implementation panics instead of returning any value. Test 2 fails because it's not yet possible to invoke contracts.

## Result

The function depends on features also required by [`gas_left`](https://paritytech.github.io/ink/ink_env/fn.gas_left.html) and [`invoke_contract`](https://paritytech.github.io/ink/ink_env/fn.invoke_contract.html); namely, storage and retrieval of secondary contracts, and gas calculations. Assuming that those features are already in place, the function is fairly trivial. It should not take more than a few minutes to implement.
On-chain implementation at `frame/contracts/src/exec.rs:1385`.

## Update on Correcting this Issue

In our proposed implementation in [PR #1991](https://github.com/paritytech/ink/pull/1991), we have added the `depth` field to the `ExecContext` struct to detect caller changes whenever calls are made between contracts.

Basically every time the callee changes in the `instantiate_contract()` or `invoke_contract()` function calls, 1 is added to `depth`. And when the callee returns to the previous one, 1 is subtracted.

The `caller_is_origin()` function compares the value of depth to zero to check if the current contract caller is the origin of the entire call stack.
