use embedded_graphics::geometry::Point;

use crate::{
    modules::{image::Image, module::Module, time::Time},
    pixel_display::pixel_display::PixelDisplay,
};

use super::app::App;

const SAKURA: &'static [u8; 6282] = include_bytes!("../../assets/sakura-bg.bmp");

pub struct MainMenu {
    time: Time,
    background: Image,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let time = Time {};
        let background = Image::new(SAKURA);

        Self { time, background }
    }
}

impl App for MainMenu {
    fn draw(&self, display: &mut PixelDisplay) {
        self.background.draw(Point::new(0, 0), display);
        self.time.draw(Point::new(2, 6), display);
    }
}
