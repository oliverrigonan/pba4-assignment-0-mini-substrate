///! A trait that will be used in mini substrate.


/// A way to convey a value of type `T` via a `type`
/// (ie) a struct that implements `Get`.
/// 
/// You encountered this trait during the qualifier when you
/// wrote your impl_get! macro.
pub trait Get<T> {
	fn get() -> T;
}