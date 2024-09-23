use winit::dpi::PhysicalSize;

pub struct Point {
    pub x: Measurement,
    pub y: Measurement
}

impl Point {
    pub fn new(x: Measurement, y: Measurement) -> Self {
        Self { x, y }
    }

    pub fn get_screen_position(&self, screen_size: PhysicalSize<u32>) -> [f32; 2] {
        [
            get_measurement_screen_percentage(&self.x, screen_size.width),
            get_measurement_screen_percentage(&self.y, screen_size.height)
        ]
    }
}

#[derive(Clone, Copy)]
pub enum Measurement {
    Pixels(f32),
    Percentage(f32)
}

// we need to convert the measurements to percentage (-1 to 1)
pub fn get_measurement_screen_percentage(
    measurement: &Measurement,
    screen_size: u32
) -> f32 {
    match measurement {
        Measurement::Pixels(value) => value / screen_size as f32,
        Measurement::Percentage(percent) => percent / 50.0 - 1.0
    }
}