use std::fmt::Display;

use num_traits::NumCast;
use num_traits::cast;

use super::{DEFAULT_HEIGHT, DEFAULT_WIDTH, Chart, ChartWindow};
use crate::math::datatypes::rkgtab::RkgTabN;

/// A 2d scatter plot chart compatible with the ChartWindow
pub struct ScatterPlot<T: Clone + Copy + Display> {
	/// width
	pub width: usize,
	/// height
	pub height: usize,
	/// x pos in window
	pub x: usize,
	/// y pos in window
	pub y:usize,
	/// data that the scatter plot displays, must be 2d
	pub data_table: RkgTabN<T>,
	/// which index to use for the charts x axis
	pub x_axis: usize,
	/// which index to use for the charts y axis
	pub y_axis: usize,
	/// print the axis
	pub show_axis: bool,
	/// print the axis labels
	pub show_labels: bool,
	/// id name pairs of classes
	pub classifications: Vec<(u32,String)>,
	/// show the key
	pub show_key: bool,
	/// dot radius px
	pub point_radius: usize
}

impl<T: Copy+Clone+Display> ScatterPlot<T> {
	pub fn from_table(table: RkgTabN<T>) -> Self{
		Self {
			width: DEFAULT_WIDTH,
			height: DEFAULT_HEIGHT,
			x:0,
			y:0,
			data_table: table,
			x_axis:0,
			y_axis:1,
			show_axis:true,
			show_labels:true,
			classifications: Vec::new(),
			show_key: true,
			point_radius: 5,
		}
	}
}


impl<T: NumCast+Clone+Copy+Display> Chart for ScatterPlot<T> {
	fn draw(&self) -> Vec<u32> {
		let mut buf = Vec::new();

		return buf
	}

	fn draw_with_time(&self, t: f64) -> Vec<u32> {
		let mut buf = Vec::new();

		return buf
	}

	fn draw_to_buf_with_time(&self, t:f64, buf:&mut Vec<u32>, buf_width:usize, buf_height:usize) {
		let mut cord: Vec<usize> = vec![0; 2];
		
		for i in 0..self.data_table.shape()[0] {
			cord[0] = i;
			cord[1] = self.x_axis;

			let x:i32 = cast(self.data_table.get(&cord)).unwrap();
			cord[1] = self.y_axis;
			let y:i32 = cast(self.data_table.get(&cord)).unwrap();
			let y_flip = -y + (self.height as i32);

			let index = x + y_flip* self.width as i32;

			if index > 0 {
				let map_index = ChartWindow::map(index as usize, self.width, self.x, self.y, buf_width);

				buf[map_index] = 0xFF00FF00;
			}


			
		}
	}

	fn pos(&self) -> (usize, usize) {
		return (self.x, self.y)
	}

	fn shape(&self) -> (usize, usize) {
		return (self.width, self.height)
	}

	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}

	fn x(&self) -> usize {
		self.x
	}

	fn y(&self) -> usize {
		self.y
	}
}