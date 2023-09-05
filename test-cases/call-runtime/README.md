# Template

## Description

Tests that the Ink! integration tests correctly implement the call_runtime() mechanism.

## Related ink! functions

- call_runtime()

## Test case

Using https://github.com/paritytech/ink-examples/tree/main/call-runtime as a basis, the test attempts to make a transfer from the contract account to another account using the native blockchain runtime.

* Test 1 attempts to perform the transfer in integration.
* Test 2 attempts to perform the transfer in E2E.

## Comparison Integration vs E2E

Test 1 currently fails because call_runtime() is not implemented and panics when called. Test 2 works as expected.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
