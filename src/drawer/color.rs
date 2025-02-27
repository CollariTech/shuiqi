use glyphon::cosmic_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0
        ]
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        ]
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<glyphon::cosmic_text::Color> for Color {
    fn into(self) -> cosmic_text::Color {
        cosmic_text::Color::rgba(self.r, self.g, self.b, self.a)
    }
}

impl From<cosmic_text::Color> for Color {
    fn from(color: cosmic_text::Color) -> Self {
        Self {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a()
        }
    }
}

impl From<[f32; 3]> for Color {
    fn from(array: [f32; 3]) -> Self {
        Self {
            r: (array[0] * 255.0) as u8,
            g: (array[1] * 255.0) as u8,
            b: (array[2] * 255.0) as u8,
            a: 255
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(array: [f32; 4]) -> Self {
        Self {
            r: (array[0] * 255.0) as u8,
            g: (array[1] * 255.0) as u8,
            b: (array[2] * 255.0) as u8,
            a: (array[3] * 255.0) as u8
        }
    }
}

impl From<[u8; 3]> for Color {
    fn from(array: [u8; 3]) -> Self {
        Self { r: array[0], g: array[1], b: array[2], a: 255 }
    }
}

impl From<[u8; 4]> for Color {
    fn from(array: [u8; 4]) -> Self {
        Self { r: array[0], g: array[1], b: array[2], a: array[3] }
    }
}