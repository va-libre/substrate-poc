use super::*;
use parity_scale_codec::Encode;

parameter_types! {
	pub const MaxRemarkLength: u8 = 50;
	pub const IncentivePercentage: Percent = Percent::from_percent(INCENTIVE_PERCENTAGE);
	pub const PaymentPalletId: PalletId = PalletId(*b"payments");
}

#[cfg(feature = "runtime-benchmarks")]
pub struct BenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_payments::BenchmarkHelper<AccountId, AssetIdForTrustBackedAssets, Balance> for BenchmarkHelper {
	fn create_asset(id: AssetIdForTrustBackedAssets, admin: AccountId, is_sufficient: bool, min_balance: Balance) {
		<Assets as frame_support::traits::tokens::fungibles::Create<AccountId>>::create(
			id,
			admin,
			is_sufficient,
			min_balance,
		)
		.unwrap();
	}
}

pub struct KreivoFeeHandler;

const MANDATORY_FEE: bool = true;
pub const SYSTEM_FEE: u8 = 1;
pub const SYSTEM_FEE_PERCENTAGE: Percent = Percent::from_percent(SYSTEM_FEE);
pub const INCENTIVE_PERCENTAGE: u8 = 10;

impl FeeHandler<Runtime> for KreivoFeeHandler {
	fn apply_fees(
		_asset: &AssetIdOf<Runtime>,
		_sender: &AccountId,
		_beneficiary: &AccountId,
		amount: &Balance,
		_remark: Option<&[u8]>,
	) -> Fees<Runtime> {
		let sender_fee: Vec<(AccountId, Balance, bool)> = vec![(
			TreasuryAccount::get(),
			SYSTEM_FEE_PERCENTAGE.mul_floor(*amount),
			MANDATORY_FEE,
		)];
		let beneficiary_fee: Vec<(AccountId, Balance, bool)> = vec![(
			TreasuryAccount::get(),
			SYSTEM_FEE_PERCENTAGE.mul_floor(*amount),
			MANDATORY_FEE,
		)];

		let sender_pays: FeeDetails<Runtime> = BoundedVec::try_from(sender_fee).unwrap();
		let beneficiary_pays: FeeDetails<Runtime> = BoundedVec::try_from(beneficiary_fee).unwrap();

		Fees {
			sender_pays,
			beneficiary_pays,
		}
	}
}

impl pallet_payments::PaymentId<Runtime> for virto_common::PaymentId {
	fn next(_: &AccountId, beneficiary: &AccountId) -> Option<Self> {
		let block: u32 = System::block_number();
		let idx = System::extrinsic_index()?;
		Some((block, idx, beneficiary.encode().as_slice()).into())
	}
}

impl pallet_payments::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Assets = Assets;
	type AssetsBalance = Balance;
	type PaymentId = virto_common::PaymentId;
	type FeeHandler = KreivoFeeHandler;
	type IncentivePercentage = IncentivePercentage;
	type MaxRemarkLength = MaxRemarkLength;
	type DisputeResolver = frame_system::EnsureRootWithSuccess<AccountId, TreasuryAccount>;
	type PalletId = PaymentPalletId;
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxDiscounts = ConstU32<10>;
	type MaxFees = ConstU32<50>;
	type RuntimeCall = RuntimeCall;
	type Scheduler = Scheduler;
	type Preimages = Preimage;
	type CancelBufferBlockLength = ConstU32<14400>; // 2 days
	type PalletsOrigin = OriginCaller;
	type WeightInfo = pallet_payments::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = BenchmarkHelper;
}
