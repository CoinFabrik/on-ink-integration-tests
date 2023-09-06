# Function `invoke_contract`

```rust
pub fn invoke_contract<E, Args, R>(
    params: &CallParams<E, Call<E>, Args, R>,
) -> Result<ink_primitives::MessageResult<R>>
```

## Description

`invoke_contract` is a low-level way to evaluate another smart contract. Invokes a contract message and returns its result.

## Related ink! functions

- [`invoke_contract`](https://paritytech.github.io/ink/ink_env/fn.invoke_contract.html)
- [`invoke`](https://paritytech.github.io/ink/src/ink_env/call/call_builder.rs.html#679-681)

## Test case

The original contract initiates a call to another pre-existing contract, denoted as `contract_to_call`. It invokes a straightforward function (`split_profit`) with hardcoded arguments, ensuring determinism in the results for the purpose of specifically test the contract invocation. In End-to-End testing phase, it was also verified that it fails when the `account_id` of the contract to which the call is delegated is invalid.

These tests use the low-level function [`invoke_contract`](https://paritytech.github.io/ink/ink_env/fn.invoke_contract.html) but in the documentation it is recommended to use the [`invoke`](https://paritytech.github.io/ink/src/ink_env/call/call_builder.rs.html#679-681) function which has a type safe approach. This approach should be taken into account for testing beyond this initial comparison instance.

## Comparison Integration vs E2E

The End-to-End test works correctly since it invokes successfully the call to the second contract. In Integration did not work since it's [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L432).

Integration:

- Test 1: attemps to invoke a call to a mocked `account_id` and panics due to lack of implementation.

End-to-End:

- Test 1: attemps to invoke a call to an invalid `account_id`.
- Test 2: attemps to invoke a call to a valid contract and checks that the result obtained is correct.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
