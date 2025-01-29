
//! Autogenerated weights for `pallet_referenda`
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
// pallet_referenda
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

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: `KreivoReferenda::ReferendumCount` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:0 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `159279`
		// Minimum execution time: 134_650_000 picoseconds.
		Weight::from_parts(162_354_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365`
		//  Estimated: `317568`
		// Minimum execution time: 161_902_000 picoseconds.
		Weight::from_parts(185_978_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `880`
		//  Estimated: `159279`
		// Minimum execution time: 246_010_000 picoseconds.
		Weight::from_parts(331_043_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `921`
		//  Estimated: `159279`
		// Minimum execution time: 274_683_000 picoseconds.
		Weight::from_parts(322_476_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `317568`
		// Minimum execution time: 267_263_000 picoseconds.
		Weight::from_parts(396_333_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `507`
		//  Estimated: `317568`
		// Minimum execution time: 200_421_000 picoseconds.
		Weight::from_parts(210_350_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `4365`
		// Minimum execution time: 94_950_000 picoseconds.
		Weight::from_parts(103_943_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `4365`
		// Minimum execution time: 97_829_000 picoseconds.
		Weight::from_parts(108_379_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `317568`
		// Minimum execution time: 113_299_000 picoseconds.
		Weight::from_parts(128_067_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::MetadataOf` (r:1 w:0)
	/// Proof: `KreivoReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `617`
		//  Estimated: `317568`
		// Minimum execution time: 296_711_000 picoseconds.
		Weight::from_parts(313_813_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:0)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `102`
		//  Estimated: `3556`
		// Minimum execution time: 38_397_000 picoseconds.
		Weight::from_parts(47_986_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1021`
		//  Estimated: `159279`
		// Minimum execution time: 218_476_000 picoseconds.
		Weight::from_parts(259_440_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1021`
		//  Estimated: `159279`
		// Minimum execution time: 175_973_000 picoseconds.
		Weight::from_parts(311_365_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `652`
		//  Estimated: `4365`
		// Minimum execution time: 80_669_000 picoseconds.
		Weight::from_parts(119_408_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `619`
		//  Estimated: `4365`
		// Minimum execution time: 86_824_000 picoseconds.
		Weight::from_parts(116_099_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `635`
		//  Estimated: `4365`
		// Minimum execution time: 128_264_000 picoseconds.
		Weight::from_parts(141_294_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `KreivoReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `676`
		//  Estimated: `4365`
		// Minimum execution time: 127_480_000 picoseconds.
		Weight::from_parts(142_781_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `159279`
		// Minimum execution time: 53_969_000 picoseconds.
		Weight::from_parts(83_194_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `159279`
		// Minimum execution time: 74_973_000 picoseconds.
		Weight::from_parts(89_883_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `4365`
		// Minimum execution time: 48_442_000 picoseconds.
		Weight::from_parts(61_859_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `415`
		//  Estimated: `159279`
		// Minimum execution time: 76_422_000 picoseconds.
		Weight::from_parts(132_746_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `KreivoReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `574`
		//  Estimated: `159279`
		// Minimum execution time: 196_231_000 picoseconds.
		Weight::from_parts(233_242_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `159279`
		// Minimum execution time: 175_213_000 picoseconds.
		Weight::from_parts(241_673_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `610`
		//  Estimated: `159279`
		// Minimum execution time: 189_292_000 picoseconds.
		Weight::from_parts(228_883_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `159279`
		// Minimum execution time: 177_102_000 picoseconds.
		Weight::from_parts(235_471_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `631`
		//  Estimated: `159279`
		// Minimum execution time: 169_273_000 picoseconds.
		Weight::from_parts(196_791_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `631`
		//  Estimated: `159279`
		// Minimum execution time: 246_600_000 picoseconds.
		Weight::from_parts(290_601_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoCollective::MemberCount` (r:1 w:0)
	/// Proof: `KreivoCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `159279`
		// Minimum execution time: 192_753_000 picoseconds.
		Weight::from_parts(289_148_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::MetadataOf` (r:0 w:1)
	/// Proof: `KreivoReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `348`
		//  Estimated: `4365`
		// Minimum execution time: 72_729_000 picoseconds.
		Weight::from_parts(115_786_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `KreivoReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `KreivoReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `KreivoReferenda::MetadataOf` (r:1 w:1)
	/// Proof: `KreivoReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `4365`
		// Minimum execution time: 63_319_000 picoseconds.
		Weight::from_parts(87_256_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
