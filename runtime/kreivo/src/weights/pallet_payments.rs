
//! Autogenerated weights for `pallet_payments`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `virto-builder`, CPU: `Intel(R) Xeon(R) Silver 4216 CPU @ 2.10GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("kreivo-local")`, DB CACHE: 1024

// Executed Command:
// ./target/release/virto-node
// benchmark
// pallet
// --chain
// kreivo-local
// --pallet
// pallet_payments
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/kreivo/src/weights/pallet_payments.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_payments`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_payments::WeightInfo for WeightInfo<T> {
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(983), added: 3458, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Payments::PaymentParties` (r:0 w:1)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `q` is `[1, 50]`.
	fn pay(_q: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `544`
		//  Estimated: `8518`
		// Minimum execution time: 159_701_000 picoseconds.
		Weight::from_parts(253_581_329, 0)
			.saturating_add(Weight::from_parts(0, 8518))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:3 w:3)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(983), added: 3458, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn release() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1153`
		//  Estimated: `8859`
		// Minimum execution time: 414_864_000 picoseconds.
		Weight::from_parts(429_418_000, 0)
			.saturating_add(Weight::from_parts(0, 8859))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:1)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(983), added: 3458, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1120`
		//  Estimated: `8518`
		// Minimum execution time: 309_509_000 picoseconds.
		Weight::from_parts(333_648_000, 0)
			.saturating_add(Weight::from_parts(0, 8518))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn request_refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377`
		//  Estimated: `159279`
		// Minimum execution time: 57_498_000 picoseconds.
		Weight::from_parts(71_539_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:0)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:1 w:1)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(983), added: 3458, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn dispute_refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1221`
		//  Estimated: `159279`
		// Minimum execution time: 216_107_000 picoseconds.
		Weight::from_parts(236_476_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:0)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:3 w:3)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Holds` (r:2 w:2)
	/// Proof: `Assets::Holds` (`max_values`: None, `max_size`: Some(983), added: 3458, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn resolve_dispute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1223`
		//  Estimated: `8859`
		// Minimum execution time: 573_042_000 picoseconds.
		Weight::from_parts(596_747_000, 0)
			.saturating_add(Weight::from_parts(0, 8859))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Payments::PaymentParties` (r:0 w:1)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn request_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391`
		//  Estimated: `8518`
		// Minimum execution time: 56_938_000 picoseconds.
		Weight::from_parts(58_726_000, 0)
			.saturating_add(Weight::from_parts(0, 8518))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Payments::PaymentParties` (r:1 w:0)
	/// Proof: `Payments::PaymentParties` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Payments::Payment` (r:1 w:1)
	/// Proof: `Payments::Payment` (`max_values`: None, `max_size`: Some(5053), added: 7528, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:3 w:3)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn accept_and_pay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1014`
		//  Estimated: `8859`
		// Minimum execution time: 356_092_000 picoseconds.
		Weight::from_parts(361_268_000, 0)
			.saturating_add(Weight::from_parts(0, 8859))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
