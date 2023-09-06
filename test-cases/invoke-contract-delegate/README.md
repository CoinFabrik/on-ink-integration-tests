# Function `invoke_contract`

```rust
pub fn invoke_contract_delegate<E, Args, R>(
    params: &CallParams<E, DelegateCall<E>, Args, R>
) -> Result<MessageResult<R>>
```

## Description

The `invoke_contract_delegate` function provides a low-level mechanism to evaluate another smart contract via a delegate call. It invokes a contract message through a delegate call and subsequently returns its result. Notably, the delegated message is executed as if it were a part of the contract initiating the delegation.

It's imperative to understand that if the delegate call interacts with the storage, both the initiating and the called contracts should possess the same storage values being accessed, ensuring they are synchronized and match in structure and semantics.

## Related ink! functions

- [`invoke_contract_delegate`](https://docs.rs/ink_env/latest/ink_env/fn.invoke_contract_delegate.html)

## Test case

The primary contract initiates a delegate call to a secondary, pre-existing contract, referred to as `contract-to-call`. This action invokes a simple function, `get_value`, that utilizes the storage struct provided by ink!.

## Comparison Integration vs E2E

While the end-to-end test operates correctly by successfully delegating the call to the secondary contract, the integration test falters. The failure of the integration test arises from [`yet-to-be-implemented`](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L449) functionalities, such as contract delegation and code hash operations.

| \#  | Test                                                    | Integration | E2E |
| --- | ------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the code hash of the executing contract |     ❌      | ✅  |

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
