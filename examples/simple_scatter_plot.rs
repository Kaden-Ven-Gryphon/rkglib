
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
	scatter.x = 50;
	scatter.y = 50;
	scatter.width = 200;
	scatter.height = 200;
	chart_window.charts.push(Box::new(scatter));

	chart_window.width = 300;
	chart_window.height = 300;

	chart_window.background_color.0 = 0xff00000f;

	chart_window.show();
}

