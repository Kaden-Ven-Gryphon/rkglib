use std::thread::current;

use rand::{Rng, distr::slice::Empty};
use rkglib::{graphics::{canvas::{Canvas, CanvasObject, CanvasShape, Cord}, charts::{Chart, ChartWindow}, color::ColorARGB32, drawing_primitives::{DrawOptions, Rectange}}, math::datatypes::matrix::Matrix2d};

const DEFAULT_SCALE: usize = 1;

fn main() {
	#[derive(Clone, Copy, PartialEq)]
	enum CellState {
		Empty,
		Tree,
		Burning(f64),
		Ash(f64),
	}

	struct ForestSim {
		pub canvas: Canvas,
		pub pos: Cord,
		pub scale: usize,
		pub forest: [Matrix2d<CellState>; 2],
		pub grow_chance: f64,
		pub spread_chance: f64,
		pub burn_length: f64,
		pub ash_length: f64,
		pub lightning_chance: f64,
		pub tree_color: ColorARGB32,
		pub empty_color: ColorARGB32,
		pub burning_color: ColorARGB32,
		pub ash_color: ColorARGB32,
		last_time: f64,
		current_forest: usize,
	}

	impl ForestSim {
		fn new(width: usize, height: usize) -> Self {
			Self {
				canvas: Canvas::new(CanvasShape{width: width*DEFAULT_SCALE, height: height*DEFAULT_SCALE, depth:4}, rkglib::graphics::canvas::CanvasOrigin::BottomLeft),
				pos: Cord { x: 0, y: 0 },
				scale: DEFAULT_SCALE,
				forest: [Matrix2d::new(CellState::Empty, width, height), Matrix2d::new(CellState::Empty, width, height)],
				grow_chance: 0.04,
				spread_chance: 0.5,
				burn_length: 1.0,
				ash_length: 1.0,
				lightning_chance: 0.01,
				tree_color: ColorARGB32(0xFF00F000),
				empty_color: ColorARGB32(0xFF050505),
				burning_color: ColorARGB32(0xFFF00000),
				ash_color: ColorARGB32(0xFF505050),
				last_time: 0.0,
				current_forest: 0,
			}
		}

		fn update(&mut self, t: f64) {
			let mut rng = rand::rng();
			self.last_time = t;

			let c = self.current_forest;
			let n = if c == 0 { 1 } else { 0 };
			self.current_forest = n;
			let width = self.forest[c].width();

			

			for i in 0..self.forest[c].data.len() {
				self.forest[n].data[i] = self.forest[c].data[i];
			}

			for i in 0..self.forest[c].data.len() {
				// update forest
				match self.forest[c].data[i] {
					// grow tree
					CellState::Empty => {
						// if rng.random_range(0.0..1.0) < self.grow_chance {
						// 	self.forest.data[i] = CellState::Tree;
						// }
					},
					// spread fire
					CellState::Burning(burn_t) => {
						if burn_t <= t {
							self.forest[n].data[i] = CellState::Ash(self.ash_length+(t* rng.random_range(0.1..1.0) as f64));
						}
						else {
							if rng.random_range(0.0..1.0) < self.spread_chance {
								// spread fire to random near tree

								if i+width < self.forest[c].data.len() {
									if self.forest[c].data[i + width] == CellState::Tree {
										self.forest[n].data[i + width] = CellState::Burning(self.burn_length+(t*rng.random_range(1..2) as f64));
									}
								}

								if i > width {
									if self.forest[c].data[i-width] == CellState::Tree {
										self.forest[n].data[i-width] = CellState::Burning(self.burn_length+(t*rng.random_range(1..2) as f64));
									}
								}

								if i+1 < self.forest[c].data.len() {
									if self.forest[c].data[i+1 as usize] == CellState::Tree {
										self.forest[n].data[i+1 as usize] = CellState::Burning(self.burn_length+(t*rng.random_range(1..2) as f64));
									}
								}

								if i as i32 -1 >= 0 {
									if self.forest[c].data[i-1 as usize] == CellState::Tree {
										self.forest[n].data[i-1 as usize] = CellState::Burning(self.burn_length+(t*rng.random_range(1..2) as f64));
									}
								}
							}
						}
					},
					// decay ash
					CellState::Ash(ash_t) => {
						if ash_t <= t {
							self.forest[n].data[i] = CellState::Empty;
						}
						
					},
					CellState::Tree => {
						if rng.random_range(0.0..1.0) < self.grow_chance {
								let dir = rng.random_range(0..4);
								let tar = match dir {
									0 => i + 1,
									1 => if i > 0 {i-1} else {i+1},
									2 => i + width,
									_ => if i > width { i - width} else {i+width},
								};


								if tar < self.forest[c].data.len() {
									if self.forest[c].data[tar] == CellState::Empty {
										self.forest[n].data[tar] = CellState::Tree;
									}
								}
							}
					},
				}

				
			}
			// Roll chance for lightning
			if rng.random_range(0.0..1.0) < self.lightning_chance {
				let x = rng.random_range(0..width);
				let y = rng.random_range(0..self.forest[c].height());
				if self.forest[c][[x,y]] == CellState::Tree {
					self.forest[n][[x,y]] = CellState::Burning(self.burn_length+t);
				}
			}

			if rng.random_range(0.0..1.0) < self.grow_chance*4.0 {
				let x = rng.random_range(0..width);
				let y = rng.random_range(0..self.forest[c].height());
				if self.forest[c][[x,y]] == CellState::Empty {
					self.forest[n][[x,y]] = CellState::Tree;
				}
			}
		}
	}

	impl Chart for ForestSim {
		fn draw(&mut self) -> &rkglib::graphics::canvas::Canvas {
			self.draw_with_time(0.0)
		}
		fn draw_with_time(&mut self, t: f64) -> &rkglib::graphics::canvas::Canvas {
			
			
			
			// update forest
			self.update(t);
			self.update(t);
			self.update(t);
			self.update(t);

			let c = self.current_forest;
			let n = if c == 0 { 1 } else { 0 };

			let mut cell = Rectange{
				options: DrawOptions {
					offset: Cord::zero(),
					scale: Cord{x:1,y:1},
					rotation: rkglib::graphics::drawing_primitives::RotationType::None,
					color: ColorARGB32(0xFF000000),
					fill: rkglib::graphics::drawing_primitives::FillType::Fill(None),
					border: rkglib::graphics::drawing_primitives::BorderType::None,
					boarder_width: 0.0,
					line_type: rkglib::graphics::drawing_primitives::LineType::Segment,
					vertex: rkglib::graphics::drawing_primitives::VertexType::None,
				},
				point_1: Cord::zero(),
				point_2: Cord::zero(),
			};

			for i in 0..self.forest[c].width() {
				for j in 0..self.forest[c].height() {
					
					if true {
						let pos = Cord{x: (i*self.scale) as i32, y: (j*self.scale) as i32} + self.pos;
						let color = match self.forest[c][[i,j]] {
							CellState::Empty => self.empty_color,
							CellState::Tree => self.tree_color,
							CellState::Burning(t) => self.burning_color,
							CellState::Ash(t) => self.ash_color,
						};

						self.canvas.paint(&pos, color);

						
					}
				}
			}
			

			&self.canvas
		}

		fn pos(&self) -> rkglib::graphics::canvas::Cord {
			
			self.pos.clone()
		}
		fn shape(&self) -> rkglib::graphics::canvas::CanvasShape {
			self.canvas.get_shape()
		}
		fn height(&self) -> usize {
			self.forest[0].height()
		}
		fn width(&self) -> usize {
			self.forest[0].width()
		}
		fn x(&self) -> i32 {
			self.pos.x
		}
		fn y(&self) -> i32 {
			self.pos.y
		}
	}




	// START

	let mut forest = ForestSim::new(200, 200);

	let mut chart_window = ChartWindow::new();
	
	
	chart_window.rkgwindow.resize(forest.canvas.width(), forest.canvas.height());

chart_window.charts.push(Box::new(forest));

	chart_window.rkgwindow.background_color.0 = 0xFF00000F;

	chart_window.run_time = true;

	chart_window.show();

}