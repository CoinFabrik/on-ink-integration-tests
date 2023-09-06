# Function `code_hash`

```rust
pub fn code_hash<E>(account: &E::AccountId) -> Result<E::Hash>
where
    E: Environment,
```

## Description

`code_hash´ retrieves the code hash of the contract at the specified account id.

## Related ink! functions

- [`code_hash`](https://paritytech.github.io/ink/ink_env/fn.code_hash.html)
- [`own_code_hash`](https://paritytech.github.io/ink/ink_env/fn.own_code_hash.html)

## Test case

The contract has a function that returns its hash through the `code_hash` function. In order to test it, the hash of the same contract is obtained through another function called `own_code_hash`.
Once the hash is obtained from two different ways, it is verified that they are the same.

## Comparison Integration vs E2E

The End-to-End test works correctly since it invokes successfully the call. In Integration did not work since it's [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L528C5-L533C5).

Integration:

- Test 1: Attemps to invoke a call to the `account_id` of the contract and panics due to lack of implementation.
- Test 2: attemps to invoke a call to a mocked `account_id` and panics due to lack of implementation.

End-to-End:

- Test 1: Attemps to invoke a call to the `account_id` of the contract and checks that the result obtained is correct by comparing it with the hash returned in the `own_code_hash` function.
- Test 2: attemps to invoke a call to a other contract and checks that the result obtained is correct by comparing it with the hash returned in the `own_code_hash` function.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
