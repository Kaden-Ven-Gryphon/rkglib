
use rkglib::graphics::charts::test_fractal_chart::TestFractalChart;
use rkglib::graphics::charts;
fn main() {
	let mut chart_window = charts::ChartWindow::new();
	let mut fractal = TestFractalChart::new();
	fractal.x = 100;
	fractal.y = 100;
	fractal.width = 400;
	fractal.height = 400;
	chart_window.charts.push(Box::new(fractal));

	chart_window.width = 600;
	chart_window.height = 600;

	chart_window.background_color.0 = 0xff00000f;

	chart_window.show();
}

