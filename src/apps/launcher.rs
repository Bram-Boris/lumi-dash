use std::collections::VecDeque;

use crate::pixel_display::pixel_display::PixelDisplay;

use super::{app::App, main_menu::MainMenu};

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
        apps.push_back(main);

        Self { apps }
    }

    pub fn draw(&self, display: &mut PixelDisplay) {
        self.apps.front().unwrap().draw(display);
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
