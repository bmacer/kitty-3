#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap,
	RuntimeDebug
};

use frame_support::{traits::Randomness};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty(pub [u8; 16]);

impl Kitty {
	fn gender(&self) -> Gender {
		if self.0[0] % 2 == 0 {
			return Gender::Male
		}
		Gender::Female
	}
}

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq)]
pub enum Gender {
	Male,
	Female,
}

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_error! {
	pub enum Error for Module<T: Config> {
		NoneValue,
		StorageOverflow,
		KittyBreedFailedSameGender,
		KittyBreedFailureIsNotOwner,
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Config>::AccountId,
	{
		/// A kitty is created. \[owner, kitty_id, kitty\]
		KittyCreated(AccountId, u32, Kitty),
		/// A kitty is bred. \[owner, kitty_id, kitty\]
		KittyBreedSucceessful(AccountId, u32, Kitty),
		// GimmeGender(Gender),
		/// A kitty is transferred. \[from_account, to_account, kitty_id, kitty\]
		KittyTransferred(AccountId, AccountId, u32, Kitty),

	}
}

decl_storage! {
	trait Store for Module<T: Config> as Kitties {
		pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
		pub NextKittyId get(fn next_kitty_id): u32;
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		#[weight = 0]
		// Kitties::breed(Origin::signed(1), male_cat, female_cat);
		pub fn breed(origin, m: u32, f: u32) {
			let sender = ensure_signed(origin)?;
			let parent_one = Self::kitties(&sender, m);
			let parent_two = Self::kitties(&sender, f);
			match parent_one {
				None => {
					return Err(Error::<T>::KittyBreedFailureIsNotOwner)?;
				},
				Some(m) => {
					match parent_two {
						None => {
							return Err(Error::<T>::KittyBreedFailureIsNotOwner)?;
						},
						Some(f) => {
							if m.gender() == f.gender() {
								return Err(Error::<T>::KittyBreedFailedSameGender)?;
							} else {
								let payload = (m, f);
								let dna = payload.using_encoded(blake2_128);
								let kitty = Kitty(dna);
								let kitty_id = Self::next_kitty_id();
								<Kitties<T>>::insert(&sender, kitty_id, kitty.clone());
								NextKittyId::put(kitty_id + 1);
								Self::deposit_event(RawEvent::KittyBreedSucceessful(sender, kitty_id, kitty));
							}
						}
					}
				}
			}
		}


		#[weight = 2]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let payload = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				Self::next_kitty_id(),
				<frame_system::Module<T>>::extrinsic_index(),
			);
			
			let dna = payload.using_encoded(blake2_128);

			// Create and store kitty
			let kitty = Kitty(dna);
			let kitty_id = Self::next_kitty_id();
			<Kitties<T>>::insert(&sender, kitty_id, kitty.clone());

			// Update kitty ID
			NextKittyId::put(kitty_id + 1);

			// Send event
			Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty));
		}

		// #[weight = 2]
		// pub fn get_gender(origin, kid: u32) {
		// 	let sender = ensure_signed(origin)?;
		// 	let kitty = Self::kitties(&sender, kid).unwrap();
		// 	let x = Some(kitty.gender());
		// 	Self::deposit_event(RawEvent::GimmeGender(kitty.gender()));
		// }

		#[weight = 3]
		pub fn transfer(origin, dest: T::AccountId, kid: u32) {
			let sender = ensure_signed(origin)?;
			match Self::kitties(&sender, kid) {
				None => {return Err (Error::<T>::KittyBreedFailureIsNotOwner)?},
				Some(k) => {
					<Kitties<T>>::remove(&sender, kid);
					<Kitties<T>>::insert(&dest, kid, &k);

					Self::deposit_event(RawEvent::KittyTransferred(sender, dest, kid, k));
				}
			}
		}

	}
}