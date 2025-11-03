// MARK: Rational

/// This is a repensentation of a rational number.
/// 
/// This is implemented as a list of numbers that are multiplied together for
/// the numerator and a list of numbers that are multiplied together for
/// the denominator.
pub struct Rational {
	nums: Vec<i32>,
	denominators: Vec<i32>,
}

impl Rational {
	/// Inits a new Rational with empty numerator and denominator
	pub fn new() -> Self {
		let nums = Vec::new();
		let denominators = Vec::new();
		Self {nums: nums, denominators: denominators}
	}

	/// Inits a new DecompsedNum with two arrays
	pub fn new_from_arrays(nums: &[i32], denoms: &[i32]) -> Self {
		Self {nums: nums.to_vec(), denominators: denoms.to_vec() }
	}

	/// Inits a new Rational with a factorial squence
	pub fn factorial(n: u32) -> Self {
		let mut ret_val = Self::new();

		for i in 1..n+1 {
			ret_val.nums.push(i as i32);
		}

		if n == 0 {
			ret_val.nums.push(1);
		}

		ret_val
	}

	/// Attempts to multiply and divide are numbers to return the composed number
	pub fn evaluate(&self) -> i128 {
		//TODO return none if overflow
		let mut ret_val: i128 = 1;

		if self.nums.len() == 0 { ret_val = 0 }
		for i in self.nums.iter() {
			ret_val *= *i as i128;
		}

		for i in self.denominators.iter() {
			ret_val /= *i as i128;
		}

		ret_val
	}

	/// Removes any pairs from the numerator and denominator
	pub fn cancel_out(&mut self) {
		let mut i = 0;
		while i < self.denominators.len() {
			let mut found_pair = false;
			for j in 0..self.nums.len() {
				if self.nums[j] == self.denominators[i] {
					self.nums.remove(j);
					self.denominators.remove(i);
					found_pair = true;
					break;
				}
			}
			if !found_pair { i += 1 };
		}
	}

	/// Multiplies and cancels out two rational numbers
	pub fn multiply(&mut self, other: &Self) {
		for i in other.nums.iter() {
			self.nums.push(*i);
		}
		for i in other.denominators.iter() {
			self.denominators.push(*i);
		}
		self.cancel_out();
		self.nums.sort();
		self.denominators.sort();
	}

	/// Takes anouther Rational and divides it into this one
	pub fn divide(&mut self, other: &Self) {
		for i in other.nums.iter() {
			self.denominators.push(*i);
		}
		for i in other.denominators.iter() {
			self.nums.push(*i);
		}
		self.cancel_out();
		self.nums.sort();
		self.denominators.sort();
	}

}

// MARK: Tests
#[cfg(test)]
mod tests {

	use super::*;


	fn test_new_from_arrays(
		num: &[i32],
		denom: &[i32]
	) {
		let value = Rational::new_from_arrays(num, denom);

		assert_eq!(value.nums, num);
		assert_eq!(value.denominators, denom);
	}

	#[test] fn test_new_from_arrays_0() { test_new_from_arrays(
		&[0;0],
		&[0;0]
	);}
	#[test] fn test_new_from_arrays_1() { test_new_from_arrays(
		&[1,2,3],
		&[0;0]
	);}
	#[test] fn test_new_from_arrays_2() { test_new_from_arrays(
		&[0;0],
		&[1,2,3]
	);}
	#[test] fn test_new_from_arrays_3() { test_new_from_arrays(
		&[1,2,3],
		&[1,2,3]
	);}

	#[test]
	fn test_initialize_rational_factorial() {
		let value = Rational::factorial(5);
		let expected = vec![1,2,3,4,5];

		assert_eq!(value.nums.len(), expected.len());

		for i in 0..5 {
			assert_eq!(value.nums[i], expected[i]);
		}
	}

	fn test_evaluate_factorial(value: u32, expected: i128) {
		let value = Rational::factorial(value);

		assert_eq!(value.evaluate(), expected)
	}

	#[test] fn test_evaluate_factorial_0() { test_evaluate_factorial(0, 1); }
	#[test] fn test_evaluate_factorial_1() { test_evaluate_factorial(1, 1); }
	#[test] fn test_evaluate_factorial_2() { test_evaluate_factorial(2, 2); }
	#[test] fn test_evaluate_factorial_3() { test_evaluate_factorial(3, 6); }
	#[test] fn test_evaluate_factorial_4() { test_evaluate_factorial(4, 24); }
	#[test] fn test_evaluate_factorial_5() { test_evaluate_factorial(5, 120); }


	fn test_cancel_out(
		value_num: &[i32],
		value_denom: &[i32],
		result_num: &[i32],
		result_denom:&[i32]
	) {
		let mut value = Rational::new();
		for i in value_num {
			value.nums.push(*i);
		}
		for i in value_denom {
			value.denominators.push(*i);
		}
		assert_eq!(value.nums, value_num);
		assert_eq!(value.denominators, value_denom);

		value.cancel_out();

		assert_eq!(value.nums, result_num);
		assert_eq!(value.denominators, result_denom);
	}

	#[test] fn test_cancel_out_0() {test_cancel_out(
		&[1,1,2,2,3,3,5],
		&[1,3,3],
		&[1,2,2,5],
		&[0; 0]
	)}
	#[test] fn test_cancel_out_1() {test_cancel_out(
		&[1,1,2,2,3,3,5],
		&[1,3,3,7],
		&[1,2,2,5],
		&[7]
	)}
	#[test] fn test_cancel_out_2() {test_cancel_out(
		&[0; 0],
		&[1,3,3],
		&[0; 0],
		&[1,3,3]
	)}
	#[test] fn test_cancel_out_3() {test_cancel_out(
		&[1,1,2,2,3,3,5],
		&[0; 0],
		&[1,1,2,2,3,3,5],
		&[0; 0]
	)}
	#[test] fn test_cancel_out_4() {test_cancel_out(
		&[1,2,3,5],
		&[1,2,3,3,5],
		&[0; 0],
		&[3]
	)}
	
	fn test_multiply(
		value_1_num: &[i32],
		value_1_denom: &[i32],
		value_2_num: &[i32],
		value_2_denom: &[i32],
		result_num: &[i32],
		result_denom: &[i32]
	) {
		let mut value_1 = Rational::new_from_arrays(value_1_num, value_1_denom);
		let value_2 = Rational::new_from_arrays(value_2_num, value_2_denom);

		value_1.multiply(&value_2);

		assert_eq!(value_1.nums, result_num);
		assert_eq!(value_1.denominators, result_denom);
	}

	#[test] fn test_multiply_0() { test_multiply(
		&[1,2,3],
		&[0;0],
		&[2],
		&[0;0],
		&[1,2,2,3],
		&[0;0]
	);}
	#[test] fn test_multiply_1() { test_multiply(
		&[1,2,3],
		&[0;0],
		&[0; 0],
		&[0;0],
		&[1,2,3],
		&[0;0]
	);}
	#[test] fn test_multiply_2() { test_multiply(
		&[1,2,3],
		&[0;0],
		&[0;0],
		&[5,6],
		&[1,2,3],
		&[5,6]
	);}

	fn test_divide(
		value_1_num: &[i32],
		value_1_denom: &[i32],
		value_2_num: &[i32],
		value_2_denom: &[i32],
		result_num: &[i32],
		result_denom: &[i32]
	) {
		let mut value_1 = Rational::new_from_arrays(value_1_num, value_1_denom);
		let value_2 = Rational::new_from_arrays(value_2_num, value_2_denom);

		value_1.divide(&value_2);

		assert_eq!(value_1.nums, result_num);
		assert_eq!(value_1.denominators, result_denom);
	}

	#[test] fn test_divide_0() { test_divide(
		&[1,2,3],
		&[0;0],
		&[2],
		&[0;0],
		&[1,3],
		&[0;0]
	);}
	#[test] fn test_divide_1() { test_divide(
		&[1,2,3],
		&[0;0],
		&[0;0],
		&[1],
		&[1,1,2,3],
		&[0;0]
	);}
	#[test] fn test_divide_2() { test_divide(
		&[0;0],
		&[0;0],
		&[2],
		&[0;0],
		&[0;0],
		&[2]
	);}
	#[test] fn test_divide_3() { test_divide(
		&[1,2,3],
		&[4,5],
		&[2],
		&[4,8],
		&[1,3,8],
		&[5]
	);}

}