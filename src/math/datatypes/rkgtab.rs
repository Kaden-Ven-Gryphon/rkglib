use core::{fmt, panic};
use std::{fmt::Display};

/// ndarray like data structure
/// ONLY SUPORTS 1 and 2 D VERY INCOMPLETE
pub struct RkgTabN<T> {
	data: Vec<T>,
	dims: usize,
	shape: Vec<usize>,
	labels: Vec<String>,
	label_axis: Vec<usize>
}

impl<T: Clone + Copy + Display> RkgTabN<T> {

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

	/// creates a new table from a flat array and it shape
	/// panics if flat array is not the right length for the shape
	pub fn from_flat(data: &[T], shape: &[usize]) -> Self {
		Self {
			data: Vec::from(data),
			dims: shape.len(),
			shape: Vec::from(shape),
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

	/// set labels from flat array and axis_shapes
	/// panics if labels not correct size for axis, or is axis is out of bounds
	pub fn set_labels_from_flat(&mut self, labels: &[String], axis: &[usize]) {
		if axis.len() > self.dims { panic!("Number of axis is greater than number of dimensions"); }
		let mut expected_count = 1;
		for a in axis.iter() {
			if *a > self.shape.len() { panic!("axis is out of bounds"); }
			expected_count *= self.shape[*a];
		}

		if expected_count != labels.len() { panic!("Count of labels does not match shape of selected axis"); }
		
		self.labels = Vec::from(labels);
		self.label_axis = Vec::from(axis);
	}

	/// Get label using index and axis
	pub fn get_label_by_axis(&self, axis: usize, i:usize) -> String {
		if axis >= self.shape.len() { panic!("axis is out of bounds"); }
		if i >= self.shape[axis] { panic!("i is out of bounds for that axis"); }

		let mut offset = 0;

		for j in 0..axis {
			if self.label_axis.contains(&j) {
				offset+=self.shape[j];
			}
		}

		return self.labels[offset+i].clone()
	}

	/// takes in a vector and appends it to the table spicifying the axis
	pub fn append(&mut self, data: &[T], axis: usize) {
		if axis >= self.shape.len() { panic!("axis is not in this tab") }

		if axis == 0 {
			self.data.append(&mut data.to_vec());
			self.shape[0] += 1;
		}
		
	}

	/// get value using (..., layer, row, collumn) order
	pub fn get(&self, i: &[usize]) -> T {
		if i.len() != self.shape.len() { panic!("Number of index does not match table shape") };

		let mut flat_i = 0;
		let mut accumalated_width = 1;
		for n in (0..self.shape.len()).rev() {
			flat_i += accumalated_width*i[n];
			accumalated_width *= self.shape[n];
		}

		return self.data[flat_i]
	}

	/// drops extra dimensions, drops values that dont fit in new size
	pub fn resize(_s: Vec<usize>) {

	}


	/// returns a String with of a 2d table by axis
	pub fn table2d_as_string(&self, row: usize, col: usize) -> String {
		let mut ret_string = String::new();

		if self.shape.len() < 2 { panic!("Table does not have two or more dimensions"); }

		if row > self.shape.len() { panic!("row out of bounds"); }
		if col > self.shape.len() { panic!("col out of bounds"); }

		let mut col_width = 1;

		if self.label_axis.contains(&col) {
			let mut labels: Vec<String> = Vec::new();
			for l in 0..self.shape[col] {
				let label = self.get_label_by_axis(col, l);
				if col_width < label.len() { col_width = label.len(); }
				labels.push(label);
			}

			col_width+=2;

			ret_string.push('|');

			for l in labels {
				ret_string.push_str(&format!("{0:^1$}|", l, col_width));
			}

			ret_string.push('\n');

			ret_string.push_str(&"-".repeat(ret_string.len()-1));

			ret_string.push('\n');
		}

		let mut cords = self.shape.clone();
		for c in cords.iter_mut() {
			*c = 0 as usize;
		}

		for i in 0..self.shape[row] {
			ret_string.push('|');
			for j in 0..self.shape[col] {
				cords[row] = i;
				cords[col] = j;
				ret_string.push_str(&format!("{0:^1$}|", self.get(&cords), col_width));
			}
			ret_string.push('\n');
		}

		ret_string
	}
}

impl<T: fmt::Display> fmt::Display for RkgTabN<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut text = String::new();
		
		for l in self.labels.iter() {
			text.push_str(&format!("{}, ", l));
		}
		text.push('\n');
		text.push('[');
		text.push(' ');


		for i in self.data.iter() {
			text.push_str(&format!("{}, ", i));
		}

		text.pop();
		text.pop();
		text.push(' ');
		text.push(']');
		write!(f, "{}", text)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const DISPLAY: bool = true;

	#[test] fn test_new_1d_and_print() {
		let mut value: RkgTabN<i32> = RkgTabN::new();
		value.append(&mut [1,2,3,4,5].to_vec(), 0);
		if DISPLAY { println!("{value}") }

	}

	#[test] fn test_2d_indexing() {
		let table = RkgTabN::from_flat(&[1,2,3,4,5,6], &[2,3]);
		assert_eq!(table.get(&[0,0]), 1);
		assert_eq!(table.get(&[1,0]), 4);
		assert_eq!(table.get(&[0,2]), 3);
		assert_eq!(table.get(&[1,1]), 5);
		
	}

	#[test] fn test_new_2d_with_labels() {
		let mut table = RkgTabN::from_flat(&[1,2,3,4,5,6,7,8], &[4,2]);
		table.set_labels_from_flat(&["X axis".to_string(), "Y axis".to_string()], &[1]);
		let tab_string = table.table2d_as_string(0,1);
		if DISPLAY { println!("{tab_string}") }
	}

	#[test] fn test_append_to_2d() {
		let mut table = RkgTabN::from_flat(&[1,2,3,4,5,6,7,8], &[4,2]);
		table.set_labels_from_flat(&["X axis".to_string(), "Y axis".to_string()], &[1]);
		table.append(&[13,15], 0);
		let tab_string = table.table2d_as_string(0,1);
		if DISPLAY { println!("{tab_string}") }
	}

}