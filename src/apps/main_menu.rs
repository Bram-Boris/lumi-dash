use std::collections::VecDeque;

use embedded_graphics::geometry::Point;

use crate::{
    modules::{date::Date, image::Image, module::Module, time::Time},
    pixel_display::pixel_display::PixelDisplay,
};

use super::{app::App, launcher::Input};

const SAKURA: &[u8; 6282] = include_bytes!("../../assets/sakura.bmp");
const CLOUD: &[u8; 1186] = include_bytes!("../../assets/cloud.bmp");
const FOREST: &[u8; 1190] = include_bytes!("../../assets/forest.bmp");
const NIGHT: &[u8; 6282] = include_bytes!("../../assets/night.bmp");
const ART: &[u8; 6282] = include_bytes!("../../assets/art.bmp");
const OASIS: &[u8; 6282] = include_bytes!("../../assets/oasis.bmp");

pub struct MainMenu<'a> {
    time: Time,
    date: Date,
    backgrounds: VecDeque<(Image<'a>, Point, Point)>,
}

impl<'a> MainMenu<'a> {
    pub fn new() -> MainMenu<'a> {
        let time: Time = Time {};
        let date: Date = Date {};

        let mut backgrounds: VecDeque<(Image<'a>, Point, Point)> = VecDeque::new();
        backgrounds.push_back((Image::new(SAKURA), Point::new(2, 6), Point::new(23, 6)));
        backgrounds.push_back((Image::new(CLOUD), Point::new(32, 6), Point::new(48, 6)));
        backgrounds.push_back((Image::new(FOREST), Point::new(2, 6), Point::new(23, 6)));
        backgrounds.push_back((Image::new(NIGHT), Point::new(20, 28), Point::new(43, 28)));
        backgrounds.push_back((Image::new(ART), Point::new(20, 28), Point::new(43, 28)));
        backgrounds.push_back((Image::new(OASIS), Point::new(20, 28), Point::new(43, 28)));

        Self {
            time,
            date,
            backgrounds,
        }
    }
}

impl App for MainMenu<'_> {
    fn draw(&mut self, display: &mut PixelDisplay) {
        let current = self.backgrounds.front().unwrap();
        self.backgrounds
            .front()
            .unwrap()
            .0
            .draw(Point::new(0, 0), display);
        self.time.draw(current.1, display);
        self.date.draw(current.2, display);
    }

    fn input(&mut self, input: Input) {
        match input {
            Input::Next => {
                let old = self.backgrounds.pop_front().unwrap();
                self.backgrounds.push_back(old);
            }
            Input::Prev => {
                let old = self.backgrounds.pop_back().unwrap();
                self.backgrounds.push_front(old);
            }
            Input::Pressed => {}
            Input::Held => {}
        }
    }

    fn enable(&mut self) {
        ()
    }

    fn disable(&mut self) {
        ()
    }
}
