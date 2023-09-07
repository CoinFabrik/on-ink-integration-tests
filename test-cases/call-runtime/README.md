# Function `call_runtime`

```rust
pub fn call_runtime<E, Call>(call: &Call) -> Result<()>
```

## Description

Tries to trigger a runtime dispatchable, i.e. an extrinsic from a pallet. For more details consult [host function documentation](https://paritytech.github.io/substrate/master/pallet_contracts/api_doc/trait.Current.html#tymethod.call_runtime).

## Related ink! functions

- [`call_runtime`](https://paritytech.github.io/ink/ink_env/fn.call_runtime.html)

## Test case

Tests that the Integration tests correctly implement the `call_runtime` mechanism.

Using [https://github.com/paritytech/ink-examples/tree/main/call-runtime](https://github.com/paritytech/ink-examples/tree/main/call-runtime) as a basis, the test attempts to make a transfer from the contract account to another account using the native blockchain runtime.

| \#  | Test                             | Integration | E2E |
| --- | -------------------------------- | :---------: | :-: |
| 1   | Attempts to perform the transfer |     ❌      | ✅  |

Note: Although the test case is implemented, we couldn't figure out how to get the balance of a given account in Integration, so the test just tests that `call_runtime` can be called and returns successfully, without checking if the account states were changed in accordance with a balance transfer.

## Comparison Integration vs E2E

Test 1 worked as expected in End-to-End and in Integration currently fails because [`call_runtime`](https://paritytech.github.io/ink/ink_env/fn.call_runtime.html) is not implemented and panics when called.

## Result

Implementing this function is unfeasible, as it would require emulating practically the entire node to get consistent results. Testing a contract function that calls into the runtime should be done using E2E tests.
