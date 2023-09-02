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

Minimum description of tested functionality.

## Related ink! functions

- [`invoke_contract`](https://paritytech.github.io/ink/ink_env/fn.invoke_contract.html)

## Test case

Both test cases implement the contract logic and attempt to use the `invoke_contract` functions.

## Comparison Integration vs E2E

Test case worked in End-to-End but did not in Integration since it's [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L432)

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
