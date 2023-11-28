# Function `gas_left`

```rust
pub fn gas_left<E>() -> Gas
```

## Description

Returns the amount of gas left for the contract execution.

## Related ink! functions

- [`gas_left`](https://paritytech.github.io/ink/ink_env/fn.gas_left.html)

## Test case

The contract has a single function that returns remaining gas:

```rust
#[ink(message)]
pub fn get_gas_left(&self) -> u64 {
    self.env().gas_left()
}
```

We thought on the the simplest code to enable the execution of `gas_left` function in different testing environments. We condider a case successfull if returned gas left value is a no-null integer.

| \#  | Test                                                           | Integration | E2E |
| --- | -------------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts get the amount of gas left for the contract execution |     ❌      | ✅  |

Further testing can be performed in order to check gas calculation and address edge cases. However, for the current scope of this comparison instance, these aspects are not within consideration.

## Comparison Integration vs E2E

`gas_left` worked correctly in E2E and did not on Integration since it is [unimplemented](https://github.com/paritytech/ink/blob/c2af39883aab48c71dc09dac5d06583f2e84dc54/crates/engine/src/ext.rs#L405).

## Result

Because integration tests are performed on native code rather than WASM code, and because gas cost is based on number of WASM instructions executed, implementing this function is impractically complex.

