use shuiqi::painter::color::Color;
use shuiqi::painter::Object;
use shuiqi::painter::point::{Measurement, Point};
use shuiqi::painter::text::InnerText;
use shuiqi::painter::writer::draw_objects;
use shuiqi::render::Renderer;
use shuiqi::ShuiqiApp;

#[tokio::main]
async fn main() {
    let mut app = ShuiqiApp::create();
    app.intercept_render(|renderer, _size| {
        renderer.reset();
        let rectangle = Object::colored(
            Point::from_pixels(12.0, 12.0),
            Measurement::Pixels(50.0),
            Measurement::Pixels(50.0),
            Color::new(255, 0, 0),
        );

        let circle = Object::colored(
            Point::from_pixels(25.0, 300.0),
            Measurement::Pixels(50.0),
            Measurement::Pixels(50.0),
            Color::new(0, 255, 0),
        )
            .border_radius(Measurement::Percentage(50.0));

        let rounded_rectangle = Object::colored(
            Point::from_percentage(50.0, 50.0),
            Measurement::Pixels(300.0),
            Measurement::Pixels(120.0),
            Color::new(171, 119, 206),
        )
            .centered()
            .border_radius(Measurement::Pixels(64.0))
            .text(
                InnerText::of("Hello, world!")
                    .font_size(32.0)
                    .centered()
                    .color(Color::white()),
            );

        draw_objects(renderer, vec![rectangle, circle, rounded_rectangle]);
    });
    app.start();
}