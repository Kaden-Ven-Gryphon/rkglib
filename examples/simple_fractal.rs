
use rkglib::graphics::canvas::{CanvasShape, Cord};
use rkglib::graphics::charts::test_fractal_chart::TestFractalChart;
use rkglib::graphics::charts;
fn main() {
	let mut chart_window = charts::ChartWindow::new();
	let mut fractal = TestFractalChart::new();
	fractal.pos = Cord{x: 100, y: 100};
	fractal.canvas.set_shape(CanvasShape{width:300,height:300,depth:4});
	chart_window.charts.push(Box::new(fractal));

	chart_window.rkgwindow.resize(500,500);

	chart_window.rkgwindow.background_color.0 = 0xff00000f;
	chart_window.run_time = true;

	chart_window.show();
}

