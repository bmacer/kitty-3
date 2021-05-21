use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use std::{thread, time};

#[test]
fn multi_kitty_works() {
	new_test_ext().execute_with(|| {
		let kl = TemplateModule::kitty_list();
		assert_eq!(kl.is_none(), true);
		TemplateModule::create_or_append_to_kitty_list(Origin::signed(1), 12);
		let kl = TemplateModule::kitty_list();
		println!("kl: {:?}", kl);
		assert_eq!(kl.unwrap().len(), 1);
		TemplateModule::create_or_append_to_kitty_list(Origin::signed(1), 13);
		let kl = TemplateModule::kitty_list();
		println!("kl: {:?}", kl);
		assert_eq!(kl.unwrap().len(), 2);

		let total = TemplateModule::total_kitties();
		assert_eq!(total,2);
		
	})
}



#[test]
fn single_kitty_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		// println!("get one {:?}", TemplateModule::one_kitty());
		let x = TemplateModule::one_kitty();
		assert_eq!(x.is_some(), false);
		// println!("x: {:?}",  x);

		TemplateModule::create_single_kitty(Origin::signed(1), 123);
		let ten_seconds = time::Duration::from_secs(10);

		TemplateModule::create_single_kitty(Origin::signed(1), 1234);
		// thread::sleep(ten_seconds);

		TemplateModule::create_single_kitty(Origin::signed(2), 12345);

		let x = TemplateModule::one_kitty();
		println!("x: {:?}", x);
		assert_eq!(x.is_some(), true);
		// println!("x after: {:?}",  x);


		// SingleKitty get(fn one_kitty): Option<Kitty>;

		// match TemplateModule::get_single_kitty(Origin::signed(1)) {
		// 	Ok(v) => {
		// 		println!("OK  OK {:?}", v);
		// 	},
		// 	Err(e) => {
		// 		println!("errrrr");
		// 	}
		// };
		// TemplateModule::create_single_kitty(Origin::signed(1));
		// assert_eq!(TemplateModule::get_single_kitty(Origin::signed(1)), true);
	});
}
#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		TemplateModule::do_something(Origin::signed(1), 33);
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
