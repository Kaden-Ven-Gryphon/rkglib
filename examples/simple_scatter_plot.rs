
use rkglib::graphics::canvas::{CanvasShape, Cord};
use rkglib::graphics::charts::scatter_plot::ScatterPlot;
use rkglib::graphics::charts;
use rkglib::math::datatypes::rkgtab::{RkgTabN};

use rand::prelude::*;
fn main() {
	let mut chart_window = charts::ChartWindow::new();
	let mut data: Vec<i32> = Vec::new();

	for i in 0..50 {
		data.push(i);
		data.push(i+rand::random_range(-10..10))
	}

	let mut scatter = ScatterPlot::from_table(
		RkgTabN::from_flat(&data, &[50,2])
	);
	scatter.pos = Cord{x: 50, y: 50};
	scatter.canvas.set_shape(CanvasShape{width: 200, height: 200, depth: 4});
	chart_window.charts.push(Box::new(scatter));

	chart_window.rkgwindow.canvas.set_shape(CanvasShape { width: 300, height: 300, depth: 4 });

	chart_window.rkgwindow.background_color.0 = 0xff00000f;

	chart_window.show();
}

