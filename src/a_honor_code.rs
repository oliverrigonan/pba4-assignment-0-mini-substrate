//! This portion of the assignment represents an honor code. By returning `true` from each of these
//! functions you are attesting that you have followed the various rules of the assignment.
//!
//! Cheating on this assignment will only hurt yourself as you are likely to feel lost and frustrated at
//! the Polkadot Blockchain Academy if you do not have the necessary Rust background to attend.
//!
//! If you are in any doubt of something being allowed or disallowed not, please directly ask the
//! Academy staff for clarification and guidance.

/// You must write your own code on this assignment. That means it should not be copied from any
/// human or AI programmers. The academy may end up having some co-working sessions, but group working
/// is not otherwise allowed.
pub fn wrote_my_own_code() -> bool {
	// If you have followed this rule, return `true`
	// todo!()
	true
}

/// You are not allowed to use external dependencies from `crates.io` or elsewhere unless
/// explicitly stated in the problem.
pub fn no_additional_dependencies() -> bool {
	// If you have followed this rule, return `true`
	// todo!()
	true
}

#[cfg(test)]
mod tests {
	use super::*;

	fn has_honor(f: &dyn Fn() -> bool) {
		assert!(
			f(),
			"Thank you for your honesty in letting us know you have not followed the honor code."
		)
	}

	#[test]
	fn honor_code_respected() {
		has_honor(&wrote_my_own_code);
		has_honor(&no_additional_dependencies);
	}
}
