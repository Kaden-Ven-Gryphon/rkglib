use core::fmt;
use std::ops::{Shl, ShlAssign};

/// ndarray like data structure
pub struct RkgTabN<T> {
	data: Vec<T>,
	dims: usize,
	shape: Vec<usize>,
	labels: Vec<String>,
	label_axis: Vec<usize>
}

impl<T: Clone> RkgTabN<T> {

	/// creates a new empty tab
	pub fn new() -> Self {
		Self {
			data: Vec::new(),
			dims: 1,
			shape: vec![0],
			labels: Vec::new(),
			label_axis: Vec::new()
		}
	}

	/// clears the table and resizes it to a new shape filled with given values
	pub fn set_shape(&mut self, shape: &[usize], value: T) {
		self.dims = shape.len();
		self.data = Vec::new();
		self.shape.clear();
		let mut total_length = 1;
		for i in shape {
			total_length *= i;
			self.shape.push(*i);
		}
		self.data.resize(total_length, value);
	}

	/// returns a vec containing the shape of the tab
	pub fn shape(&self) -> Vec<usize> {
		self.shape.clone()
	}

	/// takes in a vector and appends it to the table spicifying the axis
	pub fn append(&mut self, data: &mut Vec<T>, axis: usize) {
		if axis >= self.shape.len() { panic!("axis is not in this tab") }

		if axis == 0 {
			self.data.append(data);
		}
		self.shape[0] += 1;
	}

	pub fn resize(s: Vec<usize>) {

	}

}

impl<T: fmt::Display> fmt::Display for RkgTabN<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut text = String::new();
		for i in self.data.iter() {
			text.push_str(&format!("{}, ", i));
		}
		write!(f, "{}", text)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test] fn test_new_1d_and_print() {
		let mut value: RkgTabN<i32> = RkgTabN::new();
		value.append(&mut [1,2,3,4,5].to_vec(), 0);
		println!("{value}")
	}
}