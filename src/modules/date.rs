use crate::pixel_display::pixel_display::PixelDisplay;
use chrono::prelude::*;
use embedded_graphics::geometry::Point;

use super::module::Module;

pub struct Date {}

impl Module for Date {
    fn draw(&self, point: Point, display: &mut PixelDisplay) {
        let local: DateTime<Local> = Local::now();
        display.draw_text(format!("{}", local.format("%d.%m")).as_str(), point);
    }
}
