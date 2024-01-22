use crate::pixel_display::pixel_display::PixelDisplay;

use super::launcher::Input;

pub trait App {
    fn draw(&self, display: &mut PixelDisplay);
    fn input(&mut self, input: Input);
}
