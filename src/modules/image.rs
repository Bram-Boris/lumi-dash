

use crate::pixel_display::pixel_display::PixelDisplay;

use embedded_graphics::{geometry::Point, pixelcolor::Rgb888};
use tinybmp::Bmp;

use super::module::Module;

pub struct Image {
    bitmap: Bmp<'static, Rgb888>,
}

impl Image {
    pub fn new(bytes: &'static [u8; 6282]) -> Self {
        Self {
            bitmap: Bmp::<'static, Rgb888>::from_slice(bytes).unwrap(),
        }
    }
}

impl Module for Image {
    fn draw(&self, point: Point, display: &mut PixelDisplay) {
        let image = embedded_graphics::image::Image::new(&self.bitmap, Point::new(0, 0));
        display.draw_image(image, point);
    }
}
