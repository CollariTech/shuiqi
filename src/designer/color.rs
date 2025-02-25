pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_array(array: [f32; 3]) -> Self {
        Self {
            r: (array[0] * 255.0) as u8,
            g: (array[1] * 255.0) as u8,
            b: (array[2] * 255.0) as u8,
            a: 255,
        }
    }

    pub fn from_array_with_alpha(array: [f32; 4]) -> Self {
        Self {
            r: (array[0] * 255.0) as u8,
            g: (array[1] * 255.0) as u8,
            b: (array[2] * 255.0) as u8,
            a: (array[3] * 255.0) as u8,
        }
    }

    pub fn into_rgb_array(self) -> [f32; 3] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0
        ]
    }

    pub fn into_rgba_array(self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        ]
    }

    pub fn into_rgba_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
