
use crate::math::datatypes::rational::Rational;
use crate::math::datatypes::set::TSetElement;
use crate::math::datatypes::set::VecSet;
use crate::math::datatypes::set::TSet;


/// Prints dectription for the Combinatorics modual
pub fn description() {
	println!("This is the combinatorics modual");
}



/// n C r combination of n objects taken r at a time
pub fn combinations(n: u32 , r: u32) -> Option<u128> {
	if r > n { return None; }
	
	let mut a = Rational::factorial(n);
	let b = Rational::factorial(r);
	let c = Rational::factorial(n-r);

	a.divide(&b);
	a.divide(&c);

	Some(a.evaluate() as u128)
}


/// n P r permutations of n objects taken r at a time.
pub fn permutations(n: u32, r: u32) -> Option<u128> {
	if r > n { return None; }

	let mut a = Rational::factorial(n);
	let b = Rational::factorial(n-r);

	a.divide(&b);

	Some(a.evaluate() as u128)
}


/// TCombinatorics trait: structs with this trait have nCr and nPr avalible
pub trait TCombinatorics<'a, T> {
	/// n C r combinations from a set taken r at a time
	fn combinations(&self, r: u32) -> Option<u128>;

	/// n P r permutations from a set taken r at a time
	fn permutations(&self, r: u32) -> Option<u128>;
}

impl<'a, T: TSetElement> TCombinatorics<'a, T> for VecSet<'a, T> {
	fn combinations(&self, r:u32) -> Option<u128> {
		combinations(self.size() as u32, r)
	}

	fn permutations(&self, r: u32) -> Option<u128> {
		permutations(self.size() as u32, r)
	}
}


// MARK: Tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_description() {
		description();
	}

// region: test_combinations
	fn test_combinations(n: u32, r: u32, expected: Option<u128>) {
		assert_eq!(combinations(n, r), expected);
	}

	#[test] fn test_combinations_0() { test_combinations(
		0,
		0,
		Some(0)
	);}
	#[test] fn test_combinations_1() { test_combinations(
		10,
		0,
		Some(0)
	);}
	#[test] fn test_combinations_2() { test_combinations(
		0,
		1,
		None
	);}
	#[test] fn test_combinations_3() { test_combinations(
		10,
		1,
		Some(10)
	);}
	#[test] fn test_combinations_4() { test_combinations(
		10,
		2,
		Some(45)
	);}
	#[test] fn test_combinations_5() { test_combinations(
		10,
		9,
		Some(10)
	);}
	#[test] fn test_combinations_6() { test_combinations(
		10,
		8,
		Some(45)
	);}
// endregion:

// region: test_permutations
	fn test_permutations(n: u32, r: u32, expected: Option<u128>) {
		assert_eq!(permutations(n, r), expected);
	}

	#[test] fn test_permutations_0() { test_permutations(
		0,
		0,
		Some(0)
	);}
	#[test] fn test_permutations_1() { test_permutations(
		10,
		0,
		Some(0)
	);}
	#[test] fn test_permutations_2() { test_permutations(
		0,
		1,
		None
	);}
	#[test] fn test_permutations_3() { test_permutations(
		10,
		1,
		Some(10)
	);}
	#[test] fn test_permutations_4() { test_permutations(
		10,
		2,
		Some(90)
	);}
	#[test] fn test_permutations_5() { test_permutations(
		10,
		9,
		Some(3628800)
	);}
	#[test] fn test_permutations_6() { test_permutations(
		10,
		8,
		Some(1814400)
	);}
	// endregion:

// region: test_set_combinations
	fn test_set_combinations(set: VecSet<u32>, r: u32, expected: Option<u128>) {
		assert_eq!(set.combinations(r), expected)
	}

	#[test] fn test_set_combinations_0() { test_set_combinations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		0,
		Some(0)
	);	}

	#[test] fn test_set_combinations_1() { test_set_combinations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		1,
		Some(5)
	);	}

	#[test] fn test_set_combinations_2() { test_set_combinations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		2,
		Some(10)
	);	}

	#[test] fn test_set_combinations_3() { test_set_combinations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		6,
		None
	);	}
// endregion:

// region: test_set_permutations
	fn test_set_permutations(set: VecSet<u32>, r: u32, expected: Option<u128>) {
		assert_eq!(set.permutations(r), expected)
	}

	#[test] fn test_set_permutations_0() { test_set_permutations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		0,
		Some(0)
	);	}

	#[test] fn test_set_permutations_1() { test_set_permutations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		1,
		Some(5)
	);	}

	#[test] fn test_set_permutations_2() { test_set_permutations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		2,
		Some(20)
	);	}

	#[test] fn test_set_permutations_3() { test_set_permutations(
		VecSet::new_from_array(&[1,2,3,4,5]),
		6,
		None
	);	}
// endregion:


}