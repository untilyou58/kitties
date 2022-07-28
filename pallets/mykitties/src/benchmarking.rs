//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use frame_benchmarking::vec;

benchmarks! { 
	// tên của benchmark
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas : Vec<u8> = b"lienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlq".to_vec();

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dnas)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	transfer {
		let alice: T::AccountId = whitelisted_caller();
		let bob: T::AccountId = account("bob", 0, 0);
		
		let dnas : Vec<u8> = b"lienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlq".to_vec();
		let dnas_hash = <T::Hashing as Hash>::hash(&dnas);

	}: transfer (RawOrigin::Signed(alice), bob.clone(), dnas_hash)
		// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
		verify {
			assert_eq!(KittyId::<T>::get(), 2);
		}

	// verify {
	// 	assert_eq!(KittyOwner::<T>::get(1), bob);
	// }
 
	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}