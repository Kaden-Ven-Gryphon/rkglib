use std::ops::{Index, IndexMut};


/// A 2d matrix, can be used as a general 2d array
pub struct Matrix2d<T> {
	/// flat data for matrix not safe to change length of
	pub data: Vec<T>,
	width: usize,
	height: usize,
}

impl<T> Index<[usize; 2]> for Matrix2d<T> {
	type Output = T;
	fn index(&self, index: [usize; 2]) -> &Self::Output {
		assert!(index[0] < self.width);
		assert!(index[1] < self.height);
		&self.data[index[0]+index[1]*self.width]
	}
}

impl<T> IndexMut<[usize; 2]> for Matrix2d<T> {
	fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
		&mut self.data[index[0]+index[1]*self.width]
	}
}

impl<T: Clone> Matrix2d<T> {
	/// create a new matrix is set size, filled with given value
	pub fn new(value: T, width:usize, height: usize) -> Self {
		Self { data: vec![value; width*height], width, height }
	}

	/// returns width of matrix
	pub fn width(&self) -> usize {
		self.width
	}

	/// returns height of matrix
	pub fn height(&self) -> usize {
		self.height
	}
}
