// MARK: TSetElement trait
/// This is a trait that wraps together orther traits that the generic T
/// is required to have to be used in a TSet set
pub trait TSetElement: Clone + Ord + Copy {}
impl <T: Clone + Ord + Copy> TSetElement for T {}



// MARK: TSet trait
/// This trait lists the required functions for a datatype to operate as a set
pub trait TSet<'a, T> {

	/// returns a new Set with empty vecs
	fn new() -> Self;

	/// returns a new Set initilized with an array
	fn new_from_array(array: &'a[T]) -> Self;

	/// returns a new Set intilized as a subset of anouther set
	fn subset_from_range(&'a self, start: usize, end: usize) -> Self;

	/// returns the usize of the set
	fn size(&self) -> usize;

	/// returns true if this it has a parent space
	fn is_subset(&self) -> bool;

	/// returns the usize of the parent space if is exisists
	fn parent_size(&self) -> Option<usize>;

	/// returns true if set is contained in other
	fn is_subset_of(&self, other: &Self) -> bool;
	
	/// returns a set that is the union of a and b
	/// will also modifiy the parent set if it exists
	fn union(a: &Self, b: &Self) -> Self;
}

// MARK: VecSet
// region: VecSet
/// Implementation of a set using vec internally
pub struct VecSet<'a, T> {
	set: Vec<T>,
	// TODO make parent an enum so it can be pointer to other VecSet, a Vec or None
	parent: Option<&'a VecSet<'a, T>>
}

// Gives Set the ability to use slices
impl<'a, T, Idx> std::ops::Index<Idx> for VecSet<'a, T>
where Idx: std::slice::SliceIndex<[T]>, {
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.set[index]
	}
}

impl<'a, T: TSetElement> TSet<'a, T> for VecSet<'a, T> {

	fn new() -> Self {
		Self {
			set: Vec::new(),
			parent: None
		}
	}

	fn new_from_array(array: &'a[T]) -> Self {
		Self { set: array.to_vec() , parent: None }
	}

	fn subset_from_range(&'a self, start: usize, end: usize) -> Self {
		Self { 
			set: self.set[start..end].to_vec(),
			parent: Some(&self)
		}
	}

	fn is_subset(&self) -> bool {
		match self.parent {
			None => return false,
			_ => return true
		}
	}

	fn size(&self) -> usize {
		self.set.len()
	}

	fn parent_size(&self) -> Option<usize> {
		match self.parent {
			None => None,
			Some(p) => Some(p.size())
		}
	}

	fn is_subset_of(&self, other: &Self) -> bool {
		if self.size() > other.size() { return false }
		
		let mut start_index = 0;
		let mut own = self.set.clone();
		let mut oth = other.set.clone();

		own.sort();
		oth.sort();

		for i in own {
			if start_index >= oth.len() { return false }
			let mut found = false;

			for j in start_index..oth.len() {
				if i == oth[j] {
					start_index = j + 1;
					found = true;
					break;
				}
			}

			if !found { return false }
		}

		return true
	}

	fn union(a: &Self, b: &Self) -> Self {
		// the union of the two sets to return
		let mut union_set = Self::new();

		// copy set a into the union
		for i in a.set.iter() {
			union_set.set.push(*i);
		}

		// sort the union (which is a) and sort a mutable copy of b
		union_set.set.sort();
		let mut b_copy = b.set.clone();
		b_copy.sort();

		// loop through b adding its elements to the union
		let mut index = 0;
		for element in b_copy {

			// no need to check a if all elements of a are alrady checked 
			if index >= a.size() {
				union_set.set.push(element);
			}
			else {
				// loop through the union untill the elements are no longer
				// less than the next element of b
				while index < a.size() && union_set[index] < element {
					index += 1;
				}
				// if the current union element is not same as the next
				// element of b then it is greater than it.
				// add it to the union.
				// if index is past the len of set a then all remaining
				// elements of b are not in a
				if index >= a.size() || union_set[index] != element {
					union_set.set.push(element);
				}
				else {
					index += 1;
				}
				
				// if the elements are the same then 
			}
		}

		return union_set
	}
}
// endregion:

// MARK: Set
// region: ArraySet
/// This is a repensentation of a set theory set
pub struct ArraySet<'a, T> {
	parent_space: Option<&'a [T]>,
	set: &'a [T]
}

// Gives Set the ability to use slices
impl<'a, T, Idx> std::ops::Index<Idx> for ArraySet<'a, T>
where
	Idx: std::slice::SliceIndex<[T]>, {
		type Output = Idx::Output;

		fn index(&self, index: Idx) -> &Self::Output {
			&self.set[index]
		}
	}

impl<'a, T> ArraySet<'a, T> {
	/// Incomplete
	pub fn new_from_array(array: &'a [T]) -> Self {
		Self {parent_space: None, set: array}
	}
	/// Incomplete
	pub fn subset_from_range(&self, start: usize, end: usize) -> Self {
		Self { parent_space: Some(self.set), set: &self.set[start..end]}
	}
	/// Incomplete
	pub fn size(&self) -> usize {
		self.set.len()
	}
	/// Incomplete
	pub fn is_subset(&self) -> bool {
		match self.parent_space {
			None => return false,
			_ => return true
		}
	}
	/// Incomplete
	pub fn parent_size(&self) -> Option<usize> {
		match self.parent_space {
			None => return None,
			Some(parent) => return Some(parent.len())
		}
	}
}
// endregion:



// MARK: Tests
#[cfg(test)]
mod tests {
	use super::*;

// region: test_new_from_array
	fn test_new_from_array(value: &[i32]) {
		let set = VecSet::new_from_array(value);
		assert_eq!(set.set, value);
		assert_eq!(set.is_subset(), false);
	}

	#[test] fn test_new_from_array_0() { test_new_from_array(&[1,2,3,4,5]); }
	#[test] fn test_new_from_array_1() { test_new_from_array(&[0; 0]); }
// endregion:
	
// region: test_subset_from_range
	fn test_subset_from_range(
		set: &VecSet<i32>,
		start: usize,
		end: usize,
		expected: &[i32]
	) {
		let subset = set.subset_from_range(start, end);
		assert_eq!(subset.set, expected);
		assert_eq!(subset.is_subset(), true);
	}

	#[test] fn test_subset_from_range_0() { test_subset_from_range(
		&VecSet::new_from_array(&[1,2,3,4,5,6,7]),
		2,
		5,
		&[3,4,5]
	);}

	#[test] fn test_subset_from_range_1() { test_subset_from_range(
		&VecSet::new_from_array(&[1,2,3,4,5,6,7]),
		0,
		0,
		&[0;0]
	);}
// endregion:
	
// region: test_is_subset
	fn test_is_subset(set: &VecSet<i32>, expected: bool) {
		assert_eq!(set.is_subset(), expected)
	}

	#[test] fn test_is_subset_0() { test_is_subset(
		&VecSet::new_from_array(&[1,2,3,4,5]),
		false
	)}

	#[test] fn test_is_subset_1() { test_is_subset(
		&VecSet::new_from_array(&[1,2,3,4,5]).subset_from_range(
			0,
			3
		),
		true
	)}
// endregion:

// region: test_size
	fn test_size(set: &VecSet<i32>, expected: usize) {
		assert_eq!(set.size(), expected)
	}

	#[test] fn test_size_0() { test_size(
		&VecSet::new_from_array(&[1,2,3,4,5]),
		5
	);}

	#[test] fn test_size_1() { test_size(
		&VecSet::new_from_array(&[0;0]),
		0
	);}
// endregion:

// region: test_parent_size
	fn test_parent_size(subset: &VecSet<i32>, expected: Option<usize>) {
		assert_eq!(subset.parent_size(), expected)
	}

	#[test] fn test_parent_size_0() { test_parent_size(
		&VecSet::new_from_array(&[1,2,3,4,5]),
		None
	);}

	#[test] fn test_parent_size_1() { test_parent_size(
		&VecSet::new_from_array(&[1,2,3,4,5]).subset_from_range(
			0,
			3
		),
		Some(5)
	);}
	

// endregion:

// region: test_is_subset_of
	fn test_is_subset_of(set: VecSet<i32>, other: VecSet<i32>, expected: bool) {
		assert_eq!(set.is_subset_of(&other), expected)
	}

	#[test] fn test_is_subset_of_0() { test_is_subset_of(
		VecSet::new_from_array(&[3,2,6]), 
		VecSet::new_from_array(&[1,2,3,4,5,6,7]), 
	true
	);}

	#[test] fn test_is_subset_of_1() { test_is_subset_of(
		VecSet::new_from_array(&[3,2,6]), 
		VecSet::new_from_array(&[6,2,3,4,5,1,7]), 
	true
	);}

	#[test] fn test_is_subset_of_2() { test_is_subset_of(
		VecSet::new_from_array(&[3,2,6]), 
		VecSet::new_from_array(&[1,3,4,5,6,7]), 
		false
	);}

	#[test] fn test_is_subset_of_3() { test_is_subset_of(
		VecSet::new_from_array(&[3,2,6]), 
		VecSet::new_from_array(&[1,7]), 
		false
	);}
// endregion:

// region: test_union
	fn test_union(a: VecSet<i32>, b: VecSet<i32>, expected: VecSet<i32>) {
		let mut union = VecSet::union(&a, &b);
		union.set.sort();
		assert_eq!(union.set, expected.set);
	}

	#[test] fn test_union_0() { test_union(
		VecSet::new_from_array(&[1,2,3]), 
		VecSet::new_from_array(&[3,4,5]), 
		VecSet::new_from_array(&[1,2,3,4,5])
	);}

	#[test] fn test_union_1() { test_union(
		VecSet::new_from_array(&[1,2,3]), 
		VecSet::new_from_array(&[3,3,4,5]), 
		VecSet::new_from_array(&[1,2,3,3,4,5])
	);}

	#[test] fn test_union_2() { test_union(
		VecSet::new_from_array(&[3,2,1]), 
		VecSet::new_from_array(&[3,3,4,5]), 
		VecSet::new_from_array(&[1,2,3,3,4,5])
	);}

	#[test] fn test_union_3() { test_union(
		VecSet::new_from_array(&[3,2,3,1]), 
		VecSet::new_from_array(&[3,3,3,4,5]), 
		VecSet::new_from_array(&[1,2,3,3,3,4,5])
	);}

	#[test] fn test_union_4() { test_union(
		VecSet::new_from_array(&[0;0]), 
		VecSet::new_from_array(&[3,3,3,4,5]), 
		VecSet::new_from_array(&[3,3,3,4,5])
	);}

	#[test] fn test_union_5() { test_union(
		VecSet::new_from_array(&[4,5,6]), 
		VecSet::new_from_array(&[1,2,3]), 
		VecSet::new_from_array(&[1,2,3,4,5,6])
	);}

	#[test] fn test_union_6() { test_union(
		VecSet::new_from_array(&[1,4,5,6]), 
		VecSet::new_from_array(&[1,2,3]), 
		VecSet::new_from_array(&[1,2,3,4,5,6])
	);}
// endregion:

}