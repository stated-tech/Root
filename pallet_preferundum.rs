#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*,traits::LockIdentifier,traits::LockableCurrency,traits::WithdrawReasons, traits::Currency, inherent::Vec, sp_runtime::traits::Hash, transactional, traits::ExistenceRequirement,dispatch::DispatchResultWithPostInfo};
	use frame_system::pallet_prelude::*;
	use frame_system::ensure_signed;

	
	// The LockIdentifier constant.
	const vote_id: LockIdentifier = *b"vote ";


	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		#[pallet::constant]
        type QuestionMinBytes: Get<u32>;

        #[pallet::constant]
        type QuestionMaxBytes: Get<u32>;

        #[pallet::constant]
        type SubjectMinBytes: Get<u32>;

        #[pallet::constant]
        type SubjectMaxBytes: Get<u32>; 

        #[pallet::constant]
        type PossibilityMinBytes: Get<u32>;

        #[pallet::constant]
        type PossibilityMaxBytes: Get<u32>; 
	}	

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Question<T: Config> {
        pub content: Vec<u8>,
        pub author: <T as frame_system::Config>::AccountId,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Subject<T: Config> {
        pub content: Vec<u8>,
        pub question_id: T::Hash,
        pub author: <T as frame_system::Config>::AccountId,
	}

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Possibility<T: Config> {
        pub content: Vec<u8>,
        pub subject_id: T::Hash,
        pub author: <T as frame_system::Config>::AccountId,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Vote<T: Config> {
        pub number: Vec<u8>,
        pub possibility_id: T::Hash,
        pub author: <T as frame_system::Config>::AccountId,
	} // ADD CONSTRAINT TO 100%



	/// Storage Map for Question by questionid (Hash) to a question
	#[pallet::storage]
	#[pallet::getter(fn question)]
	pub(super) type Question<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Question<T>>;

	/// Storage Map from question id (Hash) to a list of subject for this question
	#[pallet::storage]
	#[pallet::getter(fn subject)]
	pub(super) type Subject<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<Subject<T>>>;

    /// Storage Map from subject id (Hash) to a list of possibility for this subject
	#[pallet::storage]
	#[pallet::getter(fn possibility)]
	pub(super) type Possibility<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<Possibility<T>>>;

	/// Storage Map from subject id (Hash) to a list of possibility for this subject
	#[pallet::storage]
	#[pallet::getter(fn possibility)]
	pub(super) type Vote<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<Vote<T>>>;
	


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		SomethingStored(u32, T::AccountId),
		QuestionCreated(Vec<u8>, T::AccountId, T::Hash),
		SubjectCreated(Vec<u8>, T::AccountId, T::Hash),
        PossibilityCreated(Vec<u8>, T::AccountId, T::Hash),
	}

		/// Lockable currency can emit three event types.
		#[pallet::event]
		#[pallet::metadata(T::AccountId = "AccountId")]
		#[pallet::generate_deposit(pub(super) fn deposit_event)]
		pub enum Event<T: Config> 
		{
			/// Balance was locked successfully.
			Locked(<T as frame_system::Config>::AccountId, BalanceOf<T>),
			/// Lock was extended successfully.
			ExtendedLock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
			/// Balance was unlocked successfully.
			Unlocked(<T as frame_system::Config>::AccountId),
		}

	#[pallet::error]
	pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        QuestionNotEnoughBytes, 
        QuestionTooManyBytes, 
        SubjectNotEnoughBytes,
        SubjectTooManyBytes,
        PossibilityNotEnoughBytes,
        PossibilityTooManyBytes,
        QuestionNotFound,
     
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub(super) fn lock_capital(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>
		) -> DispatchResultWithPostInfo {
			
			let user = ensure_signed(origin)?;
		
			T::Currency::set_lock(
				vote_id,
				&user,
				amount,
				WithdrawReasons::all(),
			);
	
			Self::deposit_event(Event::Locked(user, amount));
			Ok(().into())
		}
	
		#[pallet::weight(1_000)]
		pub(super) fn extend_lock(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;
			
			T::Currency::extend_lock(
				vote_id,
				&user,
				amount,
				WithdrawReasons::all(),
			);
	
			Self::deposit_event(Event::ExtendedLock(user, amount));
			Ok(().into())
		}
	
		#[pallet::weight(1_000)]
		pub(super) fn unlock_all(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;
	
			T::Currency::remove_lock(vote_id, &user);
	
			Self::deposit_event(Event::Unlocked(user));
			Ok(().into())
		}
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10000)]
		#[transactional]
		pub fn create_question(origin: OriginFor<T>, content: Vec<u8>) -> DispatchResult {
				let author = ensure_signed(origin)?;

				ensure!(
						(content.len() as u32) > T::QuestionMinBytes::get(),
						<Error<T>>::QuestionNotEnoughBytes
				);

				ensure!(
						(content.len() as u32) < T::QuestionMaxBytes::get(),
						<Error<T>>::QuestionTooManyBytes
				);

				let question = Question { content: content.clone(), author: author.clone() };

				let question_id = T::Hashing::hash_of(&question);

				<Question<T>>::insert(question_id, question);

				let subjects_vec: Vec<Subject<T>> = Vec::new();

				<Subject<T>>::insert(question_id, subjects_vec);

				Self::deposit_event(Event::QuestionCreated(content, author, question_id));

				Ok(())
		}
	
		#[pallet::weight(5000)]
		pub fn create_subject(
				origin: OriginFor<T>,
				content: Vec<u8>,
				question_id: T::Hash,
		) -> DispatchResult {
				let comment_author = ensure_signed(origin)?;
		
				ensure!(
						(content.len() as u32) > T::QuestionMinBytes::get(),
						<Error<T>>::SubjectNotEnoughBytes
				);
		
				ensure!(
						(content.len() as u32) < T::QuestionMaxBytes::get(),
						<Error<T>>::SubjectTooManyBytes
				);
		
				let subject = Subject {
						author: comment_author.clone(),
						content: content.clone(),
						question_id: question_id.clone(),
				};
		
				<Subject<T>>::mutate(question_id, |comments| match comments {
						None => Err(()),
						Some(vec) => {
								vec.push(subject);
								Ok(())
						},
				})
				.map_err(|_| <Error<T>>::QuestionNotFound)?;
		
				Self::deposit_event(Event::SubjectCreated(
						content,
						comment_author,
						question_id,
				));
		
				Ok(())
		}

        #[pallet::weight(5000)]
		pub fn create_possibility(
				origin: OriginFor<T>,
				content: Vec<u8>,
				subject_id: T::Hash,
		) -> DispatchResult {
				let comment_author = ensure_signed(origin)?;
		
				ensure!(
						(content.len() as u32) > T::QuestionMinBytes::get(),
						<Error<T>>::PossibilityNotEnoughBytes
				);
		
				ensure!(
						(content.len() as u32) < T::QuestionMaxBytes::get(),
						<Error<T>>::PossiblityTooManyBytes
				);
		
				let possibility = Possibility {
						author: comment_author.clone(),
						content: content.clone(),
						subject_id: subject_id.clone(),
				};
		
				<Possibility<T>>::mutate(subject_id, |comments| match comments {
						None => Err(()),
						Some(vec) => {
								vec.push(possibility);
								Ok(())
						},
				})
				.map_err(|_| <Error<T>>::QuestionNotFound)?;
		
				Self::deposit_event(Event::PossibilityCreated(
						content,
						comment_author,
						subject_id,
				));
		
				Ok(())
		}


	}

	

	// ADD CONSTRAINT THAT NOONE MORE THAN 100 tokens

	#[pallet::weight(5000)]
	pub fn vote(
			origin: OriginFor<T>,
			number: Vec<u8>,
			possibility_id: T::Hash,
	) -> DispatchResult {
			let comment_author = ensure_signed(origin)?;
			lock_capital(origin,number); // CHECK THAT NUMBER IS OK BECAUSE I NEED A BALANCE TYPE

			// ADD CONSTRAINT OF NUMBER TOKEN HERE ORELSE ERROR THROUGH ENSURE LIKE ABOVE
	
			let vote = Vote {
					author: comment_author.clone(),
					number: number.clone(),
					subject_id: subject_id.clone(),
			};
	
			<Vote<T>>::mutate(subject_id, |comments| match comments {
					None => Err(()),
					Some(vec) => {
							vec.push(vote);
							Ok(())
					},
			})
			.map_err(|_| <Error<T>>::QuestionNotFound)?; // CHANGE IT FOR THE GOOD ONE
	
			// ADD EVENT IF NEEDED

			// CREATE FUNCTION UNLOCK !!! 
	
			Ok(())
	} // ADD LOGIC IN THIS TO SAY LOKEN STAKED etc AND FACT THAT WHEN VOTE FINISH UNLOCK
	 // A MON AVIS IL FAUT DISTINGUER LE LOCK DU VOTRE CAR LE LOCK ON DIRAIT QUE CEST PALLET CALL DONC PAS UTILISABLE ICI JE PENSE

}




