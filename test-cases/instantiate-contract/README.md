# Template

```rust
pub fn instantiate_contract<E, ContractRef, Args, Salt, R>(
    params: &CreateParams<E, ContractRef, Args, Salt, R>
) -> Result<ConstructorResult<<R as ConstructorReturnType<ContractRef>>::Output>>
where
    E: Environment,
    ContractRef: FromAccountId<E>,
    Args: Encode,
    Salt: AsRef<[u8]>,
    R: ConstructorReturnType<ContractRef>,
```

## Description

`instantiate_contract` is a low level way to instantiate another smart contract.

## Related ink! functions

- [instantiate_contract](https://paritytech.github.io/ink/ink_env/fn.instantiate_contract.html)

## Test case



## Comparison Integration vs E2E

The End-to-End test works correctly since it invokes successfully the call to the second contract. In Integration did not work since it's [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L464).

| \#  | Test                                                            | Integration | E2E |
| --- | --------------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to instantiate a contract.                             |     ❌      | ✅  |

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
