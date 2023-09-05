# Function `invoke_contract`

### Off-chain:

```rust
fn invoke_contract_delegate<E, Args, R>(
    &mut self,
    params: &CallParams<E, DelegateCall<E>, Args, R>,
) -> Result<ink_primitives::MessageResult<R>>
```

### On-chain:

```rust
fn delegate_call(
		ctx: _,
		memory: _,
		flags: u32,
		code_hash_ptr: u32,
		input_data_ptr: u32,
		input_data_len: u32,
		output_ptr: u32,
		output_len_ptr: u32,
	) -> Result<ReturnCode, TrapReason>
```

## Description

The `invoke_contract_delegate` function provides a low-level mechanism to evaluate another smart contract via a delegate call. It invokes a contract message through a delegate call and subsequently returns its result. Notably, the delegated message is executed as if it were a part of the contract initiating the delegation.

It's imperative to understand that if the delegate call interacts with the storage, both the initiating and the called contracts should possess the same storage values being accessed, ensuring they are synchronized and match in structure and semantics.

## Related ink! functions

- [`invoke_contract_delegate`](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L449)
- [`delegate_call`](https://github.com/paritytech/substrate/blob/28e906dffcaa91e85f59aff628d953ebeb036ae2/frame/contracts/src/wasm/runtime.rs#L1467)

## Test case

The primary contract initiates a delegate call to a secondary, pre-existing contract, referred to as `contract-to-call`. This action invokes a simple function, `get_value`, that utilizes the storage struct provided by ink!.

## Comparison Integration vs E2E

While the end-to-end test operates correctly by successfully delegating the call to the secondary contract, the integration test falters. The failure of the integration test arises from yet-to-be-implemented functionalities, such as contract delegation and code hash operations.

- Integration: The test failed due to unimplemented logic. Panic error: `not implemented: off-chain environment does not support delegated contract invocation`.

- End-to-end: The test was successfully executed.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
