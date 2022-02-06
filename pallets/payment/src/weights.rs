//! Autogenerated weights for `virto_payment`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-06, STEPS: `20`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/virto-parachain
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution
// compiled
// --extrinsic=*
// --pallet=virto-payment
// --steps=20
// --repeat=1
// --heap-pages=4096
// --output
// .

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn pay() -> Weight;
	fn pay_with_remark() -> Weight;
	fn release() -> Weight;
	fn cancel() -> Weight;
	fn resolve_cancel_payment() -> Weight;
	fn resolve_release_payment() -> Weight;
	fn request_refund() -> Weight;
	fn claim_refund() -> Weight;
}

/// Weight functions for `virto_payment`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay() -> Weight {
		(82_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay_with_remark() -> Weight {
		(76_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		(37_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		(52_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn resolve_cancel_payment() -> Weight {
		(51_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn resolve_release_payment() -> Weight {
		(40_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	fn request_refund() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn claim_refund() -> Weight {
		(49_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay() -> Weight {
		(80_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay_with_remark() -> Weight {
		(75_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		(42_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn resolve_cancel_payment() -> Weight {
		(52_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn resolve_release_payment() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	fn request_refund() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn claim_refund() -> Weight {
		(49_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
