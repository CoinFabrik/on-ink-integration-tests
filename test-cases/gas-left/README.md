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

E2E implementation:

```rust
//https://github.com/paritytech/substrate.git:28e906dffcaa91e85f59aff628d953ebeb036ae2
//frame/contracts/src/wasm/runtime.rs:1968
/// Stores the weight left into the supplied buffer.
///
/// Equivalent to the newer [`seal1`][`super::api_doc::Version2::gas_left`] version but
/// works with *ref_time* Weight only. It is recommended to switch to the latest version, once
/// it's stabilized.
#[prefixed_alias]
fn gas_left(ctx: _, memory: _, out_ptr: u32, out_len_ptr: u32) -> Result<(), TrapReason> {
	/*
	 * Charges to ctx the appropriate cost for calling this function. See:
	 *  * frame/contracts/src/wasm/runtime.rs:366-375
	 *  * frame/contracts/src/wasm/runtime.rs:289
	 *  * frame/contracts/src/schedule.rs:216
	 *  * frame/contracts/src/schedule.rs:367-371
	 *  * frame/contracts/src/schedule.rs:355-359
	 *  * frame/contracts/src/weights.rs:688-699
	 */
	ctx.charge_gas(RuntimeCosts::GasLeft)?;
	/*
	 * Gets the actual gas left value from ctx. See:
	 *  * frame/contracts/src/exec.rs:1436-1438
	 *  * frame/contracts/src/exec.rs:609-613
	 *  * frame/contracts/src/exec.rs:447
	 *  * frame/contracts/src/lib.rs:269
	 *  * frame/contracts/src/exec.rs:480
	 */
	let gas_left = &ctx.ext.gas_meter().gas_left().ref_time().encode();
	/*
	 * Writes the value to the output pointer.  See
	 *  * frame/contracts/src/wasm/runtime.rs:638-683
	 *    Note that it would appear that this function does not charge the
	 *    context. The most relevant call in this function appears to be
	 *    write_sandbox_memory(), defined in
	 *  * frame/contracts/src/wasm/runtime.rs:685-701
	 */
	Ok(ctx.write_sandbox_output(
		memory,
		out_ptr,
		out_len_ptr,
		gas_left,
		false,
		already_charged,
	)?)
}
```

The main implementation cost on integration is a missing stack structure to keep track of the {functions|contracts} (unsure which one, probably the latter) that have been executed in the current context. This stack structure also keeps track of the remaining gas for the current call. Other than that, the function merely updates a value on that stack and then serializes an integer into a buffer.
As a rough estimate. A programmer already familiar with the Substrate codebase could implement the feature in 1-2 days. One who's unfamiliar could take 5-8 days.
