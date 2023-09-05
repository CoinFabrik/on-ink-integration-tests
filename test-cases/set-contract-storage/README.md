# Template

## Description

Tests that the Ink! integration tests correctly implement set_contract_storage() checks. In particular, that they reject attempts to set data that the E2E would reject.

## Related ink! functions

- set_contract_storage()

## Test case

set_contract_storage() allows storing values of up to 16 KiB (2^14 bytes) per key. With an additional 32-bit 'length' datum required for variable-length values, this allows storing an array of 2^14 - 4 bytes.

* Test 1 attempts to store an array of 2^14 - 4 bytes in integration.
* Test 2 attempts to store an array of 2^14 - 3 bytes in integration. This should always fail.
* Test 3 attempts to store an array of 2^14 - 4 bytes in E2E.
* Test 4 attempts to store an array of 2^14 - 3 bytes in E2E. This should always fail.

## Comparison Integration vs E2E

Test 4 passes (correctly failing to store the array), but test 2 failed (incorrectly succeding to store the array). This is due to the emulated set_contract_storage() implementation not checking the size of the input array.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
