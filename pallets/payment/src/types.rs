#![allow(unused_qualifications)]
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{DispatchResult, Percent};
use sp_std::{ops::Deref, vec::Vec};

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, TypeInfo)]
pub struct Remark(Vec<u8>); // TODO : switch to BoundedVec

impl MaxEncodedLen for Remark {
	fn max_encoded_len() -> usize {
		50 // TODO : switch to BoundedVec - temp fix
	}
}

impl Deref for Remark {
	type Target = Vec<u8>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<Vec<u8>> for Remark {
	fn from(v: Vec<u8>) -> Remark {
		Remark(v)
	}
}

/// The PaymentDetail struct stores information about the payment/escrow
/// A "payment" in virto network is similar to an escrow, it is used to guarantee proof of funds
/// and can be released once an agreed upon condition has reached between the payment creator
/// and recipient. The payment lifecycle is tracked using the state field.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PaymentDetail<Asset, Amount, Account, BlockNumber> {
	/// type of asset used for payment
	pub asset: Asset,
	/// amount of asset used for payment
	pub amount: Amount,
	/// incentive amount that is credited to creator for resolving
	pub incentive_amount: Amount,
	/// enum to track payment lifecycle [Created, NeedsReview]
	pub state: PaymentState<BlockNumber>,
	/// account that can settle any disputes created in the payment
	pub resolver_account: Account,
	/// fee charged and recipient account details
	pub fee_detail: Option<(Account, Amount)>,
	/// remarks to give context to payment
	pub remark: Option<Remark>,
}

/// The `PaymentState` enum tracks the possible states that a payment can be in.
/// When a payment is 'completed' or 'cancelled' it is removed from storage and hence not tracked by a state.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaymentState<BlockNumber> {
	/// Amounts have been reserved and waiting for release/cancel
	Created,
	/// A judge needs to review and release manually
	NeedsReview,
	/// The user has requested refund and will be processed by `BlockNumber`
	RefundRequested(BlockNumber),
}

/// trait that defines how to create/release payments for users
pub trait PaymentHandler<Account, Asset, Amount, BlockNumber> {
	/// Attempt to reserve an amount of the given asset from the caller
	/// If not possible then return Error. Possible reasons for failure include:
	/// - User does not have enough balance.
	fn create_payment(
		from: Account,
		to: Account,
		asset: Asset,
		amount: Amount,
		remark: Option<Remark>,
	) -> DispatchResult;

	/// Attempt to transfer an amount of the given asset from the given payment_id
	/// If not possible then return Error. Possible reasons for failure include:
	/// - The payment does not exist
	/// - The unreserve operation fails
	/// - The transfer operation fails
	fn release_payment(from: Account, to: Account) -> DispatchResult;

	/// Attempt to cancel a payment in Created state. This will set the payment
	/// state to cancel and release the reserved amount back to the creator.
	/// If not possible then return Error. Possible reasons for failure include:
	/// - The payment does not exist
	/// - The payment is not in Created state
	/// - The unreserve operation fails
	fn cancel_payment(from: Account, to: Account) -> DispatchResult;

	/// Attempt to fetch the details of a payment from the given payment_id
	/// Possible reasons for failure include:
	/// - The payment does not exist
	fn get_payment_details(
		from: Account,
		to: Account,
	) -> Option<PaymentDetail<Asset, Amount, Account, BlockNumber>>;
}

/// DisputeResolver trait defines how to create/assing judges for solving payment disputes
pub trait DisputeResolver<Account> {
	/// Get a DisputeResolver (Judge) account
	fn get_origin() -> Account;
}

/// Fee Handler trait that defines how to handle marketplace fees to every payment/swap
pub trait FeeHandler<Asset, Amount, Account, BlockNumber> {
	/// Get the distribution of fees to marketplace participants
	fn apply_fees(
		from: &Account,
		to: &Account,
		detail: &PaymentDetail<Asset, Amount, Account, BlockNumber>,
	) -> (Account, Percent);
}
