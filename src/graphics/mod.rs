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

	use crate::graphics::color::{ColorARGB32};

	/// objects like shapes, lines, curves, gradiants, etc that can be drawn to the canvas
	pub trait CanvasObject {
		/// when given to a canvas the canvas will call this draw function on the object
		fn draw(canvas: &mut Canvas, pos: Cord);
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
		origin: CanvasOrigin,
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
					let flat_index = self.map(Cord { x: i, y: j });

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