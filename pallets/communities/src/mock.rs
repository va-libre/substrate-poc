use frame_support::{
	dispatch::DispatchResult,
	parameter_types,
	traits::{
		fungible::HoldConsideration, tokens::nonfungible_v2::ItemOf, AsEnsureOriginWithArg, ConstU128, ConstU16,
		ConstU32, ConstU64, EitherOf, EnsureOriginWithArg, EqualPrivilegeOnly, Footprint, OriginTrait,
	},
	weights::Weight,
	PalletId,
};
use frame_system::{EnsureRoot, EnsureRootWithSuccess, EnsureSigned};
use pallet_referenda::{TrackIdOf, TrackInfoOf, TracksInfo};
use parity_scale_codec::Compact;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	traits::{BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, MultiSignature,
};
pub use virto_common::{CommunityId, MembershipId};

use crate::{
	self as pallet_communities,
	origin::{DecisionMethod, EnsureCommunity},
	types::{Tally, VoteWeight},
};

type Block = frame_system::mocking::MockBlock<Test>;
type WeightInfo = ();

pub type AccountPublic = <MultiSignature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type AssetId = u32;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		Assets: pallet_assets,
		Balances: pallet_balances,
		Communities: pallet_communities,
		Nfts: pallet_nfts,
		Preimage: pallet_preimage,
		Referenda: pallet_referenda,
		Scheduler: pallet_scheduler,
		System: frame_system,
		Tracks: pallet_referenda_tracks,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// Monetary operations

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type AssetIdParameter = Compact<AssetId>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<100>;
	type AssetAccountDeposit = ConstU128<1>;
	type MetadataDepositBase = ConstU128<10>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = WeightInfo;
	type RemoveItemsLimit = ConstU32<1000>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxHolds = ConstU32<10>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = WeightInfo;
	type MaxLocks = ConstU32<10>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = RuntimeHoldReason;
	type MaxHolds = ConstU32<10>;
	type MaxFreezes = ConstU32<10>;
}

// Memberships
#[cfg(feature = "runtime-benchmarks")]
pub struct NftsBenchmarksHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_nfts::BenchmarkHelper<CommunityId, MembershipId> for NftsBenchmarksHelper {
	fn collection(_i: u16) -> CommunityId {
		COMMUNITY
	}
	fn item(i: u16) -> MembershipId {
		MembershipId(COMMUNITY, i.into())
	}
}

parameter_types! {
	pub const RootAccount: AccountId = AccountId::new([0xff; 32]);
}
impl pallet_nfts::Config for Test {
	type ApprovalsLimit = ();
	type AttributeDepositBase = ();
	type CollectionDeposit = ();
	type CollectionId = CommunityId;
	type CreateOrigin =
		AsEnsureOriginWithArg<EitherOf<EnsureRootWithSuccess<AccountId, RootAccount>, EnsureSigned<AccountId>>>;
	type Currency = ();
	type DepositPerByte = ();
	type Features = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type ItemAttributesApprovalsLimit = ();
	type ItemDeposit = ();
	type ItemId = MembershipId;
	type KeyLimit = ConstU32<64>;
	type Locker = ();
	type MaxAttributesPerCall = ();
	type MaxDeadlineDuration = ();
	type MaxTips = ();
	type MetadataDepositBase = ();
	type OffchainPublic = AccountPublic;
	type OffchainSignature = MultiSignature;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = ();
	type ValueLimit = ConstU32<10>;
	type WeightInfo = ();

	#[cfg(feature = "runtime-benchmarks")]
	type Helper = NftsBenchmarksHelper;
}

// Governance at Communities

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Weight::from_parts(1_000_000_000, 1_048_576);
	pub const MaxScheduledPerBlock: u32 = 512;
}
pub struct ConvertDeposit;
impl Convert<Footprint, u128> for ConvertDeposit {
	fn convert(a: Footprint) -> u128 {
		(a.count * 2 + a.size).into()
	}
}

parameter_types! {
	pub PreimageHoldReason: RuntimeHoldReason = pallet_preimage::HoldReason::Preimage.into();
}

impl pallet_preimage::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureSigned<AccountId>;
	type Consideration = HoldConsideration<AccountId, Balances, PreimageHoldReason, ConvertDeposit>;
}

impl pallet_scheduler::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Self>;
	type Preimages = Preimage;
}

pub struct EnsureOriginToTrack;
impl EnsureOriginWithArg<RuntimeOrigin, TrackIdOf<Test, ()>> for EnsureOriginToTrack {
	type Success = ();

	fn try_origin(o: RuntimeOrigin, id: &TrackIdOf<Test, ()>) -> Result<Self::Success, RuntimeOrigin> {
		let track_id_for_origin: TrackIdOf<Test, ()> = Tracks::track_for(&o.clone().caller).map_err(|_| o.clone())?;
		frame_support::ensure!(&track_id_for_origin == id, o);

		Ok(())
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(id: &TrackIdOf<Test, ()>) -> Result<RuntimeOrigin, ()> {
		Ok(pallet_communities::Origin::<Test>::new(id.clone()).into())
	}
}

#[cfg(feature = "runtime-benchmarks")]
use sp_runtime::SaturatedConversion;

#[cfg(feature = "runtime-benchmarks")]
pub struct TracksBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl pallet_referenda_tracks::BenchmarkHelper<Test> for TracksBenchmarkHelper {
	fn track_id(id: u32) -> TrackIdOf<Test, ()> {
		CommunityId::new(id.saturated_into())
	}
}

parameter_types! {
	pub const MaxTracks: u32 = u32::MAX;
}
impl pallet_referenda_tracks::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TrackId = CommunityId;
	type MaxTracks = MaxTracks;
	type AdminOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureOriginToTrack;
	type WeightInfo = ();

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = TracksBenchmarkHelper;
}

parameter_types! {
		pub static AlarmInterval: u64 = 1;
}
impl pallet_referenda::Config for Test {
	type WeightInfo = ();
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = Scheduler;
	type Currency = pallet_balances::Pallet<Self>;
	type SubmitOrigin = frame_system::EnsureSigned<AccountId>;
	type CancelOrigin = EnsureRoot<AccountId>;
	type KillOrigin = EnsureRoot<AccountId>;
	type Slash = ();
	type Votes = VoteWeight;
	type Tally = Tally<Test>;
	type SubmissionDeposit = ConstU128<2>;
	type MaxQueued = ConstU32<3>;
	type UndecidingTimeout = ConstU64<20>;
	type AlarmInterval = AlarmInterval;
	type Tracks = Tracks;
	type Preimages = Preimage;
}

// Communities

parameter_types! {
	pub const CommunitiesPalletId: PalletId = PalletId(*b"kv/comms");
	pub const MembershipsManagerCollectionId: CommunityId = CommunityId::new(0);
	pub const MembershipNftAttr: &'static [u8; 10] = b"membership";
	pub const TestCommunity: CommunityId = COMMUNITY;
}

type MembershipCollection = ItemOf<Nfts, MembershipsManagerCollectionId, AccountId>;

#[cfg(feature = "runtime-benchmarks")]
use crate::{
	types::{CommunityIdOf, DecisionMethodFor, MembershipIdOf, PollIndexOf},
	BenchmarkHelper, Origin,
};

#[cfg(feature = "runtime-benchmarks")]
use {
	frame_benchmarking::BenchmarkError,
	frame_support::BoundedVec,
	frame_system::pallet_prelude::{OriginFor, RuntimeCallFor},
	pallet_referenda::{BoundedCallOf, Curve, PalletsOriginOf, TrackInfo},
	parity_scale_codec::Encode,
	sp_runtime::Perbill,
};

#[cfg(feature = "runtime-benchmarks")]
pub struct CommunityBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl BenchmarkHelper<Test> for CommunityBenchmarkHelper {
	fn community_id() -> CommunityIdOf<Test> {
		COMMUNITY
	}

	fn community_desired_size() -> u32 {
		u8::MAX as u32
	}

	fn community_origin(decision_method: DecisionMethodFor<Test>) -> OriginFor<Test> {
		let mut origin = Origin::<Test>::new(Self::community_id());
		origin.with_decision_method(decision_method);

		origin.into()
	}

	fn membership_id(community_id: CommunityIdOf<Test>, index: u32) -> MembershipIdOf<Test> {
		MembershipId(community_id, index)
	}

	fn initialize_memberships_collection() -> Result<(), frame_benchmarking::BenchmarkError> {
		TestEnvBuilder::initialize_memberships_manager_collection()?;
		TestEnvBuilder::initialize_community_memberships_collection(&Self::community_id())?;
		Ok(())
	}

	fn issue_membership(
		community_id: CommunityIdOf<Test>,
		membership_id: MembershipIdOf<Test>,
	) -> Result<(), frame_benchmarking::BenchmarkError> {
		use frame_support::traits::tokens::nonfungible_v2::Mutate;

		let community_account = Communities::community_account(&community_id);
		MembershipCollection::mint_into(&membership_id, &community_account, &Default::default(), true)?;

		Ok(())
	}

	fn prepare_track(track_origin: PalletsOriginOf<Test>) -> Result<(), BenchmarkError> {
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

		Tracks::insert(RuntimeOrigin::root(), id, info, track_origin.clone())?;

		Ok(())
	}

	fn prepare_poll(
		origin: OriginFor<Test>,
		proposal_origin: PalletsOriginOf<Test>,
		proposal_call: RuntimeCallFor<Test>,
	) -> Result<PollIndexOf<Test>, BenchmarkError> {
		let proposal = BoundedCallOf::<Test, ()>::Inline(BoundedVec::truncate_from(proposal_call.encode()));
		let enactment_moment = frame_support::traits::schedule::DispatchTime::After(1);
		Referenda::submit(origin.clone(), Box::new(proposal_origin), proposal, enactment_moment)?;
		Referenda::place_decision_deposit(origin, 0)?;

		System::set_block_number(2);
		Referenda::nudge_referendum(RuntimeOrigin::root(), 0)?;

		Ok(0)
	}

	fn finish_poll(index: PollIndexOf<Test>) -> Result<(), BenchmarkError> {
		System::set_block_number(8);
		Referenda::nudge_referendum(RuntimeOrigin::root(), index)?;

		frame_support::assert_ok!(Referenda::ensure_ongoing(index));

		System::set_block_number(9);
		Referenda::nudge_referendum(RuntimeOrigin::root(), index)?;

		frame_support::assert_err!(
			Referenda::ensure_ongoing(index),
			pallet_referenda::Error::<Test, ()>::NotOngoing
		);

		Ok(())
	}
}

impl pallet_communities::Config for Test {
	type PalletId = CommunitiesPalletId;
	type CommunityId = CommunityId;
	type MembershipId = MembershipId;

	type Assets = Assets;
	type Balances = Balances;
	type MemberMgmt = Nfts;
	type Polls = Referenda;

	type CommunityMgmtOrigin = EnsureRoot<AccountId>;
	type MemberMgmtOrigin = EnsureCommunity<Self>;

	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type WeightInfo = WeightInfo;

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = CommunityBenchmarkHelper;
}

pub const COMMUNITY: CommunityId = CommunityId::new(1);
pub const COMMUNITY_ORIGIN: OriginCaller =
	OriginCaller::Communities(pallet_communities::Origin::<Test>::new(COMMUNITY));

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(members: &[AccountId], memberships: &[MembershipId]) -> sp_io::TestExternalities {
	TestEnvBuilder::new()
		.add_community(COMMUNITY, DecisionMethod::Membership, members, memberships, None)
		.build()
}

#[derive(Default)]
pub(crate) struct TestEnvBuilder {
	assets_config: AssetsConfig,
	balances: Vec<(AccountId, Balance)>,
	communities: Vec<CommunityId>,
	decision_methods: sp_std::collections::btree_map::BTreeMap<CommunityId, DecisionMethod<AssetId>>,
	members: Vec<(CommunityId, AccountId)>,
	memberships: Vec<(CommunityId, MembershipId)>,
	tracks: Vec<(TrackIdOf<Test, ()>, TrackInfoOf<Test>)>,
}

impl TestEnvBuilder {
	pub(crate) fn new() -> Self {
		Self::default()
	}

	pub(crate) fn add_asset(
		mut self,
		id: &AssetId,
		owner: &AccountId,
		is_sufficient: bool,
		min_balance: Balance,
		// name, symbol, decimals
		maybe_metadata: Option<(Vec<u8>, Vec<u8>, u8)>,
		maybe_accounts: Option<Vec<(AccountId, Balance)>>,
	) -> Self {
		self.assets_config
			.assets
			.push((*id, owner.clone(), is_sufficient, min_balance));

		if let Some((name, symbol, decimals)) = maybe_metadata {
			self.assets_config.metadata.push((*id, name, symbol, decimals));
		}

		self.assets_config.accounts.append(
			&mut maybe_accounts
				.unwrap_or_default()
				.into_iter()
				.map(|(account_id, balance)| (*id, account_id, balance))
				.collect(),
		);

		self
	}

	pub(crate) fn add_community(
		mut self,
		community_id: CommunityId,
		decision_method: DecisionMethod<AssetId>,
		members: &[AccountId],
		memberships: &[MembershipId],
		maybe_track: Option<TrackInfoOf<Test>>,
	) -> Self {
		self.communities.push(community_id);
		self.decision_methods.insert(community_id, decision_method);
		self.members
			.append(&mut members.iter().map(|m| (community_id, m.to_owned())).collect::<Vec<_>>());
		self.memberships.append(
			&mut memberships
				.iter()
				.map(|m| (community_id, m.to_owned()))
				.collect::<Vec<_>>(),
		);
		if let Some(track) = maybe_track {
			self.tracks.push((community_id, track));
		}

		self
	}

	pub(crate) fn with_balances(mut self, balances: &[(AccountId, Balance)]) -> Self {
		self.balances = balances.to_vec();
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let t = RuntimeGenesisConfig {
			assets: self.assets_config,
			balances: pallet_balances::GenesisConfig {
				balances: self.balances,
			},
			system: Default::default(),
		}
		.build_storage()
		.unwrap();

		let mut ext = TestExternalities::new(t);

		ext.execute_with(|| {
			System::set_block_number(1);

			Self::initialize_memberships_manager_collection().expect("collection is initialized");

			for community_id in &self.communities {
				Self::initialize_community_memberships_collection(community_id).expect("collection is initialized");

				let decision_method = self
					.decision_methods
					.get(community_id)
					.expect("should include decision_method on add_community");
				let community_origin: RuntimeOrigin = Self::create_community_origin(community_id, decision_method);

				Communities::create(RuntimeOrigin::root(), community_origin.caller().clone(), *community_id)
					.expect("can add community");

				Communities::set_decision_method(RuntimeOrigin::root(), *community_id, decision_method.clone())
					.expect("can set decision info");

				let mut members = self.members.iter().filter(|(cid, _)| cid == community_id);
				let memberships = self.memberships.iter().filter(|(cid, _)| cid == community_id);

				assert!(
					self.memberships.len() >= self.members.len(),
					"there should be at least as many memberships as there are members"
				);

				for (_, membership) in memberships {
					use frame_support::traits::tokens::nonfungible_v2::Mutate;

					let account = Communities::community_account(community_id);
					MembershipCollection::mint_into(membership, &account, &Default::default(), true)
						.expect("can mint membership");

					if let Some((_, who)) = members.next() {
						Communities::add_member(community_origin.clone(), who.clone()).expect("can add member");
					}
				}

				for (_, track_info) in self.tracks.iter().filter(|(cid, _)| cid == community_id) {
					Tracks::insert(
						RuntimeOrigin::root(),
						*community_id,
						track_info.clone(),
						community_origin.caller().clone(),
					)
					.expect("can add track");
				}
			}
		});

		ext
	}

	pub(crate) fn initialize_memberships_manager_collection() -> DispatchResult {
		Nfts::do_create_collection(
			MembershipsManagerCollectionId::get(),
			RootAccount::get(),
			RootAccount::get(),
			Default::default(),
			0,
			pallet_nfts::Event::ForceCreated {
				collection: MembershipsManagerCollectionId::get(),
				owner: RootAccount::get(),
			},
		)
	}

	pub(crate) fn initialize_community_memberships_collection(community_id: &CommunityId) -> DispatchResult {
		let account = Communities::community_account(community_id);
		Nfts::do_create_collection(
			*community_id,
			account.clone(),
			account.clone(),
			Default::default(),
			0,
			pallet_nfts::Event::ForceCreated {
				collection: *community_id,
				owner: account,
			},
		)
	}

	pub fn create_community_origin(
		community_id: &CommunityId,
		decision_method: &DecisionMethod<AssetId>,
	) -> RuntimeOrigin {
		let mut origin = pallet_communities::Origin::<Test>::new(*community_id);
		origin.with_decision_method(decision_method.clone());

		origin.into()
	}
}
