/// modual for dealing with color structs, their converstions, and other operations
pub mod color;

/// This modual contains methods for creating various diffrent types of charts
pub mod charts;

/// This modual contains the basic window container
/// TODO: Add basic control handling, things like frame rate control etc
pub mod window {
	use minifb::{Scale, ScaleMode, Window, WindowOptions, Key};

	use crate::graphics::{canvas::{Canvas, CanvasShape, Cord}, color::ColorARGB32};


	/// container for a basic window storing the basic required atributes
	pub struct RkgWindow {
		/// The title of window displayed on the desktop bar
		pub name: String,
		/// width in pixels
		pub width: usize,
		/// height in pixels
		pub height: usize,
		/// the root canvas that should be the same width and height as window and is drawn to the screen
		pub canvas: Canvas,
		/// the pixel buffer that is drawn to the screen
		pub buffer: Vec<u32>,
		/// the background color of the borders
		pub background_color: ColorARGB32,
		window: Option<Window>
	}

	impl RkgWindow {
		/// creates a new window container with the window set to none
		pub fn new(name: &str, width:usize, height:usize, background_color: ColorARGB32) -> Self {
			Self {
				name:name.to_string(),
				width,
				height,
				canvas: Canvas::new(CanvasShape{width,height,depth:4}, super::canvas::CanvasOrigin::BottomLeft),
				buffer: vec![background_color.0; width*height],
				background_color,
				window: None
			}
		}

		/// opens the window and draws the root canvas to the screen
		pub fn show(&mut self) {
		self.window = Some(Window::new(
			self.name.as_str(),
			self.width,
			self.height,
			WindowOptions {
				resize: true,
				scale: Scale::X1,
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

		self.canvas.draw_on_to_buffer(&mut self.buffer, self.width, &Cord{x:0,y:0});

		

		while self.window.as_ref().unwrap().is_open() && !self.window.as_ref().unwrap().is_key_down(Key::Escape) {
			self.window.as_mut().unwrap().update_with_buffer(&self.buffer, self.width, self.height).unwrap();	
		}
	}
	}
}




/// This is a modual for the basic canvas object that handles cordinates and drawing opperations
pub mod canvas {

	use std::ops::{Add, AddAssign};

use crate::graphics::color::{ColorARGB32};

	/// objects like shapes, lines, curves, gradiants, etc that can be drawn to the canvas
	pub trait CanvasObject {
		/// when given to a canvas the canvas will call this draw function on the object
		fn draw(&self, canvas: &mut Canvas, pos: Cord);
	}

	/// options for canvas origin
	pub enum CanvasOrigin {
		/// 0,0 is the top left pixel
		TopLeft,
		/// 0,0 is the bottom left pixel
		BottomLeft,
		/// 0,0 is the center pixel
		Center,
	}

	/// 2d canvas cord
	#[derive(Clone, Copy)]
	pub struct Cord {
		/// x
		pub x: i32,
		/// y
		pub y: i32,
	}

	impl Cord {
		/// returns new cord with 0,0
		pub fn zero() -> Self {
			Self {x:0, y:0}
		}
		
		/// returns point cliped to fit in canvas
		pub fn clip(&self, canvas: &Canvas) -> Self {
			match canvas.origin {
				CanvasOrigin::TopLeft => {
					let mut cliped = self.clone();
						if cliped.x < 0 { cliped.x = 0; }
						if cliped.y < 0 { cliped.y = 0; }
						if cliped.x > canvas.shape.width as i32 -1 { cliped.x = canvas.shape.width as i32 -1; }
						if cliped.y > canvas.shape.height as i32 -1 { cliped.y = canvas.shape.height as i32 -1; }
					cliped
				},
				CanvasOrigin::BottomLeft => {
					let mut cliped = self.clone();
						if cliped.x < 0 { cliped.x = 0; }
						if cliped.y < 0 { cliped.y = 0; }
						if cliped.x > canvas.shape.width as i32 -1 { cliped.x = canvas.shape.width as i32 -1; }
						if cliped.y > canvas.shape.height as i32 -1 { cliped.y = canvas.shape.height as i32 -1; }
					cliped
				},
				CanvasOrigin::Center => {
					// TODO: Test this to see if it is off by one
					let mut cliped = self.clone();
						if cliped.x < 0 - canvas.shape.width as i32 / 2 { cliped.x = 0 - canvas.shape.width as i32 / 2; }
						if cliped.y < 0 - canvas.shape.height as i32 / 2 { cliped.y = 0 - canvas.shape.height as i32 / 2; }
						if cliped.x > canvas.shape.width as i32 / 2 { cliped.x = canvas.shape.width as i32 / 2; }
						if cliped.y > canvas.shape.height as i32 / 2 { cliped.y = canvas.shape.height as i32 / 2; }
					cliped
				}
			}
		}
	}

	impl Add for Cord {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			Self {x: self.x+rhs.x, y: self.y+rhs.y}
		}
	}

	impl AddAssign for Cord {
		fn add_assign(&mut self, rhs: Self) {
			*self = Self { 
				x: self.x + rhs.x,
				y: self.y + rhs.y
			}
		}
	}

	/// spcifies the height, width, and depth of canvas
	#[derive(Clone)]
	pub struct CanvasShape {
		/// width
		pub width: usize,
		/// height
		pub height: usize,
		/// depth
		pub depth: usize,
	}


	/// Canvas struct that handles basic drawing operations
	pub struct Canvas {
		shape: CanvasShape,
		data: Vec<u32>,
		/// the origin of the canvas
		pub origin: CanvasOrigin,
	}

	impl Canvas {
		/// Creates new canvas with shape and origin
		pub fn new(shape: CanvasShape, origin: CanvasOrigin) -> Self {
			Self {
				data: vec![0; shape.width*shape.height],
				shape,
				origin
			}
		}
	
		/// takes a cord and returns the index of pixel in flatened array
		/// TODO: handle center origin
		fn map(&self, cord: Cord) -> usize {
			match self.origin {
				CanvasOrigin::TopLeft => (cord.x+(cord.y* (self.shape.width as i32))) as usize,
				CanvasOrigin::BottomLeft => (cord.x+(((self.shape.height-1) as i32 -cord.y)* (self.shape.width as i32))) as usize,
				CanvasOrigin::Center => (cord.x+(self.shape.width as i32 / 2)) as usize,
			}
		}

		/// draws the image on to a flat buffer with given width at pos
		pub fn draw_on_to_buffer(&self, buffer: &mut Vec<u32>, buffer_width: usize, pos: &Cord) {
			for i in 0..self.shape.width as i32 {
				for j in 0..self.shape.height as i32 {
					let buffer_index = ((i+pos.x)+((j+pos.y)* (buffer_width as i32))) as usize;
					let flat_index = (i+(j* (self.shape.width as i32))) as usize;

					// TODO add color mixing
					let pixel = self.data[flat_index];

					if pixel & 0xFF000000 != 0 {
						buffer[buffer_index] = pixel
					}
				}
			}
		}

		/// paints the pixel at position pos the given color
		pub fn paint(&mut self, pos: &Cord, color: ColorARGB32) {
			let flat_index = self.map(*pos);
			self.data[flat_index] = color.0
		}
	
		/// gets the color of the pixel at the given position
		pub fn get(&self, pos: &Cord) -> ColorARGB32 {
			return ColorARGB32(self.data[self.map(*pos)])
		}

		/// fills the canvas with a given color
		pub fn fill(&mut self, color: ColorARGB32) {
			for p in self.data.iter_mut() {
				*p = color.0
			}
		}

		

	}

	

}

/// Canvas Objects for common things like lines, text, circls, triangles etc.
pub mod drawing_primitives {
	use std::{cmp::{max, min}};

	use crate::graphics::{canvas::{Canvas, CanvasObject, Cord}, color::ColorARGB32};

	/// Options for how an object is filled
	#[derive(Clone, Copy)]
	pub enum FillType {
		/// Fill the object with its default color or spcified color
		Fill(Option<ColorARGB32>),
		/// Do not fill the object
		NoFill,
	}

	/// Options for how the border is drawn
	#[derive(Clone, Copy)]
	pub enum BorderType {
		/// draw a solid line deault color or spcified color
		Solid(Option<ColorARGB32>),
		/// Do not draw border
		None,
	}

	/// types of lines
	#[derive(Clone, Copy)]
	pub enum LineType {
		/// infinite in both directions
		Line,
		/// infinite in one direction
		Ray,
		/// line segment
		Segment,
	}

	/// style for lines
	#[derive(Clone, Copy)]
	pub enum LineStyle {
		/// solid line
		Solid,
		/// dashed line of seg length
		Dashed(i32),
	}

	enum LineOverlap {
		None,
		Major,
		Minor,
		Both,
	}

	/// types of vertexs
	#[derive(Clone, Copy)]
	pub enum VertexType {
		/// dot at vertex with color of border or spcified color
		Circle(Option<ColorARGB32>),
		/// no added vertex draing
		None,
	}
	/// set of options that most canvas object will use
	/// Object might not use all of this settings or might override them with object spcific settings
	#[derive(Clone, Copy)]
	pub struct DrawOptions {
		/// can be used as the position of the object to draw, or the position of parent
		pub offset: Cord,
		/// scale of object 
		pub scale: Cord,
		/// rotation of object
		pub rotation: f32,
		/// default color of object might be overiden by the fill border or object spcific settings
		pub color: ColorARGB32,
		/// the fill setting for the object
		pub fill: FillType,
		/// the border setting for the object
		pub border: BorderType,
		/// width of border in pixels
		pub boarder_width: f32,
		/// line type setting for object
		pub line_type: LineType,
		/// vertex settings
		pub vertex: VertexType,
	}

	impl DrawOptions {
		/// new default settings
		pub fn new() -> Self {
			Self {
				offset: Cord{x:0, y:0},
				scale: Cord{x:1,y:1},
				rotation: 0.0,
				color: ColorARGB32(0xFFFFFFFF),
				fill: FillType::Fill(None),
				border: BorderType::Solid(None),
				boarder_width: 1.0,
				line_type: LineType::Segment,
				vertex: VertexType::None,
			}
		}
	}

	/// basic line
	pub struct Line {
		/// options for color, fill, offset, type
		pub options: DrawOptions,
		/// first point in line
		pub point_1: Cord,
		/// second point in line
		pub point_2: Cord,
		/// style of line
		pub style: LineStyle,
		/// width of line
		pub width: i32,
	}

	impl Line {
		/// creates a default empty line
		pub fn new() -> Self {
			Self {
				options: DrawOptions::new(),
				point_1: Cord{x:0,y:0},
				point_2: Cord{x:0,y:0},
				style: LineStyle::Solid,
				width: 1,
			}
		}
		/// does not check bounds
		/// Uses Arduino-BlueDisplay algorithm
		fn draw_line_overlap(canvas: &mut Canvas, start: Cord, end: Cord, overlap: LineOverlap, color: ColorARGB32) {
			let mut x = start.x;
			let mut y = start.y;
			
			let mut t_delta_x = end.x - start.x;
			let mut t_delta_y = end.y - start.y;
			let mut t_step_x = 1;
			let mut t_step_y = 1;

			let mut t_error = 0;

			if t_delta_x < 0 {
				t_delta_x *= -1;
				t_step_x = -1;
			}

			if t_delta_y < 0 {
				t_delta_y *= -1;
				t_step_y = -1;
			}

			let t_delta_x_2 = t_delta_x << 1;
			let t_delta_y_2 = t_delta_y << 1;

			canvas.paint(&start, color);

			if t_delta_x > t_delta_y {
				t_error = t_delta_y_2 - t_delta_x;

				while x != end.x {
					x += t_step_x;
					if t_error >= 0 {
						match overlap {
							LineOverlap::Major => {
								canvas.paint(&Cord { x, y }, color);
								y += t_step_y;
							},
							LineOverlap::Minor => {
								y += t_step_y;
								canvas.paint(&Cord { x, y }, color);

							},
							LineOverlap::Both => {
								canvas.paint(&Cord { x, y }, color);
								y += t_step_y;
								canvas.paint(&Cord { x, y }, color);
							},
							LineOverlap::None => {
								y += t_step_y;
							}
						}
						t_error -= t_delta_x_2;
					}
					t_error += t_delta_y_2;
					canvas.paint(&Cord { x, y }, color);
				}
			} else {
				t_error = t_delta_x_2 - t_delta_y;
				while y != end.y {
					y += t_step_y;
					if t_error >= 0 {
						match overlap {
							LineOverlap::Major => {
								canvas.paint(&Cord { x, y }, color);
								x += t_step_x;
							},
							LineOverlap::Minor => {
								x += t_step_x;
								canvas.paint(&Cord { x, y }, color);

							},
							LineOverlap::Both => {
								canvas.paint(&Cord { x, y }, color);
								x += t_step_x;
								canvas.paint(&Cord { x, y }, color);
							},
							LineOverlap::None => {
								x += t_step_x;
							}
						}
						t_error -= t_delta_y_2;
					}
					t_error += t_delta_x_2;
					canvas.paint(&Cord { x, y }, color);
				}
			}
		}
	}

	// TODO: line from point and slope


	impl CanvasObject for Line {
		fn draw(&self, canvas: &mut super::canvas::Canvas, pos: Cord) {

			// TODO: use match on line type to find start and end cords
			let point_1 = self.point_1 + pos + self.options.offset;
			let point_2 = self.point_2 + pos + self.options.offset;
			
			// translate points
			// scale
			// rotate
			// TODO: work on matrix mult operations for transforms

			match self.style {
				LineStyle::Solid => {
					if self.width == 1 {
						Self::draw_line_overlap(canvas, point_1, point_2, LineOverlap::None, self.options.color)
					}
				},
				_ => {}
			}
		}
	}


	/// basic rectange
	pub struct Rectange {
		/// common draw options
		pub options: DrawOptions,
		/// point 1
		pub point_1: Cord,
		/// point 2
		pub point_2: Cord,
	}

	impl Rectange {
		/// create new defualt rec of empty size
		pub fn new() -> Self {
			Self {
				options: DrawOptions::new(),
				point_1: Cord{x:0, y:0},
				point_2: Cord{x:0, y:0},
			}
		}
	}

	impl CanvasObject for Rectange {
		fn draw(&self, canvas: &mut super::canvas::Canvas, pos: Cord) {
			let mut point_1 = self.point_1 + pos + self.options.offset;
			let mut point_2 = self.point_2 + pos + self.options.offset;
			// TODO: Handel Scale

			point_1 = point_1.clip(canvas);
			point_2 = point_2.clip(canvas);

			match self.options.fill {
				FillType::Fill(c) => {
					let x0 = min(point_1.x, point_2.x);
					let y0 = min(point_1.y, point_2.y);
					let xn = max(point_1.x, point_2.x);
					let yn = max(point_1.y, point_2.y);

					let fill_color = match c {
						Some(c) => c,
						None => self.options.color
					};

					for i in x0..xn {
						for j in y0..yn {
							canvas.paint(&Cord{x:i,y:j}, fill_color);
						}
					}
				},
				FillType::NoFill => {}
			}
			

		}
	}

	/// basic circle
	pub struct Circle {
		/// common draw options
		pub options: DrawOptions,
		/// center
		pub center: Cord,
		/// radius
		pub radius: i32,
	}

	impl Circle {
		/// new circle
		pub fn new() -> Self {
			Self {
				options: DrawOptions::new(),
				center: Cord::zero(),
				radius: 0,
			}
		}
	}

	impl CanvasObject for Circle {
		fn draw(&self, canvas: &mut super::canvas::Canvas, pos: Cord) {
			let center = pos + self.center + self.options.offset;
			match self.options.fill {
				FillType::Fill(c) => {
					let fill_color = match c {
						Some(c) => c,
						None => self.options.color
					};
					let p1 = Cord{x: center.x - self.radius, y: center.y - self.radius}.clip(canvas);
					let p2 = Cord{x: center.x + self.radius, y: center.y + self.radius}.clip(canvas);
					for i in p1.x..p2.x {
						for j in (p1.y)..(p2.y) {
							let dist = ((center.x-i) * (center.x-i)) + ((j-center.y) * (j-center.y));
							if dist <= self.radius*self.radius {
								canvas.paint(&Cord { x: i, y: j }, fill_color);
							}
						}
					}
				},
				FillType::NoFill => {}
			}
		}
	}

}

/// a text engine using rusttype to rasterize and draw text to canvas
pub mod drawing_text {
	// text engine canvas object
	// takes in text objects and draws them to canvas

	// text object struct that contains text and possible overrideds for default settings

}