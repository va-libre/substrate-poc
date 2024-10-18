//! System support stuff.

use fc_traits_authn::{composite_authenticator, util::AuthorityFromPalletId, Challenge, Challenger};
use frame_support::PalletId;
use frame_system::EnsureRootWithSuccess;
use polkadot_core_primitives::HashT;

use super::*;

// #[runtime::pallet_index(0)]
// pub type System
#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	type Lookup = CommunityLookup;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	type Block = Block;
	type Nonce = Nonce;
	/// Maximum number of block number to block hash mappings to keep (oldest
	/// pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate
	/// prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// #[runtime::pallet_index(1)]
// pub type ParachainSystem
parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
	type ConsensusHook = ConsensusHook;
}

// #[runtime::pallet_index(2)]
// pub type Timestamp
impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

// #[runtime::pallet_index(3)]
// pub type ParachainInfo
impl parachain_info::Config for Runtime {}

// #[runtime::pallet_index(4)]
// pub type Origins
impl pallet_custom_origins::Config for Runtime {}

// #[runtime::pallet_index(5)]
// pub type Sudo
#[cfg(feature = "paseo")]
impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_sudo::weights::SubstrateWeight<Self>;
}

// #[runtime::pallet_index(6)]
// pub type Pass
parameter_types! {
	pub PassPalletId: PalletId = PalletId(*b"kreivo_p");
	pub NeverPays: Option<pallet_pass::DepositInformation<Runtime>> = None;
}

pub struct UnincludedBlockChallenger;

impl Challenger for UnincludedBlockChallenger {
	type Context = BlockNumber;

	fn check_challenge(cx: &Self::Context, challenge: &[u8]) -> Option<()> {
		(*cx >= System::block_number().saturating_sub(3)).then_some(())?;
		Self::generate(cx).eq(challenge).then_some(())
	}

	fn generate(cx: &Self::Context) -> Challenge {
		BlakeTwo256::hash(&cx.to_le_bytes()).0
	}
}

// pub type WebAuthn = pass_webauthn::Authenticator<UnincludedBlockChallenger,
// AuthorityFromPalletId<PassPalletId>>;
pub type Dummy = fc_traits_authn::util::dummy::Dummy<AuthorityFromPalletId<PassPalletId>>;

composite_authenticator!(
	pub Pass<AuthorityFromPalletId<PassPalletId>> {
		// WebAuthn,
		Dummy,
	}
);

impl pallet_pass::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type WeightInfo = pallet_pass::SubstrateWeight<Self>;
	type Authenticator = PassAuthenticator;
	type PalletsOrigin = OriginCaller;
	type PalletId = PassPalletId;
	type MaxSessionDuration = ConstU32<{ 15 * MINUTES }>;
	type RegisterOrigin = EitherOf<
		// Root never pays
		EnsureRootWithSuccess<Self::AccountId, NeverPays>,
		// EitherOf<
		// 	// Communities never pay
		// 	MapSuccess<AsSignedByCommunity<Runtime>, NeverPays>,
		// Signed users must deposit ED for creating a pass account
		pallet_pass::EnsureSignedPays<
			Runtime,
			<Runtime as pallet_balances::Config>::ExistentialDeposit,
			TreasuryAccount,
		>,
		// >,
	>;

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = PassBenchmarkHelper;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct PassBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl pallet_pass::BenchmarkHelper<Test> for PassBenchmarkHelper {
	fn register_origin() -> frame_system::pallet_prelude::OriginFor<Test> {
		RuntimeOrigin::root()
	}

	fn device_attestation(device_id: fc_traits_authn::DeviceId) -> pallet_pass::DeviceAttestationOf<Test, ()> {
		todo!("Insert Dummy authenticator that works with benchmarks first")
	}

	fn credential(user_id: HashedUserId) -> pallet_pass::CredentialOf<Test, ()> {
		todo!("Insert Dummy authenticator that works with benchmarks first")
	}
}
