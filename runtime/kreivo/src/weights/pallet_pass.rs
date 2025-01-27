
//! Autogenerated weights for `pallet_pass`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 45.0.0
//! DATE: 2025-01-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `virto-us3`, CPU: `Intel(R) Xeon(R) Silver 4216 CPU @ 2.10GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// /home/devops/.cargo/bin/frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime
// target/release/wbuild/kreivo-runtime/kreivo_runtime.compact.compressed.wasm
// --pallet
// pallet_pass
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/kreivo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_pass`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pass::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Pass::Devices` (r:0 w:1)
	/// Proof: `Pass::Devices` (`max_values`: None, `max_size`: Some(220), added: 2695, mode: `MaxEncodedLen`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3593`
		// Minimum execution time: 41_325_000 picoseconds.
		Weight::from_parts(53_969_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
