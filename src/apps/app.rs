use crate::pixel_display::pixel_display::PixelDisplay;

use super::launcher::Input;

pub trait App {
    fn draw(&mut self, display: &mut PixelDisplay);
    fn enable(&mut self);
    fn disable(&mut self);
    fn input(&mut self, input: Input);
}
