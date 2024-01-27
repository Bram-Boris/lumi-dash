use std::collections::VecDeque;

use super::{app::App, main_menu::MainMenu, spotify::Spotify};
use crate::pixel_display::pixel_display::PixelDisplay;

pub struct Launcher {
    apps: VecDeque<Box<dyn App>>,
}

pub enum Input {
    Next,
    Prev,
    Pressed,
    Held,
}

impl Launcher {
    pub fn new() -> Self {
        let mut apps = VecDeque::<Box<dyn App>>::new();
        let main: Box<MainMenu<'_>> = Box::new(MainMenu::new());
        let spotify: Box<Spotify> = Box::new(Spotify::new());

        apps.push_back(spotify);
        apps.push_back(main);

        Self { apps }
    }

    pub fn draw(&mut self, display: &mut PixelDisplay) {
        self.apps.front_mut().unwrap().draw(display);
    }

    pub fn handle_input(&mut self, input: Input) {
        match input {
            Input::Held => self.switch_app(),
            _ => (),
        }

        self.apps.front_mut().unwrap().input(input);
    }

    fn switch_app(&mut self) {
        let old = self.apps.pop_front().unwrap();
        self.apps.push_back(old);
    }
}
