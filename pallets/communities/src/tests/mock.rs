use frame_support::{
	parameter_types,
	traits::{
		fungible::HoldConsideration, membership::NonFungibleAdpter, tokens::nonfungible_v2::ItemOf,
		AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32, ConstU64, EnsureOriginWithArg, EqualPrivilegeOnly,
		Footprint,
	},
	weights::Weight,
	PalletId,
};
use frame_system::{EnsureRoot, EnsureRootWithSuccess, EnsureSigned};
use pallet_referenda::{TrackIdOf, TracksInfo};
use parity_scale_codec::Compact;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	traits::{BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, MultiSignature,
};
use virto_common::{CommunityId, MembershipId, MembershipInfo};

use crate::{
	self as pallet_communities,
	origin::EnsureCommunity,
	types::{Tally, VoteWeight},
};

type Block = frame_system::mocking::MockBlock<Test>;
type WeightInfo = ();

pub type AccountPublic = <MultiSignature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type AssetId = u32;
pub type CollectionId = u32;

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

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type AssetIdParameter = Compact<u32>;
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
	type MaxHolds = ();
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
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<10>;
	type MaxFreezes = ConstU32<10>;
}

parameter_types! {
	pub const RootAccount: AccountId = AccountId::new([0xff; 32]);
}
impl pallet_nfts::Config for Test {
	type ApprovalsLimit = ();
	type AttributeDepositBase = ();
	type CollectionDeposit = ();
	type CollectionId = CollectionId;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureRootWithSuccess<AccountId, RootAccount>>;
	type Currency = ();
	type DepositPerByte = ();
	type Features = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type ItemAttributesApprovalsLimit = ();
	type ItemDeposit = ();
	type ItemId = MembershipId;
	type KeyLimit = ConstU32<10>;
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

pub struct EnsureOriginToTrack;
impl EnsureOriginWithArg<RuntimeOrigin, TrackIdOf<Test, ()>> for EnsureOriginToTrack {
	type Success = ();

	fn try_origin(o: RuntimeOrigin, id: &TrackIdOf<Test, ()>) -> Result<Self::Success, RuntimeOrigin> {
		let track_id_for_origin: TrackIdOf<Test, ()> = Tracks::track_for(&o.clone().caller).map_err(|_| o.clone())?;
		frame_support::ensure!(&track_id_for_origin == id, o);

		Ok(())
	}
}

impl pallet_referenda_tracks::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TrackId = CommunityId;
	type MaxTracks = ConstU32<2>;
	type AdminOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureOriginToTrack;
	type WeightInfo = ();
}

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

parameter_types! {
	pub const CommunitiesPalletId: PalletId = PalletId(*b"kv/comms");
	pub const MembershipsCollectionId: CollectionId = 1;
	pub const MembershipNftAttr: &'static [u8; 10] = b"membership";
	pub const TestCommunity: CommunityId = COMMUNITY;
	pub VoteHoldReason: RuntimeHoldReason = pallet_communities::HoldReason::VoteCasted(0).into();
}

type MembershipCollection = ItemOf<Nfts, MembershipsCollectionId, AccountId>;
type Memberships = NonFungibleAdpter<MembershipCollection, MembershipInfo, MembershipNftAttr>;

impl pallet_communities::Config for Test {
	type Assets = Assets;
	type Balances = Balances;
	type CommunityId = CommunityId;
	type CommunityMgmtOrigin = EnsureRoot<AccountId>;
	type MemberMgmtOrigin = EnsureCommunity<Test>;
	type MemberMgmt = Memberships;
	type Membership = MembershipInfo;
	type PalletId = CommunitiesPalletId;
	type Polls = Referenda;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type VoteHoldReason = VoteHoldReason;
	type WeightInfo = WeightInfo;
}

pub const COMMUNITY: CommunityId = CommunityId::new(1);
pub const COMMUNITY_ORGIN: OriginCaller = OriginCaller::Communities(pallet_communities::Origin::<Test>::new(COMMUNITY));

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(members: &[AccountId], memberships: &[MembershipId]) -> sp_io::TestExternalities {
	TestEnvBuilder::new()
		.with_members(members)
		.with_memberships(memberships)
		.build()
}

#[derive(Default)]
pub(crate) struct TestEnvBuilder {
	assets_config: AssetsConfig,
	balances: Vec<(AccountId, Balance)>,
	members: Vec<AccountId>,
	memberships: Vec<MembershipId>,
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

	pub(crate) fn with_balances(mut self, balances: &[(AccountId, Balance)]) -> Self {
		self.balances = balances.to_vec();
		self
	}

	pub(crate) fn with_members(mut self, members: &[AccountId]) -> Self {
		self.members = members.to_vec();
		self
	}

	pub(crate) fn with_memberships(mut self, memberships: &[MembershipId]) -> Self {
		self.memberships = memberships.to_vec();
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
			Communities::create(frame_system::RawOrigin::Root.into(), COMMUNITY_ORGIN, COMMUNITY)
				.expect("Adds community");

			Self::create_memberships(&self.memberships);
			for m in self.members {
				Communities::add_member(COMMUNITY_ORGIN.into(), m.clone()).expect("Adds member");
			}
		});

		ext
	}

	fn create_memberships(memberships: &[MembershipId]) {
		use frame_support::traits::tokens::nonfungible_v2::Mutate;

		let account = Communities::community_account(&COMMUNITY);
		let collection = MembershipsCollectionId::get();

		Nfts::do_create_collection(
			collection,
			account.clone(),
			account.clone(),
			Default::default(),
			0,
			pallet_nfts::Event::ForceCreated {
				collection,
				owner: account.clone(),
			},
		)
		.expect("creates collection");

		for m in memberships {
			MembershipCollection::mint_into(m, &account, &Default::default(), true).expect("can mint");
		}
	}
}
