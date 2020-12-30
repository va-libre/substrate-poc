#![allow(
    // `construct_runtime` can't de "fixed"
    clippy::large_enum_variant
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

mod opaque {
    use crate::{impl_opaque_keys, Aura, Grandpa};
    use alloc::vec::Vec;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

use alloc::{boxed::Box, vec::Vec};
use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        Weight,
    },
};
use frame_system::EnsureRoot;
use pallet_grandpa::{
    fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, impl_opaque_keys,
    traits::{Block as BlockT, IdentityLookup, NumberFor, Saturating},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, Perbill,
};
use sp_version::RuntimeVersion;
use valiu_node_commons::Asset;
use valiu_node_runtime_types::{
    AccountData, AccountId, Balance, BlockNumber, Hash, Hashing, Header, Index, Signature,
};

const MILLISECS_PER_BLOCK: u64 = 6000;
const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const VERSION: RuntimeVersion = RuntimeVersion {
    apis: RUNTIME_API_VERSIONS,
    authoring_version: 1,
    impl_name: create_runtime_str!("vln-node"),
    impl_version: 1,
    spec_name: create_runtime_str!("vln-node"),
    spec_version: 1,
    transaction_version: 1,
};

pub type Block = valiu_node_runtime_types::Block<Call, Runtime>;
type Executive = valiu_node_runtime_types::Executive<AllModules, Call, Runtime>;
type UncheckedExtrinsic = valiu_node_runtime_types::UncheckedExtrinsic<Call, Runtime>;

parameter_types! {
    pub MaximumExtrinsicWeight: Weight = AvailableBlockRatio::get().saturating_sub(Perbill::from_percent(10)) * MaximumBlockWeight::get();
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const BlockHashCount: BlockNumber = 2400;
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
    /// Assume 10% of weight for average on_initialize calls.
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
    pub const OffchainUnsignedGracePeriod: u32 = 5;
    pub const OffchainUnsignedInterval: u32 = 128;
    pub const TransactionByteFee: Balance = 1;
    pub const Version: RuntimeVersion = VERSION;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
   pub enum Runtime
   where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        Aura: pallet_aura::{Config<T>, Inherent, Module},
        Grandpa: pallet_grandpa::{Call, Config, Event, Module, Storage},
        LiquidityProvider: pallet_liquidity_provider::{Call, Event<T>, Module, Storage},
        ProviderMembers: pallet_membership::{Call, Config<T>, Event<T>, Module},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Call, Module, Storage},
        Sudo: pallet_sudo::{Call, Config<T>, Event<T>, Module, Storage},
        System: frame_system::{Call, Config, Event<T>, Module, Storage},
        Timestamp: pallet_timestamp::{Call, Inherent, Module, Storage},
        Tokens: orml_tokens::{Config<T>, Event<T>, Module},
    }
);

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }

        fn version() -> RuntimeVersion {
            VERSION
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            RandomnessCollectiveFlip::random_seed()
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }

        fn slot_duration() -> u64 {
            Aura::slot_duration()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }

        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }

        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Module as SystemBench;
            impl frame_system_benchmarking::Trait for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);
            add_benchmark!(params, batches, pallet_liquidity_provider, LiquidityProvider);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

impl frame_system::Trait for Runtime {
    type AccountData = AccountData;
    type AccountId = AccountId;
    /// Portion of the block weight that is available to all normal transactions.
    type AvailableBlockRatio = AvailableBlockRatio;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// The weight of the overhead invoked on the block import process, independent of the
    /// extrinsics included in that block.
    type BlockExecutionWeight = BlockExecutionWeight;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    type BlockNumber = BlockNumber;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The ubiquitous event type.
    type Event = Event;
    /// The base weight of any extrinsic processed by the runtime, independent of the
    /// logic of that extrinsic. (Signature verification, nonce increment, fee, etc...)
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    type Hash = Hash;
    type Hashing = Hashing;
    type Header = Header;
    type Index = Index;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = IdentityLookup<AccountId>;
    /// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
    type MaximumBlockLength = MaximumBlockLength;
    /// Maximum weight of each block.
    type MaximumBlockWeight = MaximumBlockWeight;
    /// The maximum weight that a single extrinsic of `Normal` dispatch class can have,
    /// idependent of the logic of that extrinsics. (Roughly max block weight - average on
    /// initialize cost).
    type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Version of the runtime.
    type Version = Version;
}

impl<LC> frame_system::offchain::SendTransactionTypes<LC> for Runtime
where
    Call: From<LC>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = AccountId;
    type Signature = Signature;
}

impl pallet_aura::Trait for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_grandpa::Trait for Runtime {
    type Call = Call;

    type Event = Event;

    type HandleEquivocation = ();

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerProofSystem = ();

    type WeightInfo = ();
}

impl pallet_timestamp::Trait for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_sudo::Trait for Runtime {
    type Call = Call;
    type Event = Event;
}

impl pallet_membership::Trait for Runtime {
    type AddOrigin = EnsureRoot<AccountId>;
    type Event = Event;
    type MembershipChanged = ();
    type MembershipInitialized = ();
    type PrimeOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type ResetOrigin = EnsureRoot<AccountId>;
    type SwapOrigin = EnsureRoot<AccountId>;
}

// Configure the pallet providers in pallets/providers.
impl pallet_liquidity_provider::Trait for Runtime {
    type Asset = orml_tokens::Module<Runtime>;
    type Collateral = orml_tokens::Module<Runtime>;
    type Event = Event;
    type OffchainAuthority = OffchainAppCrypto;
    type OffchainUnsignedGracePeriod = OffchainUnsignedGracePeriod;
    type OffchainUnsignedInterval = OffchainUnsignedInterval;
    type WeightInfo = pallet_liquidity_provider::DefaultWeightInfo;
}

impl orml_tokens::Trait for Runtime {
    type Amount = i64;
    type Balance = Balance;
    type CurrencyId = Asset;
    type Event = Event;
    type OnReceived = ();
    type WeightInfo = ();
}

#[derive(Debug)]
pub struct OffchainAppCrypto;

impl frame_system::offchain::AppCrypto<AccountId, Signature> for OffchainAppCrypto {
    type GenericPublic = AccountId;
    type GenericSignature = Signature;
    type RuntimeAppPublic = pallet_liquidity_provider::Public;
}

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> sp_version::NativeVersion {
    sp_version::NativeVersion {
        can_author_with: Default::default(),
        runtime_version: VERSION,
    }
}
