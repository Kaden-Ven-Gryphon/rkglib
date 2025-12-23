/// Contains the struc rational which models a rational number
pub mod rational;

/// Contains the struc set which models a set theroy set
pub mod set;

/// Contains the struct rkgtab which is a table struct simular to pandas or numpy aray
pub mod rkgtab;

/// Contains structs for matrix like objects.  These are have the expected spcilized functions for
/// matrix operations, but can also be used for general purpuse 2d, 3d etc arrays
pub mod matrix;

/// Prints the decription of the datatypes modual
pub fn description() {
	println!("This is the datatypes modual");
}

// MARK: Tests
#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_get_description() {
		description();
	}
}