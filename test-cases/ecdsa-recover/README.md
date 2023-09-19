# Function `ecdsa_recover`

```rust
pub fn ecdsa_recover(
    signature: &[u8; 65],
    message_hash: &[u8; 32],
    output: &mut [u8; 33]
) -> Result<()>
```

## Description

The `ecdsa_recover` module provides functionality to recover the compressed ECDSA public key from a given signature and message hash. The method stores the result in the provided output.

## Related ink! functions

- [`ecdsa_recover`](https://paritytech.github.io/ink/ink_env/fn.ecdsa_recover.html)

## Test case

Both integration and end-to-end tests initialize the `ECDSARecover` contract. They use the `ecdsa_recover` method to compute the compressed ECDSA public key from a given signature and message hash, and then verify the result against an expected value.

## Comparison Integration vs E2E

This functionality works as expected in both scenarios.

| \#  | Test                                                  | Integration | E2E |
| --- | ----------------------------------------------------- | :---------: | :-: |
| 1   | Computes and verifies the compressed ECDSA public key |     ✅      | ✅  |

## Result

- This functionality is implemented and works as expected in both end-to-end and integration environments.
