use crate::painter::reference::Corner;
use winit::dpi::PhysicalSize;

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: Measurement,
    pub y: Measurement,
}

impl Point {
    pub fn new(x: Measurement, y: Measurement) -> Self {
        Self { x, y }
    }

    pub fn positioned(
        x: f32,
        y: f32,
        width_px: f32,
        height_px: f32,
        corner: Corner
    ) -> Point {
        let (center_x, center_y) = match corner {
            Corner::TopLeft => (x + width_px / 2.0, y + height_px / 2.0),
            Corner::TopRight => (x - width_px / 2.0, y + height_px / 2.0),
            Corner::BottomLeft => (x + width_px / 2.0, y - height_px / 2.0),
            Corner::BottomRight => (x - width_px / 2.0, y - height_px / 2.0),
            Corner::Center => (x, y)
        };
        Point::from_pixels(center_x, center_y)
    }

    pub fn from_percentage(x_percent: f32, y_percent: f32) -> Self {
        Self {
            x: Measurement::Percentage(x_percent),
            y: Measurement::Percentage(y_percent),
        }
    }

    pub fn center() -> Self {
        Self::from_percentage(50.0, 50.0)
    }

    pub fn from_pixels(x_pixels: f32, y_pixels: f32) -> Self {
        Self {
            x: Measurement::Pixels(x_pixels),
            y: Measurement::Pixels(y_pixels),
        }
    }

    pub fn to_screen_space(&self, screen_size: PhysicalSize<u32>) -> [f32; 2] {
        [
            self.x.transform_with_bound(screen_size.width),
            self.y.transform_with_bound(screen_size.height),
        ]
    }

    pub fn transform_with_bounds(&self, width: u32, height: u32) -> [f32; 2] {
        [
            self.x.transform_with_bound(width),
            self.y.transform_with_bound(height),
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

#[derive(Clone, Debug, Copy)]
pub enum Measurement {
    Pixels(f32),
    Percentage(f32)
}

impl Measurement {
    pub fn transform_with_bound(&self, screen_dimension: u32) -> f32 {
        self.broken_transform_with_bound(screen_dimension as f32)
    }

    // todo: come up with a better name
    pub fn broken_transform_with_bound(&self, screen_dimension: f32) -> f32 {
        match self {
            Measurement::Pixels(value) => *value,
            Measurement::Percentage(percent) => ((screen_dimension) * *percent) / 100.0
        }
    }
}