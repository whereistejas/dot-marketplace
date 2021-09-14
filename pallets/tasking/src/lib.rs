#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, inherent::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	type TaskId = u32;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId")]
	pub enum Event<T: Config> {
		/// Create new task. \[who, task-id\]
		NewTask(T::AccountId, TaskId),
		/// Remove existing task. \[who, task-id\]
		RemoveTask(T::AccountId, TaskId),
		/// Get all task Ids \[task-id\]
		GetTask(Vec<TaskId>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Duplicate tasks cannot exist.
		TaskAlreadyExists,
		/// Task does not exist.
		TaskDoesNotExist,
		/// WrongTaskOwner
		TaskWrongOwner,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type Tasks<T: Config> =
		StorageMap<_, Blake2_128Concat, TaskId, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_task(origin: OriginFor<T>, task: TaskId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(!Tasks::<T>::contains_key(&task), Error::<T>::TaskAlreadyExists);

			let current_block = <frame_system::Pallet<T>>::block_number();

			Tasks::<T>::insert(&task, (&sender, current_block));

			Self::deposit_event(Event::NewTask(sender, task));

			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn remove_task(origin: OriginFor<T>, task: TaskId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(Tasks::<T>::contains_key(&task), Error::<T>::TaskDoesNotExist);

			let (owner, _) = Tasks::<T>::get(&task);

			ensure!(sender == owner, Error::<T>::TaskWrongOwner);

			Tasks::<T>::remove(&task);

			Self::deposit_event(Event::RemoveTask(sender, task));

			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn get_task(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let taskid: Vec<_> = Tasks::<T>::iter().map(|(key, ..)| key).collect();

			Self::deposit_event(Event::GetTask(taskid));

			Ok(().into())
		}
	}
}
