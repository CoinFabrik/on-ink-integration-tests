# Report: `weight_to_fee()`

On our research into integration and end-to-end function implementation differences, we stumbled upon `weight_to_fee()` not working as expected.

Adding to the [test-case and documentation](https://github.com/CoinFabrik/on-ink-integration-tests/tree/main/test-cases/weight-to-fee) we built for analyzing this issue, we explain below the problem, we describe the work we have undertaken to try to identify its source, and propose next steps for resolving it.

## The problem

Given a basic contract with this message

```rust
#[ink(message)]
pub fn weight_to_fee(&self, gas: u64) -> Balance {
	self.env().weight_to_fee(gas)
}
```

When we call the function weight_to_fee() from an integration test we see that the fee for 1 unit of gas is 100.

```rust
#[ink::test]
fn integration_weight_to_fee_value() {
	// Given
	let contract = WeightToFee::new();
	// When
	let fee = contract.weight_to_fee(1);
	// Then
	assert_eq!(fee, 100); // True
}
```

When we call the function weight_to_fee() from an end-to-end test, we see that the fee for 1 unit of gas is 0.

```rust
#[ink_e2e::test]

async fn end_to_end_weigth_to_fee_1(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
	// Given
	let constructor = WeightToFeeRef::new();
	let contract_acc_id = client	
		.instantiate("weight-to-fee", &ink_e2e::alice(), constructor, 0, None)
		.await
		.expect("instantiate failed")
		.account_id;
	
	// When
	let fee = client
		.call(
			&ink_e2e::bob(),
			build_message::<WeightToFeeRef>(contract_acc_id)
			.call(|contract| contract.weight_to_fee(42)), // changing this number doesn't changes the outcome
			0,
			None,
		)
		.await
		.expect("weight-to-fee failed")
		.return_value();
		  
	// Then
	assert_eq!(fee, 0); // True	
	
	Ok(())
}
```

This behavior is seen in the local development node too, but not in testnets like Aleph Zero testnet. 

Looking at the runtime in `polkadot-sdk`, we found a few implementations of the functions, but modifying them didn't change the outcome of the tests as predicted. 
The functions that we modified to check if something changed were:

1)  `polkadot-sdk/substrate/frame/contracts/src/exec.rs`, the `get_weight_price()` function. 
2) `polkadot-sdk/substrate/frame/transaction-payment/src/lib.rs`, the `convert()` function.

3) `polkadot-sdk/substrate/frame/system/src/limits.rs`, the `max_block: Weight::zero()` inside the `builder()` function seems to affect at least the`polkadot-sdk/substrate/frame/transaction-payment/src/lib.rs` function `weight_to_fee()`:
```rust
	/// Compute the unadjusted portion of the weight fee by invoking the configured `WeightToFee`
	/// impl. Note that the input `weight` is capped by the maximum block weight before computation.
	pub fn weight_to_fee(weight: Weight) -> BalanceOf<T> {
		// cap the weight to the maximum defined in runtime, otherwise it will be the
		// `Bounded` maximum of its data type, which is not desired.
		let capped_weight = weight.min(T::BlockWeights::get().max_block);
		T::WeightToFee::weight_to_fee(&capped_weight)
	}
``` 

This seems to be always zero, as the `max_block` field is zero.

## Observations

In `polkadot-sdk/substrate/frame/contracts/src/wasm/runtime.rs`, there are 2 functions:
```rust
	fn weight_to_fee(
		ctx: _,
		memory: _,
		gas: u64,
		out_ptr: u32,
		out_len_ptr: u32,
	) -> Result<(), TrapReason> {
		let gas = Weight::from_parts(gas, 0);
		ctx.charge_gas(RuntimeCosts::WeightToFee)?;
		Ok(ctx.write_sandbox_output(
			memory,
			out_ptr,
			out_len_ptr,
			&ctx.ext.get_weight_price(gas).encode(),
			false,
			already_charged,
		)?)
	}
//**---**//
	#[unstable]
	fn weight_to_fee(
		ctx: _,
		memory: _,
		ref_time_limit: u64,
		proof_size_limit: u64,
		out_ptr: u32,
		out_len_ptr: u32,
	) -> Result<(), TrapReason> {
		let weight = Weight::from_parts(ref_time_limit, proof_size_limit);
		ctx.charge_gas(RuntimeCosts::WeightToFee)?;
		Ok(ctx.write_sandbox_output(
			memory,
			out_ptr,
			out_len_ptr,
			&ctx.ext.get_weight_price(weight).encode(),
			false,
			already_charged,
		)?)
	}
```
Where the first one has a hardcoded zero. Is that intentional as "working as intended", or is that a workaround for something else?

## Next Steps

- Make the gas value in e2e the same as in integration tests
- Make a setter like `env().setGasPrice(...)` to set the value in the tests (with a default value)
- Make the gas value an arbitrary X and document it.

Any of this decisions should be properly documented. 
