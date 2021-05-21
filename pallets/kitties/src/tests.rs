
use super::*;

use crate::{mock::{Kitties, Origin, new_test_ext, Test, System}};
use frame_support::{assert_err};
// use frame_support::{assert_ok, assert_noop}
// use std::{thread, time};
// use sp_core::Pair;

fn setup_blocks(blocks: u64) {
	let mut parent_hash = System::parent_hash();

	for i in 1 .. (blocks + 1) {
		System::initialize(
			&i,
			&parent_hash,
			&Default::default(),
			frame_system::InitKind::Full,
		);
		let header = System::finalize();
		parent_hash = header.hash();
		System::set_block_number(header.number);
	}
}

#[test]
fn trivial() {
    assert_eq!(2+2, 4);
}

#[test]
fn create_three_kitties_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Kitties::next_kitty_id(), 0);
		setup_blocks(100);
		for _ in 0..3 {
			Kitties::create(Origin::signed(1)).unwrap();
		}
		assert_eq!(Kitties::next_kitty_id(), 3);
	})
}

#[test]
fn get_gender_works() {
	// Create 10 cats, check gender against even/odd-ness of first u8
	new_test_ext().execute_with(|| {
		setup_blocks(100);
		for _ in 1..10 {
			Kitties::create(Origin::signed(1)).unwrap();
		}
		for i in 0..9 {
			let kitty = Kitties::kitties(1, &i).unwrap();
			if kitty.0[0] % 2 == 0 {
				assert_eq!(kitty.gender(), Gender::Male);
			} else {
				assert_eq!(kitty.gender(), Gender::Female);
			}
		}
	})
}

#[test]
fn breed_kitty_works() {
	// Create two kitties with owner 1, breed them, check that it exists.
	new_test_ext().execute_with(|| {
		setup_blocks(100);
		Kitties::create(Origin::signed(1)).unwrap();
		Kitties::create(Origin::signed(1)).unwrap();
		Kitties::breed(Origin::signed(1), 0, 1).unwrap();
		assert_eq!(Kitties::kitties(1, 2).is_some(), true);
	});
}

#[test]
fn breed_same_gender_kitty_fails() {
	// We create 10 cats to ensure we'll have 2 of the same gender.  Then we pick the first and loop the rest until
	// we find a same-gender match.  Then we try to breed them, expecting KittyBreedFailedSameGender error.
	new_test_ext().execute_with(|| {
		setup_blocks(100);
		for _ in 1..10 {
			Kitties::create(Origin::signed(1)).unwrap();
		}
		let first_parent = Kitties::kitties(1, 0).unwrap();
		for i in 1..9 {
			let second_parent = Kitties::kitties(1, &i).unwrap();
			if first_parent.gender() == second_parent.gender() {
				assert_err!(Kitties::breed(Origin::signed(1), 0, i), Error::<Test>::KittyBreedFailedSameGender);
				break;
			}
		}
	});
}

#[test]
fn breed_someone_elses_kitty_fails() {
	// Create two kitties with owner 1, try to breed with owner 2
	new_test_ext().execute_with(|| {
		setup_blocks(100);
		Kitties::create(Origin::signed(1)).unwrap();
		Kitties::create(Origin::signed(1)).unwrap();
		assert_err!(Kitties::breed(Origin::signed(2), 0, 1), Error::<Test>::KittyBreedFailureIsNotOwner);
	});
}

#[test]
fn transfer_succeeds() {
	new_test_ext().execute_with(|| {
		setup_blocks(100);
		Kitties::create(Origin::signed(1)).unwrap();
		Kitties::transfer(Origin::signed(1), 2, 0).unwrap();
		assert_eq!(Kitties::kitties(2, 0).is_some(), true);
	});
}

#[test]
fn transfer_fails_wrong_owner() {
	new_test_ext().execute_with(|| {
		setup_blocks(100);
		Kitties::create(Origin::signed(1)).unwrap();
		assert_err!(Kitties::transfer(Origin::signed(2), 3, 1), Error::<Test>::KittyBreedFailureIsNotOwner); 
	});
}
