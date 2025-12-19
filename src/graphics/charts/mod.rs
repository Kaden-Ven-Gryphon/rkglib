
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use crate::graphics::canvas::{Canvas, CanvasShape, Cord};

use super::color::ColorARGB32;
use super::window::RkgWindow;

/// The default width of new charts and chartwindows
pub const DEFAULT_WIDTH: usize = 400;
/// The default hight of new charts and chartwindows
pub const DEFAULT_HEIGHT: usize = 400;

/// Color struct for RGBA 32bit



/// Trait for objects that can be drawn to a ChartWindow must implement
pub trait Chart {
	/// updates its internal canvas and returns a referance to it
	fn draw(&mut self) -> &Canvas;
	/// updates its internal canvas and returns a referance to it
	fn draw_with_time(&mut self, t: f64) -> &Canvas;
	/// returns tuple with the position of the chart
	fn pos(&self) -> Cord;
	/// returns the width and height of the chart as a tuple
	fn shape(&self) -> CanvasShape;
	/// returns width of chart
	fn width(&self) -> usize;
	/// returns height of chart
	fn height(&self) -> usize;
	/// returns x cord of top right of chart
	fn x(&self) -> i32;
	/// returns y cord of top right of chart
	fn y(&self) -> i32;
}

/// Test fractal visulization from the minifb create modified to implement the Chart trait
pub mod test_fractal_chart;
/// 2d scatter plot that uses a RkgTab as data source
pub mod scatter_plot;

// TODO: change this to use standard window and canvas
// chartwindow contains a normal canvas but with time varibles etc, framerate
// has its own show function that copys the base windows one, but has time 

/// This is the default window that can display chart objects
pub struct ChartWindow {
	/// default window struct
	pub rkgwindow: RkgWindow,
	/// minifb window
	pub window: Option<Window>,
	/// the list of charts to draw to window
	pub charts: Vec<Box< dyn Chart>>,
	/// current time
	pub time:f64,
	/// time step size
	pub time_step: f64
}

impl ChartWindow {
	/// makes a ChartWindow with default settings
	pub fn new() -> Self {
		Self {
			rkgwindow: RkgWindow::new("Charts", DEFAULT_WIDTH, DEFAULT_HEIGHT, ColorARGB32(0xFF050F0F)),
			window: None, 
			charts: Vec::new(),
			time: 0.0,
			time_step: 0.1
		}
	}


	/// calls draw on all charts and draws them to window buffer
	pub fn update(&mut self) {
		if self.window.is_some(){
			for i in 0..self.charts.len() {
				let canvas = self.charts[i].draw_with_time(self.time);
				self.rkgwindow.canvas.paste_canvas(canvas, Cord::zero());
			}

			self.window.as_mut().unwrap().update_with_buffer(&self.rkgwindow.buffer, self.rkgwindow.width, self.rkgwindow.height).unwrap()
		} 
	}

	/// Opens window and renders all charts to window
	pub fn show(&mut self) {
		self.window = Some(Window::new(
			self.rkgwindow.name.as_str(),
			self.rkgwindow.width,
			self.rkgwindow.height,
			WindowOptions {
				resize: true,
				scale: Scale::X2,
				scale_mode: ScaleMode::AspectRatioStretch,
				..WindowOptions::default()
			},
			)
			.expect("Unable to create the window")
		);

		self.window.as_mut().unwrap().set_target_fps(60);

		self.window.as_mut().unwrap().set_background_color(
			self.rkgwindow.background_color.r(),
			self.rkgwindow.background_color.g(),
			self.rkgwindow.background_color.b()
		);

		for pixel in self.rkgwindow.buffer.iter_mut() {
			*pixel = self.rkgwindow.background_color.0;
		}

		while self.window.as_ref().unwrap().is_open() && !self.window.as_ref().unwrap().is_key_down(Key::Escape) {
			self.update();
			self.time+=self.time_step;
		}
	}
}


// MARK: Tests
#[cfg(test)]
mod tests {




}