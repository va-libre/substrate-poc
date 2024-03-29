use super::*;

use frame_support::{
	pallet_prelude::{EnsureOrigin, PhantomData},
	traits::OriginTrait,
};
use pallet_communities::origin::EnsureCommunity;
use sp_runtime::traits::AccountIdConversion;
use virto_common::{CommunityId, MembershipId};

pub mod governance;
pub mod memberships;

#[cfg(feature = "runtime-benchmarks")]
use self::{
	governance::{CommunityReferendaInstance, CommunityTracksInstance},
	memberships::CommunityMembershipsInstance,
};

#[cfg(feature = "runtime-benchmarks")]
use ::{
	frame_benchmarking::BenchmarkError,
	frame_support::traits::{schedule::DispatchTime, tokens::nonfungible_v2::Mutate},
	frame_system::pallet_prelude::{OriginFor, RuntimeCallFor},
	pallet_communities::{
		types::{CommunityIdOf, DecisionMethodFor, MembershipIdOf, PalletsOriginOf, PollIndexOf},
		BenchmarkHelper, Origin,
	},
	pallet_nfts::Pallet as Nfts,
	pallet_referenda::{BoundedCallOf, Curve, Pallet as Referenda, TrackInfo},
	pallet_referenda_tracks::Pallet as Tracks,
	sp_core::Encode,
	sp_runtime::Perbill,
	virto_common::MembershipId,
};

parameter_types! {
  pub const CommunityPalletId: PalletId = PalletId(*b"kv/cmtys");
	pub const MembershipsCollectionId: CommunityId = CommunityId::new(0);
	pub const MembershipNftAttr: &'static [u8; 10] = b"membership";
}

pub struct EnsureCommunityAccountId<T>(PhantomData<T>);

impl<T> EnsureOrigin<T::RuntimeOrigin> for EnsureCommunityAccountId<T>
where
	T::RuntimeOrigin: OriginTrait + From<frame_system::RawOrigin<T::AccountId>> + From<pallet_communities::Origin<T>>,
	T: pallet_communities::Config,
{
	type Success = T::CommunityId;

	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		match o.clone().into() {
			Ok(frame_system::RawOrigin::Signed(account_id)) => {
				let (_, community_id) = PalletId::try_from_sub_account(&account_id).ok_or(o.clone())?;
				Ok(community_id)
			}
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		Ok(Origin::new(T::BenchmarkHelper::community_id()).into())
	}
}

impl pallet_communities::Config for Runtime {
	type CommunityId = CommunityId;

	type CommunityMgmtOrigin = EnsureRoot<AccountId>;
	type MemberMgmtOrigin = EitherOf<EnsureCommunity<Self>, EnsureCommunityAccountId<Self>>;
	type MemberMgmt = CommunityMemberships;
	type MembershipId = MembershipId;

	type Polls = CommunityReferenda;

	type Assets = Assets;
	type Balances = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_communities::weights::SubstrateWeight<Runtime>;

	type PalletId = CommunityPalletId;

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = CommunityBenchmarkHelper;
}

#[cfg(feature = "runtime-benchmarks")]
type MembershipCollection = ItemOf<CommunityMemberships, MembershipsCollectionId, AccountId>;

#[cfg(feature = "runtime-benchmarks")]
pub struct CommunityBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl BenchmarkHelper<Runtime> for CommunityBenchmarkHelper {
	fn community_id() -> CommunityIdOf<Runtime> {
		CommunityId::new(1)
	}
	fn community_desired_size() -> u32 {
		u32::MAX
	}
	fn community_origin(decision_method: DecisionMethodFor<Runtime>) -> OriginFor<Runtime> {
		let mut origin = Origin::<Runtime>::new(Self::community_id());
		origin.with_decision_method(decision_method.clone());
		origin.into()
	}

	fn membership_id(community_id: CommunityIdOf<Runtime>, index: u32) -> MembershipIdOf<Runtime> {
		MembershipId(community_id, index)
	}

	fn initialize_memberships_collection() -> Result<(), BenchmarkError> {
		let collection = MembershipsCollectionId::get();
		Nfts::<Runtime, CommunityMembershipsInstance>::do_create_collection(
			collection,
			TreasuryAccount::get(),
			TreasuryAccount::get(),
			Default::default(),
			0,
			pallet_nfts::Event::ForceCreated {
				collection,
				owner: TreasuryAccount::get(),
			},
		)?;

		let community_id = Self::community_id();
		let community_account = pallet_communities::Pallet::<Runtime>::community_account(&community_id);
		Nfts::<Runtime, CommunityMembershipsInstance>::do_create_collection(
			community_id,
			community_account,
			community_account,
			Default::default(),
			0,
			pallet_nfts::Event::ForceCreated {
				collection: community_id,
				owner: community_account,
			},
		)?;

		Ok(())
	}

	fn issue_membership(
		community_id: CommunityIdOf<Runtime>,
		membership_id: MembershipIdOf<Runtime>,
	) -> Result<(), BenchmarkError> {
		let community_account = pallet_communities::Pallet::<Runtime>::community_account(&community_id);
		MembershipCollection::mint_into(&membership_id, &community_account, &Default::default(), true)?;

		Ok(())
	}

	fn prepare_track(pallet_origin: PalletsOriginOf<Runtime>) -> Result<(), BenchmarkError> {
		let id = Self::community_id();
		let info = TrackInfo {
			name: sp_runtime::str_array("Community"),
			max_deciding: 1,
			decision_deposit: 5,
			prepare_period: 1,
			decision_period: 5,
			confirm_period: 1,
			min_enactment_period: 1,
			min_approval: Curve::LinearDecreasing {
				length: Perbill::from_percent(100),
				floor: Perbill::from_percent(50),
				ceil: Perbill::from_percent(100),
			},
			min_support: Curve::LinearDecreasing {
				length: Perbill::from_percent(100),
				floor: Perbill::from_percent(0),
				ceil: Perbill::from_percent(100),
			},
		};

		Tracks::<Runtime, CommunityTracksInstance>::insert(RuntimeOrigin::root(), id, info, pallet_origin)?;

		Ok(())
	}

	fn prepare_poll(
		origin: OriginFor<Runtime>,
		proposal_origin: PalletsOriginOf<Runtime>,
		proposal_call: RuntimeCallFor<Runtime>,
	) -> Result<PollIndexOf<Runtime>, BenchmarkError> {
		let bounded_call = BoundedVec::truncate_from(proposal_call.encode());
		let proposal_origin = Box::new(proposal_origin);
		let proposal = BoundedCallOf::<Runtime, CommunityReferendaInstance>::Inline(bounded_call);
		let enactment_moment = DispatchTime::After(1);

		let index = 0u32;
		Referenda::<Runtime, CommunityReferendaInstance>::submit(
			origin.clone(),
			proposal_origin,
			proposal,
			enactment_moment,
		)?;
		Referenda::<Runtime, CommunityReferendaInstance>::place_decision_deposit(origin, index)?;

		System::set_block_number(2);
		Referenda::<Runtime, CommunityReferendaInstance>::nudge_referendum(RuntimeOrigin::root(), 0)?;

		Ok(0)
	}

	fn finish_poll(index: PollIndexOf<Runtime>) -> Result<(), BenchmarkError> {
		System::set_block_number(8);
		Referenda::<Runtime, CommunityReferendaInstance>::nudge_referendum(RuntimeOrigin::root(), index)?;

		frame_support::assert_ok!(Referenda::<Runtime, CommunityReferendaInstance>::ensure_ongoing(index));

		System::set_block_number(9);
		Referenda::<Runtime, CommunityReferendaInstance>::nudge_referendum(RuntimeOrigin::root(), index)?;

		frame_support::assert_err!(
			Referenda::<Runtime, CommunityReferendaInstance>::ensure_ongoing(index),
			pallet_referenda::Error::<Runtime, CommunityReferendaInstance>::NotOngoing
		);

		Ok(())
	}
}
