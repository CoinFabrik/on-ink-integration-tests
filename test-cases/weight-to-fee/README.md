# Function `weight_to_fee`

```rust
pub fn weight_to_fee<E>(gas: Gas) -> E::Balance
```

## Description

Returns the price for the specified amount of gas.

## Related ink! functions

- [`weight_to_fee`](https://docs.rs/ink_env/4.3.0/ink_env/fn.weight_to_fee.html)

## Test case

The test case calls the function `weight_to_fee()` of the contract.

```rust
        #[ink(message)]
        pub fn weight_to_fee(&self, gas: u64) -> Balance {
            self.env().weight_to_fee(gas)
        }
```

The test case compairs against different values checking the returned values in integration and e2e tests.

| \#  | Test                                                        | Integration | E2E |
| --- | -------------------------------------------------------     | :---------: | :-: |
| 1   | Receives an amount of gas of 42 and expects a total gas fee of 42, i.e: fee == 1.                  |      ❌      | ❌    |
| 2   | Receives an amount of gas of 42 and expects a total gas fee of 4200, i.e: fee == 100.              |       ✅      |  ❌    |


## Comparison Integration vs E2E

Both Integration and E2E environments return a valid gas price (`u128`) for a given gas amount. However, price per gas unit differs in both envs, 100 in Integration and 0 always in E2E.


## Result

We reviewed that the function `weight_to_fee()` has multiple implementations. It is complex to determine which one of them is resposible for E2E tests because the large size of the runtime makes debugging very slow.

This incorrect behaviour of `weight_to_fee()` in E2E tests, is also observed in [`paritytech/substrate-contracts-node`](https://github.com/paritytech/substrate-contracts-node).

