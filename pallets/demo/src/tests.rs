use crate::{mock::*, Error};
// use crate::Pallet as Demo;
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_create_a_student_with_normal_name_and_age() {
	new_test_ext().execute_with(|| {
		assert_ok!(DemoModule::create_student(Origin::signed(1), b"shyoski".to_vec(), 24));
		assert_eq!(DemoModule::student_id(), 1);
	});
}

#[test]
fn create_student_should_false_with_age_lower_than_20() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			DemoModule::create_student(Origin::signed(1), b"shyoski".to_vec(), 20),
			Error::<Test>::TooYoung
		);
	});
}