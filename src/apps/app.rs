

use crate::pixel_display::pixel_display::PixelDisplay;


pub trait App {
    fn draw(&self, display: &mut PixelDisplay);
}
