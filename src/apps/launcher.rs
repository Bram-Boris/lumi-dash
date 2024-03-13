use std::collections::VecDeque;

use super::{app::App, main_menu::MainMenu, spotify::Spotify};
use crate::pixel_display::pixel_display::PixelDisplay;
use std::sync::mpsc::Sender;
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
    pub fn new(input_tx: Sender<Input>) -> Self {
        let mut apps = VecDeque::<Box<dyn App>>::new();
        let main: Box<MainMenu<'_>> = Box::new(MainMenu::new());
        let spotify: Box<Spotify> = Box::new(Spotify::new(input_tx.clone()));

        apps.push_back(main);
        apps.push_back(spotify);

        apps.front_mut().unwrap().enable();

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
        let mut old = self.apps.pop_front().unwrap();
        old.disable();
        self.apps.push_back(old);
        self.apps.front_mut().unwrap().enable();
    }
}
