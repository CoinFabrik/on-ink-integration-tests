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

Note: The following estimate applies both to code_hash() and to own_code_hash(), as the two functions are very similar.

This function requires accounts and contract instantiation to be implemented. After that's done, there are two main methods to implement this functionality:
1. Hash some property of the specified contract. For example, the address of the constructor.
2. Implement the code hash equivalently to how it's done on E2E.

Option #1 is fairly easy to implement; it can be done in less than a week. However, it can only support comparing hashes returned by either own_code_hash() or code_hash(), not if the caller specifies a hash directly. E.g.
```
if code_hash(foo) == "0xf00..."{
}
```
Option #2 supports both use cases, but it's more complex to implement and also slows down integration tests by requiring the full WASM compilation of the contract.

## Update on Correcting this Issue

We implemented the function in [PR #1988](https://github.com/paritytech/ink/pull/1988) so that it returns a value that is unique for the contract, but it does not emulate what would be returned on-chain. This value is a hash of a pointer to a function generated for each contract by the compiler. Therefore, this value is unique for each contract, which is the intended purpose of this function.
