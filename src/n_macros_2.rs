// Consider the following setup.
//
// A number of different types/structs all implement a common `trait OnInitialize`.

pub struct Module1;
pub struct Module2;
pub struct Module3;
pub struct Module4;

/// Does not matter what this does.
pub trait OnInitialize {
	/// Does not matter what this does.
	fn on_initialize();
}

impl OnInitialize for Module1 {
	fn on_initialize() {}
}
impl OnInitialize for Module2 {
	fn on_initialize() {}
}
impl OnInitialize for Module3 {
	fn on_initialize() {}
}
impl OnInitialize for Module4 {
	fn on_initialize() {}
}

/// Now, we want to write a macro that reduces the boilerplate of implementing `OnInitialize` on
/// tuples of elements that each individually implement `OnInitialize`.
///
/// For example, for a tuple of 4 elements, we could have:

// impl<A, B, C, D> OnInitialize for (A, B, C, D)
// where
// 	A: OnInitialize,
// 	B: OnInitialize,
// 	C: OnInitialize,
// 	D: OnInitialize,
// {
// 	fn on_initialize() {
// 		A::on_initialize();
// 		B::on_initialize();
// 		C::on_initialize();
// 		D::on_initialize();
// 	}
// }

/// But this is a lot of boilerplate, and we want a macro for it! If we invoke the macro, it should
/// generate all tuple implementations, all the way up to 12, like the following:
///
/// ```nocompile
/// impl<T1: OnInitialize> OnInitialize for (T1,) {
///    fn on_initialize() {
///        T1::on_initialize();
///    }
/// }
///
/// impl<T1: OnInitialize, T2: OnInitialize> OnInitialize for (T1, T2) {
///    fn on_initialize() {
///        T1::on_initialize();
///        T2::on_initialize();
///    }
/// }
///
/// // And several more impl blocks supporting up to 12 elements
/// ```
#[macro_export]
macro_rules! impl_for_tuples {
	// ( $($todo:tt)* ) => {};
	() => {};

    ($head:ident $(, $tail:ident)*) => {
        impl<$head: OnInitialize, $($tail: OnInitialize),*> OnInitialize for ($head, $($tail),*) {
            fn on_initialize() {
                $head::on_initialize();
                $($tail::on_initialize();)*
            }
        }

        impl_for_tuples!($($tail),*);
    };
}
impl_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);

// Rust also supports procedural macros.
// In the section on extension traits, we discussed a hypothetical derive macro
// (a type of procedural macro) that could count the occurrences of a particular enum variant in a
// collection. The template for such a macro is provided in the separate `count-of` crate. The macro
// is currently doing nothing. Complete it!

/// Another outcome enum similar to the one in the extension traits section. The only difference is
/// that this one derives the CountOf macro.
///
/// You may use this one to test your CountOf macro.
// #[derive(count_of::CountOf, Clone)]
#[derive(count_of::CountOf, Clone, Eq, PartialEq)]
pub enum OtherOutcome {
	Ok,
	SomethingWentWrong,
	IDontKnow,
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	// todo!()
	95
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	// todo!()
	6
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn impl_for_tuples() {
		<(Module1, Module2, Module3, Module4) as OnInitialize>::on_initialize();
		// todo!("you should be able to make these work as well");
		<(Module1, Module2, Module3) as OnInitialize>::on_initialize();
		<(Module1, Module2) as OnInitialize>::on_initialize();
	}

	#[test]
	fn count_of_works() {
		use OtherOutcome::*;
		let outcomes = vec![
			SomethingWentWrong,
			SomethingWentWrong,
			IDontKnow,
			Ok,
			IDontKnow,
			IDontKnow,
		];

		// todo!("you should be able to make these work as well");
		assert_eq!(outcomes.something_went_wrong_count(), 2);
		assert_eq!(outcomes.i_dont_know_count(), 3);
		assert_eq!(outcomes.ok_count(), 1);
	}
}
