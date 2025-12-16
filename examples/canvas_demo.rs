use rkglib::graphics::{color::ColorARGB32, window::RkgWindow};



fn main() {
	let mut window = RkgWindow::new("Demo", 400, 400, ColorARGB32(0xFF050505));

	window.show();
}