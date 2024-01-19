use embedded_graphics::geometry::Point;

use crate::pixel_display::pixel_display::PixelDisplay;

pub trait Module {
    fn draw(&self, point: Point, display: &mut PixelDisplay);
}
