use super::{DEFAULT_HEIGHT, DEFAULT_WIDTH, Chart, ChartWindow};

/// Fractal pulled from the
pub struct TestFractalChart {
	/// width
	pub width: usize,
	/// height
	pub height: usize,
	/// x pos in window
	pub x: usize,
	/// y pos in window
	pub y:usize,
	/// fractal depth
	pub fractal_depth: u32,
	/// generation infinity
	pub generation_infinity: f64,
	/// range
	pub range: f64,
	/// angle
	pub angle: f64,
}
impl TestFractalChart {
	/// Creates a test chart containing a fractal
	pub fn new() -> Self {
		Self {
			width: DEFAULT_WIDTH,
			height: DEFAULT_HEIGHT,
			x: 0,
			y: 0,
			fractal_depth: 64,
			generation_infinity: 16.0,
			range: 2.0,
			angle: 0.0
		}
	}

	fn map(val: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
	start2 + (stop2 - start2) * ((val - start1) / (stop1 - start1))
}
}
impl Chart for TestFractalChart {
	
	fn draw(&self) -> Vec<u32> {
		self.draw_with_time(0.0)
	}

	fn draw_with_time(&self, t: f64) -> Vec<u32> {
		let mut buf = vec![0u32; self.width*self.height];

		let x_min = 0. - self.range;
		let y_min = 0. - self.range;

		let x_max = 0. + self.range;
		let y_max = 0. + self.range;
		for (i, pixel) in buf.iter_mut().enumerate() {
			let mut real = Self::map((i % self.width) as f64, 0., self.width as f64, x_min, x_max);
			let mut imag = Self::map((i / self.height) as f64, 0., self.height as f64, y_min, y_max);

			let mut depth = 0;
			while depth < self.fractal_depth {
				let re = real.powf(2.) - imag.powf(2.);
				let im = 2. * real * imag;

				real = re + (self.angle+t).cos();
				imag = im + (self.angle+t).sin();

				if (real + imag).abs() > self.generation_infinity {
					break; // Leave when achieve infinity
				}
				depth += 1;
			}

			*pixel = if depth == self.fractal_depth {
				0x00
			} else {
				depth * 32 % 255
			};
		}

		return buf
	}

	fn draw_to_buf_with_time(&self, t:f64, buf:&mut Vec<u32>, buf_width:usize, buf_height:usize) {
		let x_min = 0. - self.range;
		let y_min = 0. - self.range;

		let x_max = 0. + self.range;
		let y_max = 0. + self.range;
		for i in 0..(self.width*self.height) {
			let mut real = Self::map((i % self.width) as f64, 0., self.width as f64, x_min, x_max);
			let mut imag = Self::map((i / self.height) as f64, 0., self.height as f64, y_min, y_max);

			let mut depth = 0;
			while depth < self.fractal_depth {
				let re = real.powf(2.) - imag.powf(2.);
				let im = 2. * real * imag;

				real = re + (self.angle+t).cos();
				imag = im + (self.angle+t).sin();

				if (real + imag).abs() > self.generation_infinity {
					break; // Leave when achieve infinity
				}
				depth += 1;
			}

			let global_i = ChartWindow::map(i,self.width, self.x, self.y, buf_width);

			if global_i < buf_width*buf_height {
				buf[global_i] = if depth == self.fractal_depth {
					0x00
				} else {
					depth * 32 % 255
				};
			}
		}
	}

	fn pos(&self) -> (usize, usize) {
		return (0,0)
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
