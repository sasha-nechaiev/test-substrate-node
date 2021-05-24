#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap, traits::Randomness, RuntimeDebug};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty(pub [u8; 16]);

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum KittyGender {
    Male,
    Female
}

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

impl Kitty {
    pub fn get_gender(&self) -> KittyGender {
        if self.0[0] % 2 == 0 {
            KittyGender::Male
        } else {
            KittyGender::Female
        }
    }
}


decl_event! {
    pub enum Event<T> where 
        <T as frame_system::Config>::AccountId,
        {
            KittyCreated(AccountId, u32, Kitty),
        }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        KittyIdOverflow,
    }
}



decl_storage! {
    trait Store for Module<T:Config> as Kitties {
        pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
        pub NextKittyId get(fn next_kitty_id): u32;
    }
}



decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;
        
        #[weight = 1000]
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;

            // generate a random value
            let payload = (
                <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
                &sender,
                <frame_system::Module<T>>::extrinsic_index(),
            );
            let dna = payload.using_encoded(blake2_128);

            let kitty = Kitty(dna);
            let kitty_id = Self::next_kitty_id();

            <Kitties<T>>::insert(&sender, kitty_id, kitty.clone());

            let new_kitty_id = Self::next_kitty_id().checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;

            NextKittyId::put(new_kitty_id);

            // emit event
            Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty));

        }
    }
}