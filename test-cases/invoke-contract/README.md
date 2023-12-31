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

The function depends on features also required by gas_left(), instantiate_contract(); namely, storage and retrieval of secondary contracts, and gas calculations. The logic is fairly complex, so it's difficult to provide a time estimation. As a very rough estimate, it could be around 1-2 weeks worth of work.
On-chain implementation at frame/contracts/src/exec.rs:1199-1243
On-chain implementation at frame/contracts/src/exec.rs:885-1014


## Update on Correcting this Issue

Functions `invoke_contract()` and `invoke_contract_delegate()` have almost identical implementations in our proposed [PR #1988](https://github.com/paritytech/ink/pull/1988).

After getting their arguments from the `CallParams` object they call a new internal function `invoke_contract_impl()` which handles the invocation logic. Of note is that the actual call into the generated dispatch function is done by `execute_contract_call()`. This function is only instantiated by rustc per-contract when a test calls `ink::env::test::upload_code()`. Function `upload_code()` adds a reference to `execute_contract_call()` to the environment’s database. Function `invoke_contract_impl()` fetches the reference and calls it. Before and after this call it takes care of execution context bookkeeping.
