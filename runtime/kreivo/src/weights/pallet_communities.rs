
//! Autogenerated weights for `pallet_communities`
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
// pallet_communities
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

/// Weight functions for `pallet_communities`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_communities::WeightInfo for WeightInfo<T> {
	/// Storage: `Communities::Info` (r:1 w:1)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityIdFor` (r:1 w:1)
	/// Proof: `Communities::CommunityIdFor` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `4087`
		// Minimum execution time: 71_539_000 picoseconds.
		Weight::from_parts(73_261_000, 0)
			.saturating_add(Weight::from_parts(0, 4087))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Communities::CommunityIdFor` (r:1 w:2)
	/// Proof: `Communities::CommunityIdFor` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	fn set_admin_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `160`
		//  Estimated: `4087`
		// Minimum execution time: 67_495_000 picoseconds.
		Weight::from_parts(69_555_000, 0)
			.saturating_add(Weight::from_parts(0, 4087))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	/// Storage: `Assets::NextAssetId` (r:1 w:0)
	/// Proof: `Assets::NextAssetId` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityDecisionMethod` (r:0 w:1)
	/// Proof: `Communities::CommunityDecisionMethod` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_decision_method() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229`
		//  Estimated: `3689`
		// Minimum execution time: 51_255_000 picoseconds.
		Weight::from_parts(53_101_000, 0)
			.saturating_add(Weight::from_parts(0, 3689))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Account` (r:1 w:3)
	/// Proof: `CommunityMemberships::Account` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Collection` (r:2 w:1)
	/// Proof: `CommunityMemberships::Collection` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Attribute` (r:6 w:2)
	/// Proof: `CommunityMemberships::Attribute` (`max_values`: None, `max_size`: Some(477), added: 2952, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::CollectionConfigOf` (r:2 w:0)
	/// Proof: `CommunityMemberships::CollectionConfigOf` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::ItemConfigOf` (r:2 w:1)
	/// Proof: `CommunityMemberships::ItemConfigOf` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Item` (r:2 w:2)
	/// Proof: `CommunityMemberships::Item` (`max_values`: None, `max_size`: Some(859), added: 3334, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::ItemPriceOf` (r:0 w:1)
	/// Proof: `CommunityMemberships::ItemPriceOf` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::PendingSwapOf` (r:0 w:1)
	/// Proof: `CommunityMemberships::PendingSwapOf` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `928`
		//  Estimated: `18702`
		// Minimum execution time: 446_351_000 picoseconds.
		Weight::from_parts(544_610_000, 0)
			.saturating_add(Weight::from_parts(0, 18702))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(11))
	}
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Account` (r:1 w:3)
	/// Proof: `CommunityMemberships::Account` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Attribute` (r:5 w:3)
	/// Proof: `CommunityMemberships::Attribute` (`max_values`: None, `max_size`: Some(477), added: 2952, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Collection` (r:2 w:1)
	/// Proof: `CommunityMemberships::Collection` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::ItemConfigOf` (r:2 w:1)
	/// Proof: `CommunityMemberships::ItemConfigOf` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Item` (r:2 w:2)
	/// Proof: `CommunityMemberships::Item` (`max_values`: None, `max_size`: Some(859), added: 3334, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::ItemMetadataOf` (r:1 w:0)
	/// Proof: `CommunityMemberships::ItemMetadataOf` (`max_values`: None, `max_size`: Some(345), added: 2820, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::CollectionConfigOf` (r:1 w:0)
	/// Proof: `CommunityMemberships::CollectionConfigOf` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::ItemPriceOf` (r:0 w:2)
	/// Proof: `CommunityMemberships::ItemPriceOf` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::ItemAttributesApprovalsOf` (r:0 w:1)
	/// Proof: `CommunityMemberships::ItemAttributesApprovalsOf` (`max_values`: None, `max_size`: Some(999), added: 3474, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::PendingSwapOf` (r:0 w:2)
	/// Proof: `CommunityMemberships::PendingSwapOf` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn remove_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1389`
		//  Estimated: `15750`
		// Minimum execution time: 552_210_000 picoseconds.
		Weight::from_parts(666_901_000, 0)
			.saturating_add(Weight::from_parts(0, 15750))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Attribute` (r:2 w:2)
	/// Proof: `CommunityMemberships::Attribute` (`max_values`: None, `max_size`: Some(477), added: 2952, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Collection` (r:1 w:1)
	/// Proof: `CommunityMemberships::Collection` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	fn promote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `755`
		//  Estimated: `6894`
		// Minimum execution time: 180_116_000 picoseconds.
		Weight::from_parts(292_475_000, 0)
			.saturating_add(Weight::from_parts(0, 6894))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Attribute` (r:2 w:2)
	/// Proof: `CommunityMemberships::Attribute` (`max_values`: None, `max_size`: Some(477), added: 2952, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Collection` (r:1 w:1)
	/// Proof: `CommunityMemberships::Collection` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	fn demote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `793`
		//  Estimated: `6894`
		// Minimum execution time: 170_118_000 picoseconds.
		Weight::from_parts(235_429_000, 0)
			.saturating_add(Weight::from_parts(0, 6894))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `CommunityMemberships::Account` (r:1 w:0)
	/// Proof: `CommunityMemberships::Account` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityDecisionMethod` (r:1 w:0)
	/// Proof: `Communities::CommunityDecisionMethod` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityVotes` (r:1 w:1)
	/// Proof: `Communities::CommunityVotes` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `CommunityReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `CommunityReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityVoteLocks` (r:2 w:1)
	/// Proof: `Communities::CommunityVoteLocks` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::Freezes` (r:1 w:1)
	/// Proof: `AssetsFreezer::Freezes` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:1)
	/// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2494`
		//  Estimated: `6148`
		// Minimum execution time: 314_045_000 picoseconds.
		Weight::from_parts(458_108_000, 0)
			.saturating_add(Weight::from_parts(0, 6148))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `CommunityMemberships::Account` (r:1 w:0)
	/// Proof: `CommunityMemberships::Account` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityDecisionMethod` (r:1 w:0)
	/// Proof: `Communities::CommunityDecisionMethod` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `CommunityReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `CommunityReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityVotes` (r:1 w:1)
	/// Proof: `Communities::CommunityVotes` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `CommunityMemberships::Attribute` (r:1 w:0)
	/// Proof: `CommunityMemberships::Attribute` (`max_values`: None, `max_size`: Some(477), added: 2952, mode: `MaxEncodedLen`)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2970`
		//  Estimated: `4365`
		// Minimum execution time: 204_186_000 picoseconds.
		Weight::from_parts(299_085_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `CommunityReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `CommunityReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Communities::CommunityVoteLocks` (r:2 w:1)
	/// Proof: `Communities::CommunityVoteLocks` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(4658), added: 7133, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1154`
		//  Estimated: `8123`
		// Minimum execution time: 198_655_000 picoseconds.
		Weight::from_parts(275_426_000, 0)
			.saturating_add(Weight::from_parts(0, 8123))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Communities::Info` (r:1 w:0)
	/// Proof: `Communities::Info` (`max_values`: None, `max_size`: Some(19), added: 2494, mode: `MaxEncodedLen`)
	fn dispatch_as_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `3484`
		// Minimum execution time: 37_320_000 picoseconds.
		Weight::from_parts(45_762_000, 0)
			.saturating_add(Weight::from_parts(0, 3484))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
