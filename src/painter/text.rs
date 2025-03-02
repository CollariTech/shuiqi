use crate::painter::color::Color;
use crate::painter::reference::Corner;

pub struct InnerText {
    pub content: String,
    pub font_family: glyphon::Family<'static>,
    pub font_size: f32,
    pub line_height: f32,
    pub color: Color,
    pub corner: Corner
}

impl InnerText {
    pub fn new(
        content: String,
        font_family: glyphon::Family<'static>,
        font_size: f32,
        line_height: f32,
        corner: Corner,
        color: Color
    ) -> Self {
        Self {
            content,
            font_family,
            font_size,
            line_height,
            corner,
            color
        }
    }

    pub fn of(text: &str) -> Self {
        Self {
            content: text.to_string(),
            font_family: glyphon::Family::SansSerif,
            font_size: 16.0,
            line_height: 1.0,
            corner: Corner::default(),
            color: Color::black()
        }
    }

    pub fn at(mut self, corner: Corner) -> Self {
        self.corner = corner;
        self
    }

    pub fn centered(mut self) -> Self {
        self.corner = Corner::Center;
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn font_family(mut self, font_family: glyphon::Family<'static>) -> Self {
        self.font_family = font_family;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}