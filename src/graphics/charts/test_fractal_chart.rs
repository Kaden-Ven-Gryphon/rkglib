use crate::graphics::canvas::{Canvas, CanvasOrigin, CanvasShape, Cord};

use super::{DEFAULT_HEIGHT, DEFAULT_WIDTH, Chart, ChartWindow};

/// Fractal pulled from the
pub struct TestFractalChart {
	///canvas
	pub canvas: Canvas,
	/// x pos in window
	pub pos: Cord,
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
			canvas: Canvas::new(CanvasShape{width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT, depth: 4}, CanvasOrigin::BottomLeft),
			pos: Cord::zero(),
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
	
	fn draw(&mut self) -> &Canvas {
		self.draw_with_time(0.0)
	}

	fn draw_with_time(&mut self, t: f64) -> &Canvas {

		let x_min = 0. - self.range;
		let y_min = 0. - self.range;

		let x_max = 0. + self.range;
		let y_max = 0. + self.range;

		for i in 0..self.canvas.width() as i32 {
			for j in 0..self.canvas.height() as i32 {
				let mut real = Self::map(i as f64, 0., self.canvas.width() as f64, x_min, x_max);
				let mut imag = Self::map(j as f64, 0., self.canvas.height() as f64, y_min, y_max);

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

				let pixel_color = if depth == self.fractal_depth {
					0x00
				} else {
					depth * 32 % 255
				};
				self.canvas.paint(&Cord { x: i, y: j }, crate::graphics::color::ColorARGB32(pixel_color));
			}

		}
		
		&self.canvas
	}


	fn pos(&self) -> Cord {
		return self.pos
	}

	fn shape(&self) -> CanvasShape {
		self.canvas.get_shape()
	}

	fn width(&self) -> usize {
		self.canvas.width()
	}

	fn height(&self) -> usize {
		self.canvas.height()
	}

	fn x(&self) -> i32 {
		self.pos.x
	}

	fn y(&self) -> i32 {
		self.pos.y
	}

}
