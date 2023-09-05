# Function `set_contract_storage`

```rust
pub fn set_contract_storage<K, V>(key: &K, value: &V) -> Option<u32>
```

## Description

Writes the `value` to the contract storage under the given storage key and returns the size of pre-existing value if any.

`set_contract_storage` allows storing values of up to 16 KiB (2^14 bytes) per key. With an additional 32-bit 'length' datum required for variable-length values, this allows storing an array of 2^14 - 4 bytes.

## Related ink! functions

- [`set_contract_storage`](https://paritytech.github.io/ink/ink_env/fn.set_contract_storage.html)

## Test case

Tests that the Ink! Integration and End-to-End tests correctly implement `set_contract_storage` checks. In particular, that Integration reject attempts to set data that the E2E would reject (more than maximum size).

| \#  | Test                                                                   | Integration | E2E |
| --- | ---------------------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to store an array of 2^14 - 4 bytes.                          |     ✅      | ✅  |
| 2   | Attempts to store an array of 2^14 - 3 bytes. This should always fail. |     ❌      | ✅  |

## Comparison Integration vs E2E

Test 2 passes on E2E (correctly failing to store the array), but failed on Integration (incorrectly succeding to store the array). This is due to the emulated `set_contract_storage` implementation not checking the size of the input array.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
