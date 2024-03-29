use crate::origin::DecisionMethod;
use crate::{CommunityDecisionMethod, CommunityRanksSum, Config};
use fc_traits_memberships::Inspect;
use frame_support::pallet_prelude::*;
use frame_support::traits::{
	fungible::{self, Inspect as FunInspect},
	fungibles::{self, Inspect as FunsInspect},
	Polling,
};
use sp_runtime::traits::{StaticLookup, UniqueSaturatedInto};
use sp_runtime::SaturatedConversion;

pub type AssetIdOf<T> = <<T as Config>::Assets as fungibles::Inspect<AccountIdOf<T>>>::AssetId;
pub type AssetBalanceOf<T> = <<T as Config>::Assets as fungibles::Inspect<AccountIdOf<T>>>::Balance;
pub type NativeBalanceOf<T> = <<T as Config>::Balances as fungible::Inspect<AccountIdOf<T>>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CommunityIdOf<T> = <T as Config>::CommunityId;
pub type VoteOf<T> = Vote<AssetIdOf<T>, AssetBalanceOf<T>, NativeBalanceOf<T>>;
pub type DecisionMethodFor<T> = DecisionMethod<AssetIdOf<T>>;
pub type PollIndexOf<T> = <<T as Config>::Polls as Polling<Tally<T>>>::Index;
pub type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
pub type PalletsOriginOf<T> =
	<<T as frame_system::Config>::RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
pub type MembershipIdOf<T> = <T as Config>::MembershipId;

pub type SizedField<S> = BoundedVec<u8, S>;
pub type ConstSizedField<const S: u32> = SizedField<ConstU32<S>>;

#[cfg(feature = "runtime-benchmarks")]
pub type BenchmarkHelperOf<T> = <T as Config>::BenchmarkHelper;

/// The Community struct holds the basic definition of a community. It includes
/// the current state of a community, the [`AccountId`][1] for the community
/// admin, and (if any) the ID of the community-issued asset the community has
/// marked to be sufficient.
///
/// [1]: `frame_system::Config::AccountId`
#[derive(Decode, Default, Encode, MaxEncodedLen, TypeInfo)]
pub struct CommunityInfo {
	/// The current state of the community.
	pub state: CommunityState,
}

/// The current state of the community. It represents whether a community
/// is awaiting to prove their contribution to the network, is active
/// and can operate, blocked due to a violation of network norms, or
/// it's being frozen by the community administrators.
#[derive(Decode, Default, Encode, MaxEncodedLen, PartialEq, TypeInfo)]
pub enum CommunityState {
	/// The community is opperating normally.
	#[default]
	Active,
	/// The community is blocked, typically as a result of a restriction imposed
	/// by violating the norms of the network.
	Blocked,
}

/// The CommunityMetadata struct stores some descriptive information about
/// the community.
#[derive(Clone, Debug, Decode, Default, Encode, Eq, MaxEncodedLen, PartialEq, TypeInfo)]
pub struct CommunityMetadata {
	/// The name of the community
	pub name: ConstSizedField<64>,
	/// A short description of the community
	pub description: ConstSizedField<256>,
	/// The main URL that can lead to information about the community
	pub main_url: ConstSizedField<256>,
}

// Governance

pub type VoteWeight = u32;

///
#[derive(Clone, Debug, Decode, Encode, PartialEq, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(AssetId, AssetBalance, NativeBalance))]
pub enum Vote<AssetId, AssetBalance, NativeBalance> {
	AssetBalance(bool, AssetId, AssetBalance),
	NativeBalance(bool, NativeBalance),
	Standard(bool),
}

impl<A, B, N> From<Vote<A, B, N>> for VoteWeight
where
	B: UniqueSaturatedInto<VoteWeight>,
	N: UniqueSaturatedInto<VoteWeight>,
{
	fn from(value: Vote<A, B, N>) -> Self {
		match value {
			Vote::AssetBalance(_, _, balance) => balance.saturated_into(),
			Vote::NativeBalance(_, balance) => balance.saturated_into(),
			Vote::Standard(_) => 1,
		}
	}
}

impl<A, B, N> From<Vote<A, B, N>> for bool {
	fn from(value: Vote<A, B, N>) -> bool {
		match value {
			Vote::AssetBalance(say, _, _) => say,
			Vote::NativeBalance(say, _) => say,
			Vote::Standard(say) => say,
		}
	}
}

///
#[derive(Clone, Debug, Decode, Encode, Eq, MaxEncodedLen, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: Config))]
pub struct Tally<T> {
	pub(crate) _phantom: PhantomData<T>,
	pub(crate) ayes: VoteWeight,
	pub(crate) nays: VoteWeight,
	pub(crate) bare_ayes: VoteWeight,
}

impl<T> Default for Tally<T> {
	fn default() -> Self {
		Self {
			_phantom: Default::default(),
			ayes: Default::default(),
			nays: Default::default(),
			bare_ayes: Default::default(),
		}
	}
}

impl<T: Config> Tally<T> {
	pub(crate) fn max_support(community_id: CommunityIdOf<T>) -> VoteWeight {
		match CommunityDecisionMethod::<T>::get(community_id) {
			DecisionMethod::Membership => T::MemberMgmt::member_count(&community_id),
			DecisionMethod::Rank => CommunityRanksSum::<T>::get(community_id),
			DecisionMethod::NativeToken => T::Balances::total_issuance().saturated_into::<VoteWeight>(),
			DecisionMethod::CommunityAsset(asset_id) => {
				T::Assets::total_issuance(asset_id).saturated_into::<VoteWeight>()
			}
		}
	}
}

#[cfg(feature = "runtime-benchmarks")]
use {
	frame_benchmarking::BenchmarkError,
	frame_system::pallet_prelude::{OriginFor, RuntimeCallFor},
};

#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<T: Config> {
	/// Returns the ID of the community to use in benchmarks
	fn community_id() -> CommunityIdOf<T>;

	/// Returns the desired size of the community for
	/// effects of benchmark testing
	fn community_desired_size() -> u32;

	/// Returns the origin for the community
	/// as well as the caller
	fn community_origin(decision_method: DecisionMethodFor<T>) -> OriginFor<T>;

	/// Returns a new membership ID for a community with a given index.
	fn membership_id(community_id: CommunityIdOf<T>, index: u32) -> MembershipIdOf<T>;

	/// Initializes the membership collection of a community.
	fn initialize_memberships_collection() -> Result<(), frame_benchmarking::BenchmarkError>;

	/// Extends the membership collection of a community with a given
	/// membership ID.
	fn issue_membership(
		community_id: CommunityIdOf<T>,
		membership_id: MembershipIdOf<T>,
	) -> Result<(), frame_benchmarking::BenchmarkError>;

	/// This method prepares the referenda track to be used
	/// to submit the poll, for benchmarking purposes.
	fn prepare_track(track_origin: PalletsOriginOf<T>) -> Result<(), BenchmarkError>;

	/// This method prepares the poll to be used to
	/// benchmark vote-related calls.
	fn prepare_poll(
		origin: OriginFor<T>,
		proposal_origin: PalletsOriginOf<T>,
		proposal_call: RuntimeCallFor<T>,
	) -> Result<PollIndexOf<T>, BenchmarkError>;

	fn finish_poll(index: PollIndexOf<T>) -> Result<(), BenchmarkError>;
}
