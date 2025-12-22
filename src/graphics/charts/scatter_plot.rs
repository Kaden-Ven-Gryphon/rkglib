use std::fmt::Display;

use num_traits::NumCast;
use num_traits::cast;

use super::{DEFAULT_HEIGHT, DEFAULT_WIDTH, Chart, ChartWindow};
use crate::graphics::canvas::Canvas;
use crate::graphics::canvas::CanvasShape;
use crate::graphics::canvas::Cord;
use crate::graphics::color::ColorARGB32;
use crate::math::datatypes::rkgtab::RkgTabN;

/// A 2d scatter plot chart compatible with the ChartWindow
pub struct ScatterPlot<T: Clone + Copy + Display> {
	/// canvas char is drawn to
	pub canvas: Canvas,
	/// postion to paste chart into the window canvas
	pub pos: Cord,
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
	/// creates a new scatter plot using a table of data
	pub fn from_table(table: RkgTabN<T>) -> Self{
		Self {
			canvas: Canvas::new(CanvasShape{width:DEFAULT_WIDTH, height:DEFAULT_HEIGHT, depth:4}, crate::graphics::canvas::CanvasOrigin::BottomLeft),
			pos: Cord::zero(),
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
	fn draw(&mut self) -> &Canvas {
		self.draw_with_time(0.0)
	}

	fn draw_with_time(&mut self, t: f64) -> &Canvas {
		
		let mut cord = vec![0;2];
		for i in 0..self.data_table.shape()[0] {
			cord[0] = i;
			cord[1] = self.x_axis;

			let x:i32 = cast(self.data_table.get(&cord)).unwrap();
			cord[1] = self.y_axis;
			let y:i32 = cast(self.data_table.get(&cord)).unwrap();


			self.canvas.paint(&Cord { x, y }, ColorARGB32(0xFF00FF00));
			

		}
		&self.canvas
	}


	fn height(&self) -> usize {
		self.canvas.height()
	}

	fn pos(&self) -> Cord {
		self.pos
	}

	fn shape(&self) -> CanvasShape {
		self.canvas.get_shape()
	}

	fn width(&self) -> usize {
		self.canvas.width()
	}

	fn x(&self) -> i32 {
		self.pos.x
	}

	fn y(&self) -> i32 {
		self.pos.y
	}
}