/// modual for dealing with color structs, their converstions, and other operations
pub mod color;

/// This modual contains methods for creating various diffrent types of charts
pub mod charts;

/// This is a modual for the basic canvas object that handles cordinates and drawing opperations
pub mod canvas {
    use crate::graphics::color::{self, ColorARGB32};


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

	impl CanvasShape {
		/// creates a new canvas shape with given width, height, and depth
		pub fn new(width:usize, height:usize, depth:usize) -> Self {
			Self {
				width,height,depth
			}
		}
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
		fn map(&self, cord: Cord) -> usize {
			match self.origin {
				CanvasOrigin::TopLeft => (cord.x+(cord.y* (self.shape.width as i32))) as usize,
				CanvasOrigin::BottomLeft => (cord.x+((self.shape.height as i32 -cord.y)* (self.shape.width as i32))) as usize,
				CanvasOrigin::Center => (cord.x+(self.shape.width as i32 / 2)) as usize,
			}
		}

		/// draws the image on to a flat buffer with given width at pos
		pub fn draw_on_to_buffer(&self, buffer: &mut Vec<u32>, buffer_width: usize, pos: &Cord) {

		}

		/// paints the pixel at position pos the given color
		pub fn paint(&mut self, pos: &Cord, color: ColorARGB32) {

		}
	
		/// gets the color of the pixel at the given position
		pub fn get(&self, pos: &Cord) -> ColorARGB32 {
			return ColorARGB32(0x0)
		}

		/// fills the canvas with a given color
		pub fn fill(&mut self, color: ColorARGB32) {
			for p in self.data.iter_mut() {
				*p = color.0
			}
		}



	}
}