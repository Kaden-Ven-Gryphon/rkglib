/// Wrapper for u32 to store a RGBA 8bit color
#[derive(Clone, Copy)]
pub struct ColorARGB32(pub u32);

impl ColorARGB32 {
	/// Get red channel
	pub fn r(&self) -> u8 {
		return (self.0 & 0x00ff0000 >> 16) as u8 
	}
	
	/// Get green channel
	pub fn g(&self) -> u8 {
		return (self.0 & 0x0000ff00 >> 8) as u8 
	}
	
	/// Get blue channel
	pub fn b(&self) -> u8 {
		return (self.0 & 0x000000ff) as u8 
	}

	/// Get alpha channel
	pub fn a(&self) -> u8 {
		return (self.0 & 0xff000000 >> 24) as u8 
	}
	
	/// Get red from u32
	pub fn red(c:u32) -> u8 {
		return (c & 0x00ff0000 >> 16) as u8
	}

	/// Get blue from u32
	pub fn blue(c:u32) -> u8 {
		return (c & 0x0000ff00 >> 8) as u8
	}

	/// Get red from u32
	pub fn green(c:u32) -> u8 {
		return (c & 0x000000ff) as u8
	}

	/// Get red from u32
	pub fn alpha(c:u32) -> u8 {
		return (c & 0xff000000 >> 24) as u8
	}

	/// new color with new alpha
	pub fn new_alpha(&self, a: u8) -> Self {
		Self(
			(0x00FFFFFF & self.0) | ((a as u32) << 24)
		)
	}
}