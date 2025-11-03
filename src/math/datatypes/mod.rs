/// Contains the struc rational which models a rational number
pub mod rational;

/// Contains the struc set which models a set theroy set
pub mod set;

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