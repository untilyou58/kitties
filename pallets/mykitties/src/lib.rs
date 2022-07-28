#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file &&to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use frame_support::pallet_prelude::*;
use frame_support::traits::Time;
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Hash;
use frame_support::dispatch::fmt;
use frame_support::traits::Randomness;
use frame_support::traits::Currency;
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
pub type Id = u32;
use sp_runtime::ArithmeticError;
use sp_runtime::traits::{SaturatedConversion};

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;
	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: T::Hash,
		pub price: BalanceOf<T>,
		pub gender: Gender,
		pub owner: T::AccountId,
		pub created_date: <<T as Config>::TimeProvider as Time>::Moment,
	}

	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("dna", &self.dna)
				.field("price", &self.price)
				.field("owner", &self.owner)
				.field("gender", &self.gender)
				.field("created_date", &self.created_date)
				.finish()
		}
	}


	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: Time;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
		
		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owned)]
	pub type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::Hash, T::MaxKittyOwned>, ValueQuery>;

	// Create a Nonce storage item.
	// https://docs.substrate.io/v3/runtime/storage#nonce
	#[pallet::storage]
	#[pallet::getter(fn next_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u128, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new kitty was successfully created.
		Created {
			kitty: T::Hash,
			owner: T::AccountId,
		},
		Transferred {
			from: T::AccountId,
			to: T::AccountId,
			kitty: T::Hash,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		ExceedMaxKittyOwned,
		TooManyOwned,
		NoKitty,
		NotOwner,
		TransferToSelf,
		CannotConvert,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	//extrinsic
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// ensure signed by root account
			let owner = ensure_signed(origin)?;
			// let nonce = <Nonce<T>>::get();
			// let ran = T::Randomness::random(&(created_date.as_nanos() + (price as u128) + nonce).encode());
			// let next_nonce = nonce.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			// let dna = ran.0.encode();
			let gender = Self::generate_gender(&dna)?;
			let created_date = T::TimeProvider::now();
			let dna = Self::generate_dna();

			let kitty = Kitty::<T> {
				dna: dna.clone(),
				price: 0u32.into(),
				gender,
				owner: owner.clone(),
				created_date,
			};

			log::info!("Kitty: {:?}", kitty);
			log::warn!("DNA: {:?}", dna);
			log::error!("Gender: {:?}", gender);

			let max_kitties = T::MaxKittyOwned::get();
			let get_kitties = KittiesOwned::<T>::get(&owner);
			ensure!((get_kitties.len() as u32) < max_kitties, Error::<T>::ExceedMaxKittyOwned);

			let _convert = T::TimeProvider::now().saturated_into::<u64>();
			let _convert_moment: <<T as Config>::TimeProvider as Time>::Moment = created_date.try_into().map_err(|_| Error::<T>::CannotConvert)?;

			// Check if kitty has not existed in storage map
			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);
			// Check if owner has exceed max kitty owned
			// <KittiesOwned<T>>::try_mutate(&owner, |kitty_owned| {
			// 	kitty_owned.try_push(kitty.dna.clone())
			// }).map_err(|_| Error::<T>::ExceedMaxKittyOwned)?;

			// Get current id and add it to 1
			let current_id = KittyId::<T>::get();
			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			// Append kitty to KittiesOwned
			// KittiesOwned::<T>::append(&owner, kitty.dna.clone());
			KittiesOwned::<T>::try_append(&owner, kitty.dna.clone()).map_err(|_| Error::<T>::NoKitty)?;

			// Write new kitty to storage and update idx
			Kitties::<T>::insert(kitty.dna.clone(), kitty);
			KittyId::<T>::put(next_id);

			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: T::Hash) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let from = ensure_signed(origin)?;
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			ensure!(from != to, Error::<T>::TransferToSelf);

			let mut from_owned = KittiesOwned::<T>::get(&from);

			// Remove kitty from list of owned kitties.
			if let Some(ind) = from_owned.iter().position(|ids| *ids == dna) {
				from_owned.swap_remove(ind);
			} else {
				return Err(Error::<T>::NoKitty.into());
			}

			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.try_push(dna.clone()).map_err(|_| Error::<T>::ExceedMaxKittyOwned)?;
			kitty.owner = to.clone();

			// Write updates to storage
			Kitties::<T>::insert(&dna, kitty);
			KittiesOwned::<T>::insert(&to, to_owned);
			KittiesOwned::<T>::insert(&from, from_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna });

			Ok(())
		}
	}
}

// Helper function
impl<T:Config> Pallet<T> {
	fn generate_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;
		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}
		Ok(res)
	}

	fn generate_dna() -> T::Hash {
		let (seed, _) = T::Randomness::random_seed();
		let block_number = <frame_system::Pallet<T>>::block_number();
		T::Hashing::hash_of(&(seed, block_number))
	}
}
