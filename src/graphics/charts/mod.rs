
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use super::color::ColorARGB32;

/// The default width of new charts and chartwindows
pub const DEFAULT_WIDTH: usize = 400;
/// The default hight of new charts and chartwindows
pub const DEFAULT_HEIGHT: usize = 400;

/// Color struct for RGBA 32bit



/// Trait for objects that can be drawn to a ChartWindow must implement
pub trait Chart {
	/// returns a vector containing the rendered image of the object
	fn draw(&self) -> Vec<u32>;
	/// returns a vector containing the rendered image of the chart at time t
	fn draw_with_time(&self, t: f64) -> Vec<u32>;
	/// draws the chart to a buffer at time t
	fn draw_to_buf_with_time(&self, t:f64, buf:&mut Vec<u32>, buf_width:usize, buf_height:usize);
	/// returns tuple with the position of the chart
	fn pos(&self) -> (usize, usize);
	/// returns the width and height of the chart as a tuple
	fn shape(&self) -> (usize, usize);
	/// returns width of chart
	fn width(&self) -> usize;
	/// returns height of chart
	fn height(&self) -> usize;
	/// returns x cord of top right of chart
	fn x(&self) -> usize;
	/// returns y cord of top right of chart
	fn y(&self) -> usize;
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
	/// name of window
	pub name: String,
	/// width of window
	pub width: usize,
	/// height of window
	pub height: usize,
	/// background color
	pub background_color: ColorARGB32,
	window: Option<Window>,
	buffer: Vec<u32>,
	/// the list of charts to draw to window
	pub charts: Vec<Box<dyn Chart>>,
	/// current time
	pub time:f64,
	/// time step size
	pub time_step: f64
}

impl ChartWindow {
	/// makes a ChartWindow with default settings
	pub fn new() -> Self {
		Self {
			name: "New Chart".to_string(),
			width: DEFAULT_WIDTH,
			height: DEFAULT_HEIGHT,
			background_color: ColorARGB32(0xffffffff),
			window: None,
			buffer: vec![0xffffffff; 1000 * 1000],
			charts: Vec::new(),
			time: 0.0,
			time_step: 0.1
		}
	}

	/// map point i from space i to space j with space i at pos xy
	pub fn map(
		i:usize,
		i_width:usize,
		x:usize,
		y:usize,
		j_width:usize,
	) -> usize {
		let i_x = i % i_width;
		let i_y = i / i_width;
		return (i_x+x)+(i_y+y)*j_width
	}

	/// calls draw on all charts and draws them to window buffer
	pub fn update(&mut self) {
		if self.window.is_some(){
			for chart in &self.charts {
				let buf = chart.draw_with_time(self.time);
				for (i, pixel) in buf.iter().enumerate() {
					let j = Self::map(
						i,
						chart.shape().0,
						chart.pos().0,
						chart.pos().1,
						self.width,
					);
					if j < self.buffer.len() {
						self.buffer[j] = *pixel
					}
				}
			}

			self.window.as_mut().unwrap().update_with_buffer(&self.buffer, self.width, self.height).unwrap()
		} 
	}

	/// calls draw to buffer on charts to draw directly to window buffer
	pub fn update_direct(&mut self) {
		if self.window.is_some() {
			for chart in &self.charts {
				chart.draw_to_buf_with_time(self.time, &mut self.buffer, self.width, self.height);
			}
			self.window.as_mut().unwrap().update_with_buffer(&self.buffer, self.width, self.height).unwrap()
		}
	}
	/// Opens window and renders all charts to window
	pub fn show(&mut self) {
		self.window = Some(Window::new(
			self.name.as_str(),
			self.width,
			self.height,
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
			self.background_color.r(),
			self.background_color.g(),
			self.background_color.b()
		);

		for pixel in self.buffer.iter_mut() {
			*pixel = self.background_color.0;
		}

		while self.window.as_ref().unwrap().is_open() && !self.window.as_ref().unwrap().is_key_down(Key::Escape) {
			self.update_direct();
			self.time+=self.time_step;
		}
	}
}


// MARK: Tests
#[cfg(test)]
mod tests {




}