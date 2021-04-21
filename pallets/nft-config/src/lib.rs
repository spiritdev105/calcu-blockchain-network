#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	transactional
};
use frame_system::pallet_prelude::*;

mod mock;
mod tests;

pub use module::*;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// AddWhitelist \[who\]
		AddWhitelist(T::AccountId),
		/// RemoveWhitelist \[who\]
		RemoveWhitelist(T::AccountId),
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		_phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight { 0 }
		fn integrity_test () {}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
		}
	}

	/// Whitelist for class creation
	#[pallet::storage]
	#[pallet::getter(fn account_whitelist)]
	pub type AccountWhitelist<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add an account into whitelist
		#[pallet::weight((100_000, DispatchClass::Operational))]
		#[transactional]
		pub fn add_whitelist(origin: OriginFor<T>, who: T::AccountId) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			AccountWhitelist::<T>::insert(&who, ());
			Self::deposit_event(Event::AddWhitelist(who));
			Ok((None, Pays::No).into())
		}

		/// remove an account from whitelist
		#[pallet::weight((100_000, DispatchClass::Operational))]
		#[transactional]
		pub fn remove_whitelist(origin: OriginFor<T>, who: T::AccountId) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			AccountWhitelist::<T>::remove(&who);
			Self::deposit_event(Event::RemoveWhitelist(who));
			Ok((None, Pays::No).into())
		}
	}
}
