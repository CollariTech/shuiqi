use winit::dpi::PhysicalSize;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: Measurement,
    pub y: Measurement,
}

impl Point {
    pub fn new(x: Measurement, y: Measurement) -> Self {
        Self { x, y }
    }

    pub fn from_percentage(x_percent: f32, y_percent: f32) -> Self {
        Self {
            x: Measurement::Percentage(x_percent),
            y: Measurement::Percentage(y_percent),
        }
    }

    pub fn from_pixels(x_pixels: f32, y_pixels: f32) -> Self {
        Self {
            x: Measurement::Pixels(x_pixels),
            y: Measurement::Pixels(y_pixels),
        }
    }

    pub fn to_screen_space(&self, screen_size: PhysicalSize<u32>) -> [f32; 2] {
        [
            self.x.to_screen_space(screen_size.width),
            self.y.to_screen_space(screen_size.height),
        ]
    }

    pub fn to_ndc(&self, screen_size: PhysicalSize<u32>) -> [f32; 2] {
        let [screen_x, screen_y] = self.to_screen_space(screen_size);
        [
            (screen_x / screen_size.width as f32) * 2.0 - 1.0,
            -((screen_y / screen_size.height as f32) * 2.0 - 1.0),
        ]
    }
}

#[derive(Clone, Debug)]
pub enum Measurement {
    Pixels(f32),
    Percentage(f32),
}

impl Measurement {
    pub fn to_screen_space(&self, screen_dimension: u32) -> f32 {
        let screen_size = screen_dimension as f32;
        match self {
            Measurement::Pixels(value) => *value,
            Measurement::Percentage(percent) => (screen_size * *percent) / 100.0,
        }
    }

    pub fn to_size(&self, screen_dimension: u32) -> f32 {
        self.to_screen_space(screen_dimension)
    }
}