use embedded_graphics::geometry::Point;
use rand::Rng;
use std::time::Instant;

use std::{collections::VecDeque, time::Duration};

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

pub struct MainMenu<'a> {
    time: Time,
    date: Date,
    timer: Instant,
    backgrounds: VecDeque<(Image<'a>, Point, Point)>,
}

impl<'a> MainMenu<'a> {
    pub fn new() -> MainMenu<'a> {
        let time: Time = Time {};
        let date: Date = Date {};

        let mut backgrounds: VecDeque<(Image<'a>, Point, Point)> = VecDeque::new();
        backgrounds.push_back((Image::new(SAKURA), Point::new(2, 6), Point::new(23, 6)));
        backgrounds.push_back((Image::new(CLOUD), Point::new(44, 5), Point::new(44, 11)));
        backgrounds.push_back((Image::new(FOREST), Point::new(2, 30), Point::new(23, 30)));
        backgrounds.push_back((Image::new(NIGHT), Point::new(20, 28), Point::new(43, 28)));
        backgrounds.push_back((Image::new(ART), Point::new(20, 28), Point::new(43, 28)));

        let timer = Instant::now();

        Self {
            time,
            date,
            timer,
            backgrounds,
        }
    }

    fn randomize_background(&mut self) {
        let num = rand::thread_rng().gen_range(1..self.backgrounds.len());
        self.backgrounds.swap(0, num);
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

        if self.timer.elapsed().as_secs() > 2700 {
            self.randomize_background();
            self.timer = Instant::now();
        }
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
        self.randomize_background();
    }

    fn disable(&mut self) {
        ()
    }
}
