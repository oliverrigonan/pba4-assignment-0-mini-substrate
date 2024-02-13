//! In this module, we are going to write some code that would familiarize you with how Substrate
//! and FRAME work.
//!
//! In certain ways, this is like a preview of what you will learn in the future. It does include
//! some new concepts, so make sure to read this file, including all the code comments, a to z.
//!
//! Very basic knowledge of a transaction-based system (like, duh, a blockchain) is needed here, but
//! rest assured, you should be able to go through this exercise without any prior blockchain
//! knowledge.
//!
//! The next section aims to help you fill that potential knowledge-gap.
//!
//! > Note: in this module, we will use the [`trait Get`] from `k_macros`.
//!
//! ## New Concepts
//!
//! Let's walk over some of the new concepts that you will encounter in the rest of the code.
//!
//! ### Runtime, Modules, and Storage
//!
//! The end-game of this exercise is for you to build a small system that mimics a blockchain
//! runtime (a piece of logic that is capable of receiving transactions, and executes them, which
//! results to some state being updated. That's pretty much it.).
//!
//! ```nocompile
//! ┌────────────────────┐    ┌────────────────────┐    ┌───────────────────┐
//! │                    │    │                    │    │                   │
//! │        state       ◄────┤       runtime      ◄────┤    transaction    │
//! │                    │    │                    │    │                   │
//! └────────────────────┘    └────────────────────┘    └───────────────────┘
//! ```
//!
//! Our runtime is composed of multiple small modules, such as a module for a "crypto-currency"
//! functionality and a basic "staking" one. Each module is explained extensively within its
//! corresponding `mod`. See [`staking_module`] and [`currency_module`] for more information.
//!
//! Your runtime needs to store some information about the "state of the world". This is what we
//! called "state" in the previous diagram and paragraph. To update the state, it needs access to an
//! underlying storage API. This is the [`io_storage`] module. The storage api provided by
//! `io_storage` is merely a key-value based API, where both keys and values are raw bytes, i.e.
//! `Vec<u8>`. This is good to begin with, but it is not so handy. To improve this, we have
//! (partially) introduced two new abstractions to help you:
//!
//! 1. A [`StorageValue`](shared::StorageValue) trait that allows you to store a single value of a
//!    particular type `Value` (expressed as an associated type in the trait) in the storage.
//! 2. A [`StorageMap`](shared::StorageMap) trait that allows you to store a map of typed key-values
//!    (expressed as associated types in the trait) in the storage.
//!
//! The main point of these abstractions is to allow you to store "typed values" in the storage, for
//! example a u32, rather than always dealing with `Vec<u8>`. To achieve this, we need to decide on
//! two things:
//!
//! 1. How to translate a "typed value" into a `Vec<u8>`?
//!
//! This is called *encoding*, and we already have a crate that you can use: `parity-scale-codec`.
//! See the documentation of the crate for more information. But, be aware that you will only
//! need to use this crate at a very high level for now (it will be covered in
//! more depth in the classroom)!
//!
//! > For example, if you have a map of AccountId => u64 (everyone's balance), this is about
//! > determining how to convert that u64 to a `Vec<u8>` when talking to `io_storage`. Obviously,
//! > for primitive types like `u8` this is will be the (8) byte representation of the number, but
//! > `parity-scale-codec` also supports more complex types like `Vec<u8>`, `Option<u8>`, `struct`,
//! > etc.
//!
//! 2. How to construct the proper `Vec<u8>` key?
//!
//! > For example, if you have a map of AccountId => u64 (everyone's balance), this is about
//! > determining which `io_storage` raw key to use to find the balance of a particular account.
//!
//! Read the [`shared::StorageValue`] and [`shared::StorageMap`] trait to get a better understanding
//! of this. Most of the code related to this part is already written, you just have to understand
//! and use it.
//!
//! ### Call, Sender, and Dispatch
//!
//! Once you have a runtime capable of altering an underlying storage, you want to do things with
//! it. The entry point for any operation in your runtime is typically a "transaction". In this
//! exercise, we will represent a transaction with an enum called a `Call` (pun unintended).
//!
//! A call is an `enum` representing all possible transactions that can be executed in your runtime,
//! alongside arguments that it can receive from the outer world. Each module will expose its own
//! call, as such:
//!
//! ```
//! enum Call {
//!   // An operation that takes two u32s as arguments.
//!   Operation1(u32, u32),
//!   // An operation that takes no arguments.
//!   ArgumentLessOperation,
//! }
//! ```
//!
//! Every call is always executed along side an `AccountId` that is the **sender** of that
//! transaction.
//!
//! This process of *executing* a `call` is called *dispatching*, denoted by the
//! [`shared::Dispatchable`] trait.
//!
//! Needless to say, we will eventually implement a trait called `Dispatchable` for an enum called
//! `Call`.
//!
//! Once you have multiple `Call` enums, from different modules, you can group them together in a
//! parent `Call` enum, as such:
//!
//! ```
//! // in module_A
//! enum ModuleACall { Foo, Bar }
//! // in module_B
//! enum ModuleBCall { Baz, Qux }
//!
//! // final call of the entire runtime.
//! enum Call {
//!     ModuleA(ModuleACall),
//!     ModuleB(ModuleBCall),
//! }
//! ```
//!
//! You will see an example of this in [`runtime`] module, which aggregates a few things together,
//! including the `Call`.
//!
//! ### Instructions
//!
//! In the rest of this file, you will finish the implementation of a simple runtime composed of two
//! simple modules:
//!
//! 1. a "crypto-currency" module, exposing transactions like `transfer`, `mint`.
//! 2. a "staking" module, exposing only a `bond` transaction that allows someone to "stake" (i.e.
//!    reserve) some tokens.
//!
//! At a high level, each modules has the following items:
//!
//! 1. a `Call` enum, which is the list of all possible transactions.
//! 2. a `Module` struct, which is the actual implementation of the transaction.
//! 3. some storage types, based on the need.
//! 4. a `Config` trait, to allow the module to be "*configured*" by the outer runtime.
//! 5. optionally, an `enum Error`.
//!
//! > Each of these items are explained further in each module.
//!
//! Moreover, the [`shared`] modules contains some shared functionality, and a [`io_storage`]
//! contains the aforementioned storage API.
//!
//! Most of these modules are already written. You will only need to understand them, and complete
//! the missing parts, denoted by `todo!()` macro.
//!
//! Within each module, one of the tasks that you might encounter is to finish the declaration of a
//! storage value or map. For that, as noted above, you have to define under which key the data is
//! stored. Your implementation must adhere to the following specification:
//!
//! 1. For each [`shared::StorageValue`], assuming that the name of type implementing `StorageValue`
//!    is `N`, the key should (simply) be equal to `N`, represented as bytes. For example, if the
//!    storage value struct is called `Balance`, the key should be `b"Balance"`.
//!
//! 2. For each [`shared::StorageMap`], assuming that the name of type implementing `StorageMap` is
//!    `N`, the final key of a map key `k` should be `concat(N, '_', encode(k))`, where `encode`
//!    comes from `parity-scale-codec`. For example, if the storage map struct is called `Balances`,
//!    the key for the account `alice` should be `b"Balances_alice"`.
//!
//! See `storage_encoding` tests for more examples.
//!
//! A nuanced detail in this assignment is that you will have to glue the two modules together via a
//! shared trait. This is common pattern both in Rust and in Substrate. The scenario is as follows:
//!
//! The [`struct Module`](currency_module::Module) in `currency_module` is the entity that is
//! exposing the functionality of a crypto-currency. One such functionality is the ability to
//! "reserve" some funds, which is what `staking_module` needs. But, *BUT*, we don't want to tightly
//! couple the two modules together. As in, we don't want [`staking_module`] to directly rely on
//! [`currency_module`].
//!
//! > The reasoning is behind the scope of this exercise, and just to give you a hint:
//! > imagine we could have different implementations of a crypto-currency that have the same
//! > "reserve" API.
//!
//! To achieve this, we provided a new trait [`shared::CryptoCurrency`]. This is the shared
//! interface that we think any crypto-currency, including our very own [`currency_module`], should
//! implement.
//!
//! As you expect, [`currency_module::Module`] will implement [`shared::CryptoCurrency`].
//!
//! Lastly, [`staking_module`] will import this functionality through an associated type over a
//! `trait Config`, which we will learn more about later.
//!
//! See [`staking_module::Config::Currency`] for more information.
//!
//! This diagram should help you put all of this information into perspective:
//!
//! ```nocompile
//! ┌────────────────────────────────────────────────────────┐
//! │                                                        │
//! │                    R U N T I M E                       │
//! │                                                        │                ┌────────────┐
//! │   ┌───────────┐                      ┌────────────┐    │    dispatch()  │            │
//! │   │           │ trait CryptoCurrency │            │    ◄────────────────┤    CALL    │
//! │   │  staking  ├──────────────────────►  currency  │    │       ▲        │            │
//! │   │           │                      │            │    │       │        ├────────────┤
//! │   └───────────┘                      └────────────┘    │       │        ├────────────┤
//! │                                                        │       │        │            │
//! └─────────────────────────┬──────────────────────────────┘       └────────┤   SENDER   │
//!                           │                                               │            │
//!                           │ get(..), set(.., ..);                         └────────────┘
//!                           │
//!         ┌─────────────────▼──────────────────┐
//!         │                                    │
//!         │           S T O R A G E            │
//!         │                                    │
//!         └────────────────────────────────────┘
//! ```
//!
//! ### Grading
//!
//! This exercise is graded by automatically invoking methods on known types in this module, such
//! as:
//!
//! * `runtime::RuntimeCall`
//! * `currency_module::TotalIssuance`
//!
//! For example, we will construct a known call
//! (`runtime::RuntimeCall::Currency(currency_module::Call::Mint{ .. })`), `dispatch` it, and
//! inspect the storage to ensure correctness.
//!
//! This means you **should not change the name or the path of any of the types in this module**.
//! Failing to do so will cause the auto-grading to fail.
//!
//! Another way to think about it is that any change that you make to the existing code should be
//! backwards compatible. For example, `trait Storage Value` contains `type Value: Encode`. If you
//! happen to need to bound `type Value` to `Clone`, that's perfectly backwards compatible, but
//! removing the bound to `Encode`, or removing `type Value` altogether is NOT!
//!
//! Some examples of this type of auto-grading is provided in the `mod tests` module, and in the
//! rust-docs.
//!
//! By the end of this exercise, you have tinkered with a small code-base that resembles FRAME and
//! Substrate to a high extent, and it should prepare you to aptly learn the real deal during the
//! academy! Good luck.
//!
//! > You can read the documentation of this module by invoking `cargo doc --open`. We have done our
//! > best to document this module extensively, so it can act as a good example of "how to write
//! > well documented Rust code" as well ;).
//!
//! > The majority of the tests of this module are parts of the rust-doc, so you can read them in
//! > the docs, and they are still executed upon `cargo test`.

use parity_scale_codec::{Decode, Encode};

/// Everything to do with your storage. This is the backing storage that will support all your
/// module and eventually the runtime. It works off a key-value basis. You ask it for value stored
/// under some key, or store a new value under some key.
///
/// Both keys and values are opaque bytes (`Vec<u8>`).
///
/// In a real blockchain, this would be a database. In this test, we use a simple thread-local map
/// to store all the data.
///
/// Not a single line in this module should change! You can even skip understanding it, just use the
/// `get`, `set` and `clear` function when needed.
pub mod io_storage {
	use std::{cell::RefCell, collections::BTreeMap};

	pub type Key = Vec<u8>;
	pub type Value = Vec<u8>;

	thread_local! {
		static STORAGE: RefCell<BTreeMap<Key, Value>> = RefCell::new(BTreeMap::<Key, Value>::new());
	}

	/// Get the value under `key`.
	pub fn get(key: Vec<u8>) -> Option<Vec<u8>> {
		STORAGE.with(|s| s.borrow().get(&key).cloned())
	}

	/// Set the value under `key` to `value`.
	pub fn set(key: Vec<u8>, value: Vec<u8>) {
		STORAGE.with(|s| s.borrow_mut().insert(key, value));
	}

	/// Remove the value under `key`.
	pub fn clear(key: Vec<u8>) {
		STORAGE.with(|s| s.borrow_mut().remove(&key));
	}
}

/// The shared functionality between all modules.
///
/// Read this module carefully. *Most* of it is already written (except for the `mutate` methods),
/// and you will only need to use it.
pub mod shared {
	use super::*;
	use num::Zero;
	use num::{CheckedAdd, CheckedSub};
	use std::fmt::Debug;

	// This is called a "re-export", we import it, and export it as well, so other modules in this
	// file can use it as `shared::Get`.
	pub use crate::get::Get;

	/// An abstraction over the account identifier.
	///
	/// For the sake of simplicity, we use a `u32` as the account type all across this exercise.
	/// (Creating a `newtype` will help us from accidentally treating it as a number)
	///
	/// Note that the encoding of this will simply be the same as the inner `u32`.
	///
	/// ```
	/// # use parity_scale_codec::Encode;
	/// # use pba_pre_course_assignment::l_mini_substrate::shared;
	///
	/// # fn main() {
	///     assert_eq!(shared::AccountId(1000).encode(), vec![232, 3, 0, 0]);
	///     assert_eq!(1000u32.encode(), vec![232, 3, 0, 0]);
	/// # }
	/// ```
	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
	pub struct AccountId(pub u32);

	/// Something that can be dispatched.
	///
	/// This is typically implemented for various `Call` enums.
	pub trait Dispatchable {
		/// Dispatch self, assuming the given `sender`.
		fn dispatch(self, sender: AccountId) -> DispatchResult;
	}

	/// The errors that can occur during dispatch.
	#[derive(Debug, PartialEq, Clone)]
	pub enum DispatchError {
		/// An error happening in `module_id`, with the given `reason`.
		Module {
			module_id: &'static str,
			reason: String,
		},
		/// All other errors, with some explanatory string.
		Other(&'static str),
	}

	/// Final return type of all dispatch functions.
	pub type DispatchResult = Result<(), DispatchError>;

	/// Abstraction around a value stored in the storage.
	///
	/// This trait provides all the auto-implementation for a struct to become a storage value, via
	/// implementing it.
	///
	/// The only method that must be implemented is `raw_storage_key`. This method will dictate
	/// where should the key of this storage value be stored in the underlying storage (when
	/// communicating with `io_storage` module).
	///
	/// > Traits like this that give a lot of auto-implementations by adding a small number of
	/// > custom implementations are a powerful Rust abstraction that you should be familiar with!
	///
	/// ## Example
	///
	/// ```
	/// # use pba_pre_course_assignment::l_mini_substrate::shared::StorageValue;
	///
	/// /// A single u32 stored in storage.
	/// struct DummyStorageValue;
	/// impl StorageValue for DummyStorageValue {
	///     type Value = u32;
	///     fn raw_storage_key() -> Vec<u8> {
	///             b"dummy_storage_value".to_vec()
	///     }
	/// }
	///
	/// fn main() {
	///     assert_eq!(DummyStorageValue::get(), None);
	///     assert!(!DummyStorageValue::exists());
	///
	///     // when
	///     DummyStorageValue::set(42);
	///
	///     // then
	///     assert_eq!(DummyStorageValue::get(), Some(42));
	///     assert!(DummyStorageValue::exists());
	///
	///     // when
	///     DummyStorageValue::clear();
	///
	///     // then
	///     assert_eq!(DummyStorageValue::get(), None);
	///     assert!(!DummyStorageValue::exists());
	///
	///     // when
	///     DummyStorageValue::mutate(|maybe_v| {
	///         assert!(maybe_v.is_none());
	///         *maybe_v = Some(69);
	///     });
	///
	///     // then
	///     assert_eq!(DummyStorageValue::get(), Some(69));
	///     assert!(DummyStorageValue::exists());
	///
	///     // when
	///     DummyStorageValue::mutate(|maybe_v| {
	///         assert!(maybe_v.is_some());
	///         *maybe_v = Some(77);
	///     });
	///
	///     // then
	///     assert_eq!(DummyStorageValue::get(), Some(77));
	///     assert!(DummyStorageValue::exists());
	///
	///     // when
	///     DummyStorageValue::mutate(|maybe_v| {
	///         assert!(maybe_v.is_some());
	///         *maybe_v = None;
	///     });
	///
	///     // then
	///     assert_eq!(DummyStorageValue::get(), None);
	///     assert!(!DummyStorageValue::exists());
	/// }
	/// ```
	pub trait StorageValue {
		/// The type of value that this storage value holds.
		///
		/// It can be anything that is encode- and decode-able.
		type Value: Encode + Decode;

		/// The final storage key of `Self` as a storage value.
		fn raw_storage_key() -> super::io_storage::Key;

		/// Get the underlying value. If it doesn't exist, return `None`.
		fn get() -> Option<Self::Value> {
			let key = Self::raw_storage_key();
			super::io_storage::get(key)
				.and_then(|raw_value| <Self::Value as Decode>::decode(&mut &*raw_value).ok())
		}

		/// Check if the value exists in storage.
		fn exists() -> bool {
			// Could be implemented more efficiently, guess how, and why?
			Self::get().is_some()
		}

		/// Set a new value into the storage.
		fn set(new_value: Self::Value) {
			let key = Self::raw_storage_key();
			super::io_storage::set(key, new_value.encode())
		}

		/// Remove any value stored in this storage value.
		///
		/// noop if nothing exists.
		fn clear() {
			let key = Self::raw_storage_key();
			super::io_storage::clear(key)
		}

		/// Mutate the value in place based on the given `f`.
		///
		/// `f` will see `None` if the value doesn't exist, and `Some` if it does.
		///
		/// If the value doesn't exists, but `f` mutates the given `None` to `Some(_)`, it will be
		/// created.
		///
		/// If the value exists, but it is mutated to `None`, it will be removed.
		fn mutate(f: impl FnOnce(&mut Option<Self::Value>)) {
			// todo!("provide an auto-implementation of this")
			let mut storage_value = Self::get();
			f(&mut storage_value);

			match storage_value {
				Some(new_value) => Self::set(new_value),
				None => Self::clear(),
			}
		}
	}

	/// Abstraction around a map stored in the storage.
	///
	/// This trait provides all the auto-implementation for a struct to become a storage map, via
	/// implementing it.
	///
	/// The only method that must be implemented is `raw_storage_key`. This method will dictate
	/// where should the final key of any item in this map be stored in the underlying storage (when
	/// communicating with `io_storage` module).
	///
	/// ```
	/// # use std::string::String;
	/// # use pba_pre_course_assignment::l_mini_substrate::shared::StorageMap;
	/// # use parity_scale_codec::Encode;
	///
	/// /// A map from `u32` to `String`.
	/// struct DummyStorageMap;
	/// impl StorageMap for DummyStorageMap {
	///     type Key = u32;
	///     type Value = String;
	///     fn raw_storage_key(key: Self::Key) -> Vec<u8> {
	///         let mut base_key = b"dummy_storage_map".to_vec();
	///         base_key.extend(key.encode());
	///         base_key
	///     }
	/// }
	///
	/// fn main() {
	///     assert_eq!(DummyStorageMap::get(42), None);
	///     assert!(!DummyStorageMap::exists(42));
	///
	///     // when
	///     DummyStorageMap::set(42, "PBA".to_string());
	///
	///     // then
	///     assert_eq!(DummyStorageMap::get(42), Some("PBA".to_owned()));
	///     assert!(DummyStorageMap::exists(42));
	///     // but 43 still does not exist..
	///     assert_eq!(DummyStorageMap::get(43), None);
	///     assert!(!DummyStorageMap::exists(43));
	///
	///     // when
	///     DummyStorageMap::clear(42);
	///
	///     // then
	///     assert_eq!(DummyStorageMap::get(42), None);
	///     assert!(!DummyStorageMap::exists(42));
	///
	///     // when
	///     DummyStorageMap::mutate(42, |maybe_v| {
	///         assert!(maybe_v.is_none());
	///         *maybe_v = Some("Polkadot".into());
	///     });
	///
	///     // then
	///     assert_eq!(DummyStorageMap::get(42), Some("Polkadot".into()));
	///     assert!(DummyStorageMap::exists(42));
	///
	///     // when
	///     DummyStorageMap::mutate(42, |maybe_v| {
	///         assert!(maybe_v.is_some());
	///         *maybe_v = Some("Substrate".into());
	///     });
	///
	///     // then
	///     assert_eq!(DummyStorageMap::get(42), Some("Substrate".into()));
	///     assert!(DummyStorageMap::exists(42));
	///
	///     // when
	///     DummyStorageMap::mutate(42, |maybe_v| {
	///         assert!(maybe_v.is_some());
	///         *maybe_v = None;
	///     });
	///
	///     // then
	///     assert_eq!(DummyStorageMap::get(42), None);
	///     assert!(!DummyStorageMap::exists(42));
	/// }
	/// ```
	pub trait StorageMap {
		/// The key type of this map.
		type Key: Encode + Clone;
		/// The value type of the map.
		type Value: Encode + Decode;

		/// The final storage key of the given `Self::key`.
		fn raw_storage_key(key: Self::Key) -> super::io_storage::Key;

		/// Get the value associated with `key`.
		fn get(key: Self::Key) -> Option<Self::Value> {
			let key = Self::raw_storage_key(key);
			super::io_storage::get(key)
				.and_then(|raw_value| <Self::Value as Decode>::decode(&mut &*raw_value).ok())
		}

		/// Check if the value exists in storage.
		fn exists(key: Self::Key) -> bool {
			// Could be implemented more efficiently, guess how, and why?
			Self::get(key).is_some()
		}

		/// Set a new `value` into the storage associated with `key`.
		fn set(key: Self::Key, value: Self::Value) {
			let key = Self::raw_storage_key(key);
			super::io_storage::set(key, value.encode())
		}

		/// Remove any value associated with `key` from the storage.
		fn clear(key: Self::Key) {
			let key = Self::raw_storage_key(key);
			super::io_storage::clear(key)
		}

		/// Mutate the value associated with `key` in place based on the given `f`.
		///
		/// `f` will see `None` if the value doesn't exist, and `Some` if it does.
		///
		/// If the value doesn't exists, but `f` mutates the given `None` to `Some(_)`, it will be
		/// created.
		///
		/// If the value exists, but it is mutated to `None`, it will be removed.
		fn mutate(key: Self::Key, f: impl FnOnce(&mut Option<Self::Value>)) {
			// todo!("provide an auto-implementation of this")
			let storage_key = key.clone();

			let mut storage_value = Self::get(storage_key);
			f(&mut storage_value);

			match storage_value {
				Some(new_value) => {
					let storage_key = key.clone();
					Self::set(storage_key, new_value)
				}
				None => Self::clear(key),
			}
		}
	}

	/// This is just a marker trait that wraps a bunch of other traits. It is meant to represent a
	/// numeric type, like a balance, e.g. `u32`.
	///
	/// It helps us not repeat the long list of traits multiple times, and instead just have `type:
	/// BalanceT`.
	///
	/// The blanket implementation for such marker traits is interesting and a common pattern.
	///
	/// Note the usage of `CheckedSub` and `CheckedAdd`, this is how we perform "overflow-safe"
	/// arithmetic.
	///
	/// TODO: some external resources would be good.
	pub trait BalanceT:
		Copy
		+ Clone
		+ Default
		+ Encode
		+ Decode
		+ CheckedSub
		+ CheckedAdd
		+ Zero
		+ Ord
		+ PartialOrd
		+ Eq
		+ PartialEq
		+ Debug
	{
	}
	impl<
			T: Copy
				+ Clone
				+ Default
				+ Encode
				+ Decode
				+ CheckedSub
				+ CheckedAdd
				+ Ord
				+ Zero
				+ PartialOrd
				+ Eq
				+ PartialEq
				+ Debug,
		> BalanceT for T
	{
	}

	/// A trait to represent basic functionality of a crypto-currency.
	///
	/// This should be implemented by `currency_module::Module`.
	pub trait CryptoCurrency {
		/// The numeric type used to represent balances.
		type Balance: BalanceT;

		/// Transfer `amount` from `from` to `to`.
		fn transfer(from: AccountId, to: AccountId, amount: Self::Balance) -> DispatchResult;

		/// Reserve exactly `amount` from `from`.
		fn reserve(from: AccountId, amount: Self::Balance) -> DispatchResult;

		/// Get the free balance of a given account, `None` if not existent.
		fn free_balance(of: AccountId) -> Option<Self::Balance>;

		/// Get the reserved balance of a given account, `None` if non-existent.
		fn reserved_balance(of: AccountId) -> Option<Self::Balance>;
	}
}

/// The crypto-currency module.
///
/// It contains:
///
/// 1. [`currency_module::Config`]: a wrapper for configurations of this module that should come
///        from the over-arching runtime.
/// 2. [`currency_module::Module`]: a struct that will contain all the implementations, including
///    the transactions, and the [`shared::CryptoCurrency`] trait.
/// 3. [`currency_module::Call`]: The `Call` type for this module.
///
/// Among other things.
///
/// This module contains two storage items:
///
/// 1. [`currency_module::TotalIssuance`]: a `StorageValue` containing the sum of all balances in
///    the system.
/// 2. [`currency_module::BalancesMap`]: a `StorageMap` that maps from an account ID to their
///    balance.
pub mod currency_module {
	use super::{
		io_storage,
		shared::{self, DispatchResult, Get, StorageValue, StorageMap},
	};
	use num::Zero;
	use num::{CheckedAdd, CheckedSub};
	use parity_scale_codec::{Decode, Encode};

	/// Configurations of this module, coming from the outer world/runtime.
	///
	/// These are basically all the things that we don't want to make a concrete decision about, so
	/// we let the outer world decide.
	///
	/// Within this module, we keep all implementation blocks generic over a `<T: Config>`, and use
	/// the associated items as `T::MinimumBalance`, `T::Balance` etc.
	///
	/// This is a very common pattern in Substrate!
	pub trait Config {
		/// The identifier of this module.
		const MODULE_ID: &'static str;

		/// The account that is allowed to mint new tokens. Think: the admin.
		type Minter: Get<shared::AccountId>;

		/// The minimum *free* balances (as explained in [`AccountBalance::free`]) that should be
		/// held by ANY account at ANY POINT IN TIME.
		///
		/// An account with free balance less than this amount is considered a logical error.
		type MinimumBalance: shared::Get<Self::Balance>;

		/// The numeric type that we use to store balances, e.g. `u64`.
		type Balance: shared::BalanceT;
	}

	/// This module's `Call` enum.
	///
	/// Contains all of the operations, and possible arguments (except `sender`, of course).
	pub enum Call<T: Config> {
		/// Mint `amount` of tokens to `dest`. This will increase the total issuance of the system.
		///
		/// If `dest` exists, its balance is increased. Else, it is created.
		///
		/// ### Dispatch Errors
		///
		/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
		/// * [`Error::InsufficientFunds`] if the `dest`'s free balance will not be enough to pass
		/// the bar of `T::MinimumBalance`.
		/// * [`Error::NotAllowed`] if the sender is not allowed to mint.
		Mint {
			dest: shared::AccountId,
			amount: T::Balance,
		},
		/// Transfer `amount` to `dest`.
		///
		/// The `sender` must exist in ANY CASE, but the `dest` might be created in the process.
		///
		/// Both `sender` and `dest` must finish the operation with equal or more free balance than
		/// `T::MinimumBalance`.
		///
		/// ### Dispatch Errors
		///
		/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
		/// * [`Error::DoesNotExist`] if the sender does not exist.
		/// * [`Error::InsufficientFunds`] if either `sender` or `dest` finish without
		///   `T::MinimumBalance` of free balance left.
		Transfer {
			dest: shared::AccountId,
			amount: T::Balance,
		},
		/// Transfer all of sender's free balance to `dest`. This is equal to "destroying" the
		/// sender account.
		///
		/// If `sender` has some reserved balance, operation should not be allowed.
		///
		/// ### Dispatch Errors
		///
		/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
		/// * [`Error::NotAllowed`] If the sender has any free balance left.
		///
		/// Since the sender is a valid account, with more than `T::MinimumBalance`, the recipient
		/// is also guaranteed to have at least `T::MinimumBalance`.
		TransferAll { dest: shared::AccountId },
	}

	/// The error type of this module.
	///
	/// We will provide a conversion `From<Error> for shared::DispatchError`. This will allow us to
	/// easily convert the error of this particular module into the error of
	/// [`shared::DispatchResult`]. Moreover, it allows for the `?` operator to be easily used.
	pub enum Error<T: Config> {
		/// The account of choice does not exist.
		DoesNotExist,
		/// Given operation is not allowed.
		NotAllowed,
		/// The account of choice does exist, but the amount that is being used is not enough to
		/// cover the requested operation.
		InsufficientFunds,
		/// Some arithmetic operation overflowed.
		Overflow,
		/// We use T in a PhantomData so that `Error` is parameterized over `T`, allowing access to
		/// Config items like `T::MODULE_ID` when we use `Error` later.
		#[allow(non_camel_case_types)]
		__marker(std::marker::PhantomData<T>),
	}

	impl<T: Config> std::fmt::Debug for Error<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Error::DoesNotExist => write!(f, "DoesNotExist"),
				Error::NotAllowed => write!(f, "NotAllowed"),
				Error::InsufficientFunds => write!(f, "InsufficientFunds"),
				Error::Overflow => write!(f, "Overflow"),
				Error::__marker(_) => unreachable!("__marker should never be printed"),
			}
		}
	}

	impl<T: Config> From<Error<T>> for shared::DispatchError {
		fn from(e: Error<T>) -> Self {
			// todo!("provide an implementation of this");
			let module_id = T::MODULE_ID;
			match e {
				Error::DoesNotExist => shared::DispatchError::Module {
					module_id,
					reason: String::from("DoesNotExist"),
				},
				Error::NotAllowed => shared::DispatchError::Module {
					module_id,
					reason: String::from("NotAllowed"),
				},
				Error::InsufficientFunds => shared::DispatchError::Module {
					module_id,
					reason: String::from("InsufficientFunds"),
				},
				Error::Overflow => shared::DispatchError::Module {
					module_id,
					reason: String::from("Overflow"),
				},
				Error::__marker(_) => {
					shared::DispatchError::Other("__marker should never be printed")
				}
			}
		}
	}

	/// A wrapper for the balance of a user/account.
	///
	/// The free balance of an account is the subset of the account balance that can be transferred
	/// out of the account. As noted elsewhere, the free balance of ALL accounts at ALL TIMES mut be
	/// equal or more than that of `T::MinimumBalance`.
	///
	/// Conversely, the reserved part of an account is a subset that CANNOT be transferred out,
	/// unless if explicitly unreserved.
	#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, Default)]
	pub struct AccountBalance<T: Config> {
		/// The free balance that they have. This can be transferred.
		pub free: T::Balance,
		/// The reserved balance that they have. This CANNOT be transferred.
		pub reserved: T::Balance,
	}

	// NOTE: make sure to return correct [`Error`] types based on [`Call`] specifications.
	impl<T: Config> AccountBalance<T> {
		/// Reserve `amount`, if possible.
		fn reserve(&mut self, amount: T::Balance) -> shared::DispatchResult {
			// todo!(
			// 	"write this implementation based on the documentation above, including the errors"
			// );
			match self.free.checked_sub(&amount) {
				Some(leftover) if leftover >= T::MinimumBalance::get() || leftover.is_zero() => {
					match self.reserved.checked_add(&amount) {
						Some(total) => {
							self.free = leftover;
							self.reserved = total;

							Ok(())
						}
						_ => Err(Error::<T>::Overflow)?,
					}
				}
				_ => Err(Error::<T>::InsufficientFunds)?,
			}
		}

		/// Unreserve `amount`, if possible.
		fn unreserve(&mut self, amount: T::Balance) -> shared::DispatchResult {
			// todo!(
			// 	"write this implementation based on the documentation above, including the errors"
			// );
			match self.reserved.checked_sub(&amount) {
				Some(leftover) => match self.free.checked_add(&amount) {
					Some(total) => {
						self.free = total;
						self.reserved = leftover;

						Ok(())
					}
					_ => Err(Error::<T>::Overflow)?,
				},
				_ => Err(Error::<T>::InsufficientFunds)?,
			}
		}

		/// Returns true if we have enough free balance to transfer `amount`.
		fn can_transfer(&self, amount: T::Balance) -> DispatchResult {
			match self.free.checked_sub(&amount) {
				Some(leftover) if leftover >= T::MinimumBalance::get() || leftover.is_zero() => {
					Ok(())
				}
				_ => Err(Error::<T>::InsufficientFunds)?,
			}
		}

		/// Send/transfer `amount` from the free balance.
		fn transfer(&mut self, amount: T::Balance) -> shared::DispatchResult {
			match self.free.checked_sub(&amount) {
				Some(leftover) if leftover >= T::MinimumBalance::get() || leftover.is_zero() => {
					self.free = leftover;
					Ok(())
				}
				_ => Err(Error::<T>::InsufficientFunds)?,
			}
		}

		/// Returns true if this amount can be received
		fn can_receive(&self, amount: T::Balance) -> DispatchResult {
			self.free
				.checked_add(&amount)
				.ok_or(Error::<T>::Overflow)
				.and_then(|n| {
					(n >= T::MinimumBalance::get())
						.then_some(())
						.ok_or(Error::<T>::InsufficientFunds)
				})
				.map_err(Into::into)
		}

		/// Add `amount` to the free balance, if possible.
		fn receive(&mut self, amount: T::Balance) -> DispatchResult {
			self.free = self.free.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
			if self.free < T::MinimumBalance::get() {
				Err(Error::<T>::InsufficientFunds)?
			}
			Ok(())
		}
	}

	/// A map from `AccountId` -> `AccountBalance`.
	///
	/// This is where the balance of each user should be stored.
	pub struct BalancesMap<T: Config>(std::marker::PhantomData<T>);
	impl<T: Config> shared::StorageMap for BalancesMap<T> {
		type Key = shared::AccountId;
		type Value = AccountBalance<T>;
		fn raw_storage_key(key: Self::Key) -> io_storage::Key {
			// todo!("determine storage key for BalancesMap based on the required specification")
			[b"BalancesMap".as_ref(), &key.encode()].concat()
		}
	}

	/// The total issuance. This should track be the sum of **free and reserved** balance of all
	/// accounts, at all times.
	pub struct TotalIssuance<T: Config>(std::marker::PhantomData<T>);
	impl<T: Config> shared::StorageValue for TotalIssuance<T> {
		type Value = T::Balance;
		fn raw_storage_key() -> io_storage::Key {
			// todo!("determine storage key for BalancesMap based on the required specification")
			b"TotalIssuance".to_vec()
		}
	}

	/// Just a wrapper for this module's implementations.
	///
	/// Note that this struct is itself public, but the internal implementations are not. The public
	/// interface of each module is its `Call` (followed by calling `dispatch` on it), not `Module`.
	pub struct Module<T: Config>(std::marker::PhantomData<T>);
	impl<T: Config> Module<T> {
		// NOTE: better not repeat yourself in documentation ;).

		/// See [`Call::Transfer`].
		fn transfer(
			sender: shared::AccountId,
			dest: shared::AccountId,
			amount: T::Balance,
		) -> shared::DispatchResult {
			// todo!("complete this implementation based on the documentation above");
			if !BalancesMap::<T>::exists(sender) {
				Err(Error::<T>::DoesNotExist)?
			}

			let mut sender_account_balance = BalancesMap::<T>::get(sender).unwrap();
			let can_transfer = sender_account_balance.can_transfer(amount.into());
			match can_transfer {
				Ok(_) => sender_account_balance.transfer(amount.into()).unwrap_or_default(),
				Err(error) => return Err(error)
			}

			let mut dest_account_balance: AccountBalance<T> = AccountBalance {
				free: amount.into(),  
				reserved: Zero::zero(),
			};

			if BalancesMap::<T>::exists(dest) {
				dest_account_balance = BalancesMap::<T>::get(dest).unwrap();
				let can_receive = dest_account_balance.can_receive(amount.into());
				match can_receive {
					Ok(_) => dest_account_balance.receive(amount.into()).unwrap_or_default(),
					Err(error) => return Err(error)
				}
			} else {
				if amount < T::MinimumBalance::get() {
					return Err(Error::<T>::InsufficientFunds)?
				}
			}

			BalancesMap::mutate(sender, |sender_balance| {
				*sender_balance = Some(sender_account_balance);
			});
			BalancesMap::mutate(dest, |dest_balance| {
				*dest_balance = Some(dest_account_balance);
			});

			Ok(())
		}

		/// See [`Call::TransferAll`].
		fn transfer_all(
			sender: shared::AccountId,
			dest: shared::AccountId,
		) -> shared::DispatchResult {
			// todo!("complete this implementation based on the documentation above");
			if !BalancesMap::<T>::exists(sender) {
				Err(Error::<T>::DoesNotExist)?
			}

			let mut sender_account_balance = BalancesMap::<T>::get(sender).unwrap();
			let can_transfer = sender_account_balance.can_transfer(sender_account_balance.free.into());
			match can_transfer {
				Ok(_) => sender_account_balance.transfer(sender_account_balance.free.into()).unwrap_or_default(),
				Err(error) => return Err(error)
			}

			if !BalancesMap::<T>::exists(dest) {
				Err(Error::<T>::DoesNotExist)?
			}

			let mut dest_account_balance = BalancesMap::<T>::get(dest).unwrap();
			let can_receive = dest_account_balance.can_receive(dest_account_balance.free.into());
			match can_receive {
				Ok(_) => dest_account_balance.receive(dest_account_balance.free.into()).unwrap_or_default(),
				Err(error) => return Err(error)
			}

			BalancesMap::mutate(sender, |sender_balance| {
				*sender_balance = Some(sender_account_balance);
			});
			BalancesMap::mutate(dest, |dest_balance| {
				*dest_balance = Some(dest_account_balance);
			});

			Ok(())
		}

		/// See [`Call::Mint`].
		fn mint(
			sender: shared::AccountId,
			who: shared::AccountId,
			amount: T::Balance,
		) -> shared::DispatchResult {
			// todo!("complete this implementation based on the documentation above");
			if sender != T::Minter::get() {
				Err(Error::<T>::NotAllowed)?
			}

			if !BalancesMap::<T>::exists(who) {
				TotalIssuance::<T>::set(amount.clone());

				let new_account_balance: AccountBalance<T> = AccountBalance {
					free: amount.into(),  
					reserved: Zero::zero(),
				};

				BalancesMap::mutate(who, |account_balance| {
					*account_balance = Some(new_account_balance);
				});
			} 

			Ok(())
		}

		// NOTE: This is not reflected in [`Call`], so we document it here.

		/// Reserve exactly `amount` from `from`'s free balance.
		///
		/// ### Errors
		///
		/// * [`Error::DoesNotExist`] if the `from` account does not currently exist
		/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
		/// * [`Error::InsufficientFunds`] if the account does not have enough free funds to preform
		///   this operation. Recall that an accounts free balance must always remain equal or above
		///   `T::MinimumBalance`.
		pub fn reserve(from: shared::AccountId, amount: T::Balance) -> shared::DispatchResult {
			// todo!("complete this implementation based on the documentation above");
			if !BalancesMap::<T>::exists(from) {
				Err(Error::<T>::DoesNotExist)?
			}

			let mut reserve_account_balance = BalancesMap::<T>::get(from).unwrap();
			reserve_account_balance.reserve(amount.into()).unwrap_or_default();

			BalancesMap::mutate(from, |reserve_balance| {
				*reserve_balance = Some(reserve_account_balance);
			});

			Ok(())
		}

		/// Unreserve exactly `amount` from `from`'s reserved balance, returning it back
		/// to the free balance
		///
		/// ### Errors
		///
		/// * [`Error::DoesNotExist`] if the `from` account does not currently exist
		/// * [`Error::Overflow`] if any type of arithmetic operation overflows.
		/// * [`Error::InsufficientFunds`] if the account does not have enough reserved funds to
		///   preform this operation.
		pub fn unreserve(from: shared::AccountId, amount: T::Balance) -> shared::DispatchResult {
			// todo!("complete this implementation based on the documentation above");
			if !BalancesMap::<T>::exists(from) {
				Err(Error::<T>::DoesNotExist)?
			}

			let mut unreserve_account_balance = BalancesMap::<T>::get(from).unwrap();
			unreserve_account_balance.unreserve(amount.into()).unwrap_or_default();

			BalancesMap::mutate(from, |runeserve_balance| {
				*runeserve_balance = Some(unreserve_account_balance);
			});

			Ok(())
		}
	}

	impl<T: Config> shared::Dispatchable for Call<T> {
		fn dispatch(self, sender: shared::AccountId) -> shared::DispatchResult {
			match self {
				Call::Mint { dest, amount } => Module::<T>::mint(sender, dest, amount),
				Call::Transfer { dest, amount } => Module::<T>::transfer(sender, dest, amount),
				Call::TransferAll { dest } => Module::<T>::transfer_all(sender, dest),
			}
		}
	}

	impl<T: Config> shared::CryptoCurrency for Module<T> {
		type Balance = T::Balance;

		fn transfer(
			from: shared::AccountId,
			to: shared::AccountId,
			amount: Self::Balance,
		) -> shared::DispatchResult {
			Module::<T>::transfer(from, to, amount)
		}

		fn reserve(from: shared::AccountId, amount: Self::Balance) -> shared::DispatchResult {
			Module::<T>::reserve(from, amount)
		}

		fn free_balance(of: shared::AccountId) -> Option<Self::Balance> {
			// todo!("complete this implementation");
			Some(BalancesMap::<T>::get(of).unwrap().free)
		}

		fn reserved_balance(of: shared::AccountId) -> Option<Self::Balance> {
			// todo!("complete this implementation");
			Some(BalancesMap::<T>::get(of).unwrap().reserved)
		}
	}
}

/// The staking module.
///
/// The term staking is taken from "Proof of Stake" jargon, which you are free to study if you want,
/// but is not necessary to understand this module.
///
/// All you need to know is that within a staking system, accounts wish to "stake" a certain amount
/// of funds that they hold. In this context, "staking" essentially means "reserving" some funds, as
/// done in the [`currency_module`].
///
/// > For the sake of simplicity, this functionality is one-way. You can bond, but there is no way
/// > to unbond :).
///
/// This module has no storage or error of itself, it entire relies on something else that
/// implements [`shared::CryptoCurrency`], see [`staking_module::Config::Currency`].
pub mod staking_module {
	use super::{*, shared::StorageMap};

	/// The configuration trait for this module.
	pub trait Config {
		/// Some type that can provide the currency functionality to this module.
		type Currency: shared::CryptoCurrency<Balance = u64>;
	}

	/// Just a type alias to make it easier to access the balance type coming in from
	/// `Config::Currency::Balance`. Try using `Config::Currency::Balance` directly and see why it
	/// won't work. Ruminate a lot on this, make sure you get it!
	type BalanceOf<T> = <<T as Config>::Currency as shared::CryptoCurrency>::Balance;

	/// Just a wrapper for this module's implementations.
	///
	/// Note that this struct is itself public, but the internal implementations are not. The public
	/// interface of each module is its `Call` (followed by calling `dispatch` on it), not `Module`.
	pub struct Module<T: Config>(std::marker::PhantomData<T>);
	impl<T: Config> Module<T> {
		fn bond(sender: shared::AccountId, amount: BalanceOf<T>) -> shared::DispatchResult {
			// todo!("complete this implementation");
			if !currency_module::BalancesMap::<runtime::MyRuntime>::exists(sender) {
				Err(currency_module::Error::<runtime::MyRuntime>::DoesNotExist)?
			}

			currency_module::Module::<runtime::MyRuntime>::reserve(sender, amount)?;
			Ok(())
		}
	}

	/// This module's `Call` enum.
	///
	/// Contains all of the operations, and possible arguments (except `sender`, of course).
	pub enum Call<T: Config> {
		/// Bond `amount` form the `sender`, if they have enough free balance.
		Bond { amount: BalanceOf<T> },
	}

	impl<T: Config> shared::Dispatchable for Call<T> {
		fn dispatch(self, sender: shared::AccountId) -> shared::DispatchResult {
			match self {
				Call::Bond { amount } => Module::<T>::bond(sender, amount),
			}
		}
	}
}

/// This is your over-arching runtime! This is where you will:
///
/// 1. Implement the `Config` trait of individual modules, in essence specifying what the
///    configurable `type` items in each `Config` trait are!
/// 2. Create an outer `RuntimeCall` and implement [`shared::Dispatchable`] for it.
pub mod runtime {
	use super::shared::{AccountId, Dispatchable, Get};

	/// This is the runtime struct that will fulfill the `Config` trait of all the modules.
	///
	/// Note that the values that we use in this runtime (MinimumBalance = 5, Minter = 42) is
	/// totally arbitrary and can be changed. For automated grading, other values will be used.
	pub struct MyRuntime;

	// NOTE: you can use your `crate::impl_get` from a previous exercise here!
	pub struct MinimumBalance;
	impl Get<u64> for MinimumBalance {
		fn get() -> u64 {
			5
		}
	}

	/// Whoever is able to mint.
	pub struct Minter;
	impl Get<AccountId> for Minter {
		fn get() -> AccountId {
			AccountId(42)
		}
	}

	impl super::currency_module::Config for MyRuntime {
		const MODULE_ID: &'static str = "MOD_CURRENCY";
		type Balance = u64;
		type MinimumBalance = MinimumBalance;
		type Minter = Minter;
	}

	impl super::staking_module::Config for MyRuntime {
		type Currency = super::currency_module::Module<MyRuntime>;
	}

	/// The outer call enum of your runtime.
	///
	/// This is merely a wrapper for all individual call enums of each module.
	pub enum RuntimeCall {
		Currency(super::currency_module::Call<MyRuntime>),
		Staking(super::staking_module::Call<MyRuntime>),
	}

	impl Dispatchable for RuntimeCall {
		fn dispatch(self, sender: AccountId) -> super::shared::DispatchResult {
			// todo!("complete this implementation");
			match self {
				RuntimeCall::Currency(value) => super::shared::Dispatchable::dispatch(value, sender),
				RuntimeCall::Staking(value) => super::shared::Dispatchable::dispatch(value, sender),
			}
		}
	}
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	90
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	15
}

/// Some basic tests for this module. Feel free to adjust, and add more tests.
#[cfg(test)]
mod tests {
	use super::*;
	use crate::l_mini_substrate::{
		runtime::{self, MyRuntime},
		shared,
		shared::*,
	};

	// shared setup for all tests. All we do for now is create one funded account '7'. Feel free to
	// change this function based on the rest of your tests.
	fn setup() {
		let minter = shared::AccountId(42);
		let dest = shared::AccountId(7);
		let amount = 100;
		currency_module::Call::<MyRuntime>::Mint { dest, amount }
			.dispatch(minter)
			.unwrap();
	}

	mod currency_tests {
		use super::*;
		use currency_module::{BalancesMap, Call, TotalIssuance};

		#[test]
		fn storage_encoding() {
			assert_eq!(
				TotalIssuance::<MyRuntime>::raw_storage_key(),
				b"TotalIssuance"
			);
			assert_eq!(
				BalancesMap::<MyRuntime>::raw_storage_key(AccountId(42)),
				[b"BalancesMap".as_ref(), &[42u8, 0, 0, 0]].concat()
			);
		}

		#[test]
		fn transfer_works() {
			let minter = AccountId(42);
			let alice = AccountId(7);
			assert_eq!(TotalIssuance::<MyRuntime>::get().unwrap_or_default(), 0);

			assert!(Call::<MyRuntime>::Mint {
				dest: alice,
				amount: 100
			}
			.dispatch(minter)
			.is_ok());
			assert_eq!(TotalIssuance::<MyRuntime>::get().unwrap_or_default(), 100);

			// transfer 20 to 10
			assert!(Call::<MyRuntime>::Transfer {
				dest: AccountId(10),
				amount: 20
			}
			.dispatch(alice)
			.is_ok());
			assert_eq!(
				BalancesMap::<MyRuntime>::get(alice)
					.map(|b| b.free)
					.unwrap_or_default(),
				80
			);
			assert_eq!(
				BalancesMap::<MyRuntime>::get(AccountId(10))
					.map(|b| b.free)
					.unwrap_or_default(),
				20
			);
			assert_eq!(TotalIssuance::<MyRuntime>::get().unwrap_or_default(), 100);

			// alice cannot transfer more than she has.
			assert_eq!(
				Call::<MyRuntime>::Transfer {
					dest: AccountId(10),
					amount: 90
				}
				.dispatch(alice)
				.unwrap_err(),
				DispatchError::Module {
					module_id: "MOD_CURRENCY",
					reason: "InsufficientFunds".to_string()
				}
			);

			// alice cannot transfer less than 10 to a new account.
			assert_eq!(
				Call::<MyRuntime>::Transfer {
					dest: AccountId(11),
					amount: 3
				}
				.dispatch(alice)
				.unwrap_err(),
				DispatchError::Module {
					module_id: "MOD_CURRENCY",
					reason: "InsufficientFunds".to_string()
				}
			);
		}
	}

	mod staking_tests {
		use super::*;

		#[test]
		fn bonding_works() {
			setup();
			let alice = AccountId(7);
			let amount = 50;

			// Notice how `MyRuntime as staking_module::Config` is an equivalent type to
			// `currency_module::Module<MyRuntime>`.
			assert_eq!(
				<MyRuntime as staking_module::Config>::Currency::reserved_balance(alice),
				Some(0)
			);
			assert_eq!(
				currency_module::Module::<MyRuntime>::reserved_balance(alice),
				Some(0)
			);

			// Unlike in production code, an unwrap is perfectly fine here.
			staking_module::Call::<MyRuntime>::Bond { amount }
				.dispatch(alice)
				.unwrap();

			assert_eq!(
				<MyRuntime as staking_module::Config>::Currency::free_balance(alice),
				Some(50)
			);
			assert_eq!(
				<MyRuntime as staking_module::Config>::Currency::reserved_balance(alice),
				Some(50)
			);
		}
	}

	mod runtime_test {
		use super::*;

		#[test]
		fn runtime_dispatch_works() {
			setup();
			let alice = AccountId(7);
			let bob = AccountId(10);

			let currency_call = currency_module::Call::<MyRuntime>::Transfer {
				dest: bob,
				amount: 10,
			};
			let outer_call = runtime::RuntimeCall::Currency(currency_call);

			outer_call.dispatch(alice).unwrap();

			assert_eq!(
				currency_module::BalancesMap::<MyRuntime>::get(alice)
					.unwrap()
					.free,
				90
			);
			assert_eq!(
				currency_module::BalancesMap::<MyRuntime>::get(bob)
					.unwrap()
					.free,
				10
			);
		}
	}
}
