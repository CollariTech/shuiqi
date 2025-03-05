use crate::painter::point::{Measurement, Point};
use crate::painter::reference::Corner;
use crate::painter::text::InnerText;

pub mod point;
pub mod color;
pub mod reference;
pub mod writer;
pub mod text;

pub struct Object {
    screen_point: Point,
    width: Measurement,
    height: Measurement,
    reference_corner: Corner,
    border_radius: Option<Measurement>,
    fill_color: Option<color::Color>,
    text: Option<InnerText>,
}

impl Object {
    pub fn create(
        point: Point,
        width: Measurement,
        height: Measurement
    ) -> Self {
        Self {
            screen_point: point,
            width,
            height,
            reference_corner: Corner::default(),
            border_radius: None,
            fill_color: None,
            text: None
        }
    }

    pub fn colored(
        point: Point,
        width: Measurement,
        height: Measurement,
        color: color::Color
    ) -> Self {
        Self {
            screen_point: point,
            width,
            height,
            reference_corner: Corner::default(),
            border_radius: None,
            fill_color: Some(color),
            text: None
        }
    }

    pub fn border_radius(mut self, new_radius: Measurement) -> Self {
        self.border_radius = Some(new_radius);
        self
    }

    pub fn reference_point(mut self, new_reference: Corner) -> Self {
        self.reference_corner = new_reference;
        self
    }

    pub fn centered(mut self) -> Self {
        self.reference_corner = Corner::Center;
        self
    }

    pub fn background_color(mut self, new_color: color::Color) -> Self {
        self.fill_color = Some(new_color);
        self
    }

    pub fn text(mut self, new_text: InnerText) -> Self {
        self.text = Some(new_text);
        self
    }

    pub fn is_static(&self) -> bool {
        self.has_static_size() && self.has_static_position()
    }

    pub fn has_static_size(&self) -> bool {
        self.width.is_pixels() && self.height.is_pixels()
    }

    pub fn has_static_position(&self) -> bool {
        self.screen_point.x.is_pixels() && self.screen_point.y.is_pixels()
    }
}