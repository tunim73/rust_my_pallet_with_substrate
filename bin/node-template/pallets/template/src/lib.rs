#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::Vec;
	use scale_info::prelude::format;
	use frame_support::sp_runtime::traits::Hash;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MinLength: Get<u32>;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::storage]
	pub(super) type Conhecimento<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::Hash,
		(BoundedVec<u8, T::MaxLength>, i32, T::AccountId)
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddConhecimentos {
			conhecimento: BoundedVec<u8, T::MaxLength>,
		},
		ListConhecimentos {
			conhecimentos: Vec<BoundedVec<u8, T::MaxLength>>,
		},
		SearchConhecimentos {
			dono: T::AccountId,
			conhecimentos: Vec<BoundedVec<u8, T::MaxLength>>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// A name is too short.
		TooShort,
		/// A name is too long.
		TooLong,
		/// An account isn't named.
		Unnamed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({ 50_000_000 })]
		pub fn new_conhecimento(
			origin: OriginFor<T>,
			text: Vec<u8>,
			categoria: i32
		) -> DispatchResult {


			
			let sender = ensure_signed(origin)?;

			let bounded_text: BoundedVec<_, _> = text.try_into().map_err(|_| Error::<T>::TooLong)?;
			ensure!(bounded_text.len() >= (T::MinLength::get() as usize), Error::<T>::TooShort);

			let current_block = <frame_system::Pallet<T>>::block_number();
			let data_hora_str = format!("{:?}", current_block);
			let key_hash = T::Hashing::hash_of(&data_hora_str);

			Self::deposit_event(Event::AddConhecimentos { conhecimento: bounded_text.clone() });
			<Conhecimento<T>>::insert(key_hash, (bounded_text, categoria, &sender));

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight({ 50_000_000 })]
		pub fn search(origin: OriginFor<T>, search: Vec<u8>, categoria_id:i32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut result: Vec<BoundedVec<u8, T::MaxLength>> = Vec::new();

			for (_, (texto, categoria_armazenada, owner)) in <Conhecimento<T>>::iter() {
				if categoria_armazenada == categoria_id && owner == who {
					if
						texto
							.to_vec()
							.windows(search.len())
							.any(|window| window == search.as_slice())
					{
						result.push(texto);
					}
				}
			}

			Self::deposit_event(Event::SearchConhecimentos {dono:who, conhecimentos: result });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight({ 50_000_000 })]
		pub fn get_all_conhecimento(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;

			let mut all_conhecimentos: Vec<BoundedVec<u8, T::MaxLength>> = Vec::new();

			for (_, (texto, categoria_id, _)) in Conhecimento::<T>::iter() {
				all_conhecimentos.push(texto);
			}

			Self::deposit_event(Event::ListConhecimentos { conhecimentos: all_conhecimentos });

			Ok(())
		}
	}
}
