
use std::{ops::{Deref, DerefMut}, ptr::null};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

/// Color struct for RGBA 32bit
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorRGBA32(u32);
impl From<u32> for ColorRGBA32 {
	fn from(raw: u32) -> Self {
		Self(raw)
	}
}
impl AsRef<u32> for ColorRGBA32 {
	fn as_ref(&self) -> &u32 {
		&self.0
	}
}
impl Deref for ColorRGBA32 {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for ColorRGBA32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ColorRGBA32 {

}

/// Color struct for RGB 24bit
pub struct ColorRGB24(u32);


/// This is the default window that can display chart objects
pub struct ChartWindow {
	name: String,
	width: usize,
	height: usize,
	pub background_color: ColorRGB24,
	pub window: Option<Window>,
	pub buffer: Vec<u32>,
	
}

impl ChartWindow {
	pub fn new() -> Self {
		Self {
			name: "New Chart".to_string(),
			width: 400,
			height: 400,
			background_color: ColorRGB24(0xff),
			window: None,
			buffer: vec![0u32; 400 * 400]
		}
	}
	pub fn update(&mut self) {
		if self.window.is_some(){
			self.window.as_mut().unwrap().update_with_buffer(&self.buffer, self.width, self.height).unwrap()
		} 
	}
	pub fn show(&mut self) {
		self.window = Some(Window::new(
			self.name.as_str(),
			self.width,
			self.height,
			WindowOptions {
				resize: true,
				scale: Scale::X4,
				scale_mode: ScaleMode::AspectRatioStretch,
				..WindowOptions::default()
			},
			)
			.expect("Unable to create the window")
		);

		self.window.as_mut().unwrap().set_target_fps(60);

		self.window.as_mut().unwrap().set_background_color(0, 0, 20);

		self.update();
	}
}


// MARK: Tests
#[cfg(test)]
mod tests {

	use super::*;
	#[test] fn assign_u32_to_color_rgba_32() {
		let mut value = ColorRGBA32(42);
		assert_eq!(*value, 42);
		*value = 0x37;
		assert_eq!(*value, 0x37);
	}

}