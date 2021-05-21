// Check if Kitties list is empty
// Create new list with one entry if so
// Append if not





#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Encode, Decode};


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame


// use account_set::AccountSet;
// use frame_support::{
// 	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
// };
// use frame_system::ensure_signed;
// use sp_std::collections::btree_set::BTreeSet;
use sp_std::prelude::*;





use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[derive(Encode, Decode, Debug, Default)]
#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Kitty {
	pub id: u32,
}


/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {

	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		Kitties get(fn get_kitties): Option<Vec<u32>>;
		TotalKitties get(fn total_kitties): u32;
		TotalKittyCats get(fn total_kitty_kats): Option<u32>;
		SingleKitty get(fn one_kitty): Option<Kitty>;
		KittyList get(fn kitty_list): Option<Vec<Kitty>>;
		
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_single_kitty(origin, val: u32) -> dispatch::DispatchResult {
			let k = Kitty { id: val };
			let  x = SingleKitty::get();
			// println!("XXX: {:?}", x);
			match x  {
				Some(v) => {
					// println!("some {:?}", v);
					SingleKitty::put(k)
				},
				None => {
					// println!("NONE");
					SingleKitty::put(k)
				}
			}
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_or_append_to_kitty_list(origin, val: u32) -> dispatch::DispatchResult {
			let k = Kitty { id: val };
			let  x = KittyList::get();
			// println!("XXX: {:?}", x);
			match x  {
				Some(mut v) => {
					// println!("some {:?}", v);
					let mut kitty_vec = Vec::<Kitty>::new();
					v.push(k);
					KittyList::put(v);
				},
				None => {
					let mut kitty_vec = Vec::<Kitty>::new();
					kitty_vec.push(k);
					KittyList::put(kitty_vec);
				}
			}
			TotalKitties::put(TotalKitties::get()+1);
			TotalKittyCats::put(TotalKitties::get()+1);
			Ok(())
		}
		// 	Ok(SingleKitty::get().unwrap())
			// match SingleKitty::get() {
			// 	Some(v) => {
			// 		println!("ok: {:?}", v);
			// 		Ok(())
			// 	},
			// 	None => {
			// 		println!("panicking!");
			// 		Err(())
			// 	},
			// }
		// }

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);
			let mut something_vec = match Kitties::get() {
				Some(v) => v,
				None => Vec::<u32>::new(),
			};
			let k = Kitty { id: 23 };
			let s = "abc";
			something_vec.push(23);
			Kitties::put(something_vec);
			SingleKitty::put(k);
			let o = SingleKitty::get();
			// println!("o: {:?}", o);
			// println!("v: {:?}", v);
			// let mut w = Kitties::get().unwrap();
			// println!("w: {:?}", w);


			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
