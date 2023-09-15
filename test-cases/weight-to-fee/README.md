# Function `weight_to_fee`

```rust
pub fn weight_to_fee<E>(gas: Gas) -> E::Balance
```

## Description

Returns the price for the specified amount of gas.

Usage example approach proposed in `env_access.rs`:

```rust
#[ink::contract]
pub mod my_contract {
    #[ink(storage)]
    pub struct MyContract {}

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Returns a tuple of
        /// - the result of adding the `rhs` to the `lhs`
        /// - the gas costs of this addition operation
        /// - the price for the gas
        #[ink(message)]
        pub fn addition_gas_cost(&self, rhs: i32, lhs: i32) -> (i32, u64, Balance) {
            let before = self.env().gas_left();
            let result = rhs + lhs;
            let after = self.env().gas_left();
            let gas_used = after - before;
            let gas_cost = self.env().weight_to_fee(gas_used);
            (result, gas_used, gas_cost)
        }
    }
}
```

## Related ink! functions

- [`weight_to_fee`](https://docs.rs/ink_env/4.3.0/ink_env/fn.weight_to_fee.html)

## Test case

The chosen approach is even more basic that the exposed in the Description. There's a fixed amount of gas for both test environments, given by `_GAS_AMOUNT`. Test case just verifies that a gas price for that arbitrary amount is returned by [`weight_to_fee`](https://docs.rs/ink_env/4.3.0/ink_env/fn.weight_to_fee.html).

| \#  | Test                                                    | Integration | E2E |
| --- | ------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the price for a specified amount of gas |             |     |

TODO: gas price per unit should be the same in both test envs? How is gas price per unit calculated? Having that solved, an extra assert should be added, something like:

```rust
assert_eq!(EXPECTED_PRICE_PER_UNIT, gas_price / <u64 as Into<u128>>::into(gas))
```

This can even be a separate test case \#2 "Price per unit is correct" or so.

## Comparison Integration vs E2E

Both Integration and E2E environments return a valid gas price (`u128`) for a given gas amount. However, price per gas unit differs in both envs, 100 in Integration and 0 in E2E.

TODO: Is that the expected behaviour?

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
