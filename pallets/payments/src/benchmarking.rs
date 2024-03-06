use super::*;
#[allow(unused)]
use crate::{types::*, Pallet as Payments};
use frame_benchmarking::{account, v2::*};
use frame_support::{
	traits::{
		fungibles::{Inspect, Mutate},
		Get,
	},
	BoundedVec,
};

use frame_system::RawOrigin;
use log;
use sp_runtime::{traits::Zero, Percent};
use sp_std::vec;

fn assert_has_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

fn create_accounts<T: Config>() -> (T::AccountId, T::AccountId, AccountIdLookupOf<T>, AccountIdLookupOf<T>) {
	let sender: T::AccountId = account("Alice", 0, 10);
	let beneficiary: T::AccountId = account("Bob", 0, 11);
	let sender_lookup = T::Lookup::unlookup(sender.clone());
	let beneficiary_lookup = T::Lookup::unlookup(beneficiary.clone());

	(sender, beneficiary, sender_lookup, beneficiary_lookup)
}

fn create_and_mint_asset<T: Config>(
	sender: &T::AccountId,
	beneficiary: &T::AccountId,
	asset: &AssetIdOf<T>,
) -> Result<(), BenchmarkError> {
	T::BenchmarkHelper::create_asset(asset.clone(), sender.clone(), true, <BalanceOf<T>>::from(1u32));
	T::Assets::mint_into(asset.clone(), &sender, <BalanceOf<T>>::from(10000000u32))?;
	T::Assets::mint_into(asset.clone(), &beneficiary, <BalanceOf<T>>::from(10000000u32))?;

	Ok(())
}

fn create_payment<T: Config>(
	amount: &BalanceOf<T>,
	asset: &AssetIdOf<T>,
	remark: Option<BoundedDataOf<T>>,
) -> Result<
	(
		T::PaymentId,
		T::AccountId,
		T::AccountId,
		AccountIdLookupOf<T>,
		AccountIdLookupOf<T>,
	),
	BenchmarkError,
> {
	let (sender, beneficiary, sender_lookup, beneficiary_lookup) = create_accounts::<T>();
	create_and_mint_asset::<T>(&sender, &beneficiary, &asset)?;

	let (payment_id, payment_detail) = Payments::<T>::create_payment(
		&sender,
		beneficiary.clone(),
		asset.clone(),
		amount.clone(),
		PaymentState::Created,
		T::IncentivePercentage::get(),
		remark.as_ref().map(|x| x.as_slice()),
	)?;

	// reserve funds for payment
	Payments::<T>::reserve_payment_amount(&sender, payment_detail)?;

	log::info!("reserve_payment_amount executed");

	// TODO: check storage items

	Ok((payment_id, sender, beneficiary, sender_lookup, beneficiary_lookup))
}

#[benchmarks(
	where
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId: Zero
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn pay(q: Linear<1, { T::MaxRemarkLength::get() }>) -> Result<(), BenchmarkError> {
		let (sender, beneficiary, _, beneficiary_lookup) = create_accounts::<T>();

		let asset: AssetIdOf<T> = <AssetIdOf<T>>::zero();
		create_and_mint_asset::<T>(&sender, &beneficiary, &asset)?;
		let amount = <BalanceOf<T>>::from(100000_u32);

		let remark: Option<BoundedDataOf<T>> = if q == 0 {
			None
		} else {
			Some(BoundedVec::try_from(vec![1 as u8; q as usize]).unwrap())
		};
		let expected_payment_id = T::PaymentId::from_number(1);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(sender.clone()),
			beneficiary_lookup,
			asset.clone(),
			amount,
			remark.clone(),
		);

		assert_has_event::<T>(
			Event::PaymentCreated {
				payment_id: expected_payment_id,
				asset,
				amount,
				remark,
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn release() -> Result<(), BenchmarkError> {
		let amount = <BalanceOf<T>>::from(100000_u32);
		let asset = <AssetIdOf<T>>::zero();
		let (payment_id, sender, _beneficiary, _, beneficiary_lookup) = create_payment::<T>(&amount, &asset, None)?;

		log::info!("beneficiary_lookup: {:?}", beneficiary_lookup);

		#[extrinsic_call]
		_(RawOrigin::Signed(sender), payment_id);

		assert_has_event::<T>(
			Event::PaymentReleased {
				payment_id: T::PaymentId::from_number(3),
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn cancel() -> Result<(), BenchmarkError> {
		let amount = <BalanceOf<T>>::from(100000_u32);
		let asset = <AssetIdOf<T>>::zero();
		let (payment_id, _sender, beneficiary, _sender_lookup, _beneficiary_lookup) =
			create_payment::<T>(&amount, &asset, None)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(beneficiary.clone()), payment_id);

		assert_has_event::<T>(
			Event::PaymentCancelled {
				payment_id: T::PaymentId::from_number(4),
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn request_refund() -> Result<(), BenchmarkError> {
		let amount = <BalanceOf<T>>::from(100000_u32);
		let asset = <AssetIdOf<T>>::zero();
		let (payment_id, sender, _beneficiary, _sender_lookup, _beneficiary_lookup) =
			create_payment::<T>(&amount, &asset, None)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(sender.clone()), payment_id);

		let current_block = frame_system::Pallet::<T>::block_number();
		let expiry = current_block + T::CancelBufferBlockLength::get();

		assert_has_event::<T>(Event::PaymentCreatorRequestedRefund { payment_id, expiry }.into());
		Ok(())
	}

	#[benchmark]
	fn dispute_refund() -> Result<(), BenchmarkError> {
		let amount = <BalanceOf<T>>::from(100000_u32);
		let asset = <AssetIdOf<T>>::zero();
		let (payment_id, sender, beneficiary, _sender_lookup, _beneficiary_lookup) =
			create_payment::<T>(&amount, &asset, None)?;

		assert!(Payments::<T>::request_refund(RawOrigin::Signed(sender.clone()).into(), payment_id).is_ok());

		#[extrinsic_call]
		_(RawOrigin::Signed(beneficiary.clone()), payment_id);

		assert_has_event::<T>(
			Event::PaymentRefundDisputed {
				payment_id: T::PaymentId::from_number(6),
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn resolve_dispute() -> Result<(), BenchmarkError> {
		let amount = <BalanceOf<T>>::from(100000_u32);
		let asset = <AssetIdOf<T>>::zero();
		let (payment_id, sender, beneficiary, _sender_lookup, _beneficiary_lookup) =
			create_payment::<T>(&amount, &asset, None)?;

		assert!(Payments::<T>::request_refund(RawOrigin::Signed(sender.clone()).into(), payment_id).is_ok());

		assert!(Payments::<T>::dispute_refund(RawOrigin::Signed(beneficiary.clone()).into(), payment_id).is_ok());

		let dispute_result = DisputeResult {
			percent_beneficiary: Percent::from_percent(90),
			in_favor_of: Role::Sender,
		};

		#[extrinsic_call]
		_(RawOrigin::Root, payment_id, dispute_result);

		assert_has_event::<T>(
			Event::PaymentDisputeResolved {
				payment_id: T::PaymentId::from_number(7),
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn request_payment() -> Result<(), BenchmarkError> {
		let (sender, beneficiary, sender_lookup, _beneficiary_lookup) = create_accounts::<T>();
		let asset: AssetIdOf<T> = <AssetIdOf<T>>::zero();
		create_and_mint_asset::<T>(&sender, &beneficiary, &asset)?;
		let amount = <BalanceOf<T>>::from(100000_u32);

		#[extrinsic_call]
		_(RawOrigin::Signed(beneficiary.clone()), sender_lookup, asset, amount);

		assert_has_event::<T>(
			Event::PaymentRequestCreated {
				payment_id: T::PaymentId::from_number(8),
			}
			.into(),
		);
		Ok(())
	}

	#[benchmark]
	fn accept_and_pay() -> Result<(), BenchmarkError> {
		let (sender, beneficiary, sender_lookup, _beneficiary_lookup) = create_accounts::<T>();
		let asset: AssetIdOf<T> = <AssetIdOf<T>>::zero();
		create_and_mint_asset::<T>(&sender, &beneficiary, &asset)?;
		let amount = <BalanceOf<T>>::from(100000_u32);

		assert!(Payments::<T>::request_payment(
			RawOrigin::Signed(beneficiary.clone()).into(),
			sender_lookup,
			asset,
			amount
		)
		.is_ok());

		#[extrinsic_call]
		_(RawOrigin::Signed(sender.clone()), T::PaymentId::from_number(9));

		assert_has_event::<T>(
			Event::PaymentRequestCreated {
				payment_id: T::PaymentId::from_number(8),
			}
			.into(),
		);
		Ok(())
	}

	impl_benchmark_test_suite!(Payments, crate::mock::new_test_ext(), crate::mock::Test);
}
