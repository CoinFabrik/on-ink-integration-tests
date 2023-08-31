# Function `set_code_hash`

```rust
pub fn set_code_hash(code_hash: &[u8; 32]) -> Result<()>
```

## Description

`set_code_hash` replaces the contract code at the specified address with new code. This test is intended to showcase the functionality of the `set_code_hash` function, which changes contract's `code_hash` enabling adjustments to the contract's logic or behavior after deployment. Allowing modifications to the `code_hash` can yield substantial consequences, which are further detailed in the article [Unprotected Set Code Hash](https://coinfabrik.github.io/scout/docs/vulnerabilities/unprotected-set-code-hash).

## Related ink! functions

- [`set_code_hash`](https://paritytech.github.io/ink/ink_env/fn.set_code_hash.html)
- [`set_code_hash2`](https://paritytech.github.io/ink/ink_env/fn.set_code_hash2.html), new version of the existing `set_code_hash` but takes type `&E::Hash` as parameter instead of `&[u8; 32]`.
- [`code_hash`](https://paritytech.github.io/ink/ink_env/fn.code_hash.html), gets the current `code_hash` of the contract.

## Test case

The test case presents a scenario in which the `set_code_hash` function is implemented within a vulnerable setup (lacking caller verification). Thus, our goal is to substitute this with a secure implementation. Both test cases attempt to use the `set_code_hash` function.

## Comparison Integration vs E2E

Test case worked in End-to-End but did not in Integration since it's [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/env/src/engine/off_chain/impls.rs#L361). In the latter context, we proposed an arbitrary `code_hash` since there's no other contract/`code_hash` in the mocked environment to be replaced by.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
