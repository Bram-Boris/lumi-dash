use crate::pixel_display::pixel_display::PixelDisplay;
use chrono::prelude::*;
use embedded_graphics::geometry::Point;

use super::module::Module;

pub struct Time {}

impl Module for Time {
    fn draw(&self, point: Point, display: &mut PixelDisplay) {
        let local: DateTime<Local> = Local::now();
        display.draw_text(format!("{}", local.format("%R")).as_str(), point);
    }
}
