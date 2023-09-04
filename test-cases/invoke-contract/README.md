# Function `invoke_contract`

```rust
pub fn invoke_contract<E, Args, R>(
    params: &CallParams<E, Call<E>, Args, R>,
) -> Result<ink_primitives::MessageResult<R>>
where
    E: Environment,
    Args: scale::Encode,
    R: scale::Decode,
```

## Description

`invoke_contract` is a low-level way to evaluate another smart contract. Invokes a contract message and returns its result.

## Related ink! functions

- [`invoke_contract`](https://paritytech.github.io/ink/ink_env/fn.invoke_contract.html)

## Test case

The e2e test works correctly since it delegates the call to the second contract. It also fails when the account id of the contract to which the call is delegated is invalid.

* Test 1: attemps to delegate the call to an invalid account id in E2E.
* Test 2: attemps to delegate the call to a valid contract and check that the result is correct in E2E.


## Comparison Integration vs E2E

Test case worked in End-to-End but did not in Integration since it's [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L432)

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
