use crate::math::datatypes::set::{TSet, TSetElement, VecSet};

/// Implments functions for calculating probubility for a object
pub trait TProbubility {
	/// Retruns true if object is the entire sample space
	fn is_sample_space(&self) -> bool;

	/// Returns 0 to 1 the P value of the set
	/// returns 1 if this is the sample space
	fn probubility(&self) -> f64;
}

impl<'a, T: TSetElement> TProbubility for VecSet<'a, T>{
	fn is_sample_space(&self) -> bool {
		!self.is_subset()
	}

	fn probubility(&self) -> f64 {
		if self.is_sample_space() { return 1.0 }
		else {
			match self.parent_size() {
				Some(size) => return self.size() as f64 / size as f64,
				_ => panic!("Set is not sample space but parents size could not be found.")
			}
		}
	}
}



// MARK: Tests
#[cfg(test)]
mod tests {

	use super::*;

	fn test_is_sample_space(set: impl TProbubility, expected: bool) {
		assert_eq!(set.is_sample_space(), expected)
	}

	#[test] fn test_is_sample_space_0() { test_is_sample_space(
		VecSet::new_from_array(&[1,2,3,4]),
		true
	);}

	#[test] fn test_is_sample_space_1() { test_is_sample_space(
		VecSet::subset_from_range(
			&VecSet::new_from_array(&[1,2,3,4]),
			1,
			2
		),
		false
	);}

	fn test_probubility(set: impl TProbubility, expected: f64){
		assert_eq!(set.probubility(), expected)
	}

	#[test] fn test_probubility_0() { test_probubility(
		VecSet::new_from_array(&[1,2,3,4]),
		1.0
	);}

	#[test] fn test_probubility_1() { test_probubility(
		VecSet::subset_from_range(
			&VecSet::new_from_array(&[1,2,3,4]),
			1,
			2
		),
		0.25
	);}
}