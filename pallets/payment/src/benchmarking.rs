use super::*;

use crate::{Pallet as Payment, Payment as PaymentStore};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use orml_traits::MultiCurrency;
use virto_primitives::Asset;

const SEED: u32 = 0;
const INITIAL_AMOUNT: u128 = 100;
const SOME_AMOUNT: u128 = 80;

fn get_currency_id() -> Asset {
	Asset::default()
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	where_clause { where T::Asset: MultiCurrency<
		<T as frame_system::Config>::AccountId,
		CurrencyId = Asset, Balance = u128
	>
}
	// create a new payment succesfully
	pay {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent = account("recipient", 0, SEED);
	}: _(RawOrigin::Signed(caller.clone()), recipent, get_currency_id(), SOME_AMOUNT)
	verify {
		assert_last_event::<T>(Event::<T>::PaymentCreated { from : caller, asset: get_currency_id(), amount: SOME_AMOUNT}.into());
	}

	// create a new payment with remark sucessfully
	pay_with_remark {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent = account("recipient", 0, SEED);
	}: _(RawOrigin::Signed(caller.clone()), recipent, get_currency_id(), SOME_AMOUNT, vec![1u8, 50].into())
	verify {
		assert_last_event::<T>(Event::<T>::PaymentCreated { from: caller, asset: get_currency_id(), amount: SOME_AMOUNT}.into());
	}

	// release an existing payment succesfully
	release {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent : T::AccountId = account("recipient", 0, SEED);
		Payment::<T>::pay(RawOrigin::Signed(caller.clone()).into(), recipent.clone(), get_currency_id(), SOME_AMOUNT)?;
	}: _(RawOrigin::Signed(caller.clone()), recipent.clone())
	verify {
		assert_last_event::<T>(Event::<T>::PaymentReleased { from: caller, to: recipent}.into());
	}

	// cancel an existing payment succesfully
	cancel {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent : T::AccountId = account("recipient", 0, SEED);
		Payment::<T>::pay(RawOrigin::Signed(caller.clone()).into(), recipent.clone(), get_currency_id(), SOME_AMOUNT)?;
	}: _(RawOrigin::Signed(recipent.clone()), caller.clone())
	verify {
		assert_last_event::<T>(Event::<T>::PaymentCancelled { from: caller, to: recipent}.into());
	}

	// resolve an existing payment to cancellation
	resolve_cancel_payment {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent : T::AccountId = account("recipient", 0, SEED);
		Payment::<T>::pay(RawOrigin::Signed(caller.clone()).into(), recipent.clone(), get_currency_id(), SOME_AMOUNT)?;
		let resolver = PaymentStore::<T>::get(caller.clone(), recipent.clone()).unwrap().resolver_account;
	}: _(RawOrigin::Signed(resolver), caller.clone(), recipent.clone())
	verify {
		assert_last_event::<T>(Event::<T>::PaymentCancelled { from: caller, to: recipent}.into());
	}

	// resolve an existing payment to release
	resolve_release_payment {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent : T::AccountId = account("recipient", 0, SEED);
		Payment::<T>::pay(RawOrigin::Signed(caller.clone()).into(), recipent.clone(), get_currency_id(), SOME_AMOUNT)?;
		let resolver = PaymentStore::<T>::get(caller.clone(), recipent.clone()).unwrap().resolver_account;
	}: _(RawOrigin::Signed(resolver), caller.clone(), recipent.clone())
	verify {
		assert_last_event::<T>(Event::<T>::PaymentReleased { from: caller, to: recipent}.into());
	}

	// creator of payment creates a refund request
	request_refund {
		let caller = whitelisted_caller();
		let _ = T::Asset::deposit(get_currency_id(), &caller, INITIAL_AMOUNT);
		let recipent : T::AccountId = account("recipient", 0, SEED);
		Payment::<T>::pay(RawOrigin::Signed(caller.clone()).into(), recipent.clone(), get_currency_id(), SOME_AMOUNT)?;
	}: _(RawOrigin::Signed(caller.clone()), recipent.clone())
	verify {
		assert_last_event::<T>(Event::<T>::PaymentCreatorRequestedRefund { from: caller, to: recipent, expiry: 601u32.into() }.into());
	}
}

impl_benchmark_test_suite!(Payment, crate::mock::new_test_ext(), crate::mock::Test,);
