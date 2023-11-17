# Function `own_code_hash`

```rust
pub fn own_code_hash<E>() -> Result<E::Hash>
```

## Description

Retrieves the code hash of the currently executing contract.

## Related ink! functions

- [`own_code_hash`](https://paritytech.github.io/ink/ink_env/fn.own_code_hash.html)

## Test case

Test case intend to showcase how `own_code_hash` is working in both Integration and End-to-End scenarios. To check the retrieved code hash is correct, we compared it with the code obtained via [`code_hash`](https://paritytech.github.io/ink/ink_env/fn.code_hash.html) by providing the contract `account_id`.

Note in Integration this last step it is not possible due to lack of [`code_hash`](https://paritytech.github.io/ink/ink_env/fn.code_hash.html) implementation, so it is just tested that [`own_code_hash`](https://paritytech.github.io/ink/ink_env/fn.own_code_hash.html) can be called.

| \#  | Test                                                    | Integration | E2E |
| --- | ------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the code hash of the executing contract |     ❌      | ✅  |

## Comparison Integration vs E2E

Test 1 worked as expected in End-to-End but did not on Integration since it's [not implemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L535).

## Result

See estimate for code_hash().

## Update on Correcting this Issue

Our implementation in [PR #1988](https://github.com/paritytech/ink/pull/1988) returns the code hash of the account of the calling contract. It simply fetches the code hash stored in the database by `instantiate_contract()` or set by `set_code_hash()`.
