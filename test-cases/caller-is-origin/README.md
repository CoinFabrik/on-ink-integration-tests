# Template

## Description

Tests that the Ink! integration tests correctly implement caller_is_origin() checks.
Note: Currently integration is missing critical functionality to properly write some of the corresponding tests. They will need to be updated once that functionality is put in place.

## Related ink! functions

- caller_is_origin()

## Test case

caller_is_origin() returns true if the caller is the original contract that was called by the extrinsic, and false otherwise. For example, if contract A invokes contract B and B calls caller_is_origin(), caller_is_origin() will return false in that context.

* (Integration) Test 1 calls a contract that calls caller_is_origin(). caller_is_origin() is expected to return true.
* (Integration) Test 2 calls a contract that invokes another contract that calls caller_is_origin(). caller_is_origin() expected to return false. Note that this test is incompletely implemented, as integration does not have the functionality required to deploy or invoke secondary contracts.
* (E2E) Test 3 calls a contract that calls caller_is_origin(). caller_is_origin() is expected to return true.
* (E2E) Test 4 calls a contract that invokes another contract that calls caller_is_origin(). caller_is_origin() expected to return false.

## Comparison Integration vs E2E

Test 1 fails because the caller_is_origin() implementation panics instead of returning any value.
Test 2 fails because it's not yet possible to invoke contracts.
Tests 3 and 4 pass.

## Result

The function depends on features also required by gas_left() and invoke_contract(); namely, storage and retrieval of secondary contracts, and gas calculations. Assuming that those features are already in place, the function is fairly trivial. It should not take more than a few minutes to implement.
On-chain implementation at frame/contracts/src/exec.rs:1385
