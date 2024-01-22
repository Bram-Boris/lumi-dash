mod apps;
mod modules;
mod pixel_display;
use crate::apps::launcher::Input;

#[cfg(feature = "simulated")]
use embedded_graphics_simulator::sdl2::Keycode;
#[cfg(feature = "simulated")]
use embedded_graphics_simulator::SimulatorEvent;

use crate::{apps::launcher::Launcher, pixel_display::pixel_display::PixelDisplay};

fn main() -> Result<(), core::convert::Infallible> {
    use crate::{
        apps::{app::App, main_menu::MainMenu},
        pixel_display::pixel_display::{DisplayMode, DisplayOutput},
    };

    #[cfg(feature = "simulated")]
    let mut pixel_display = PixelDisplay::new(32, 64, DisplayMode::Simulated);

    #[cfg(not(feature = "simulated"))]
    let mut pixel_display = PixelDisplay::new(32, 64, DisplayMode::Real);

    let mut launcher = Launcher::new();

    'running: loop {
        pixel_display.update();
        launcher.draw(&mut pixel_display);

        match pixel_display.output {
            DisplayOutput::Real(ref mut c, ref mut m) => {
                *c = *m.update_on_vsync(Box::new(c.clone()))
            }
            DisplayOutput::Simulator(ref s, ref mut w) => {
                w.update(&s);
                #[cfg(feature = "simulated")]
                for event in w.events() {
                    match event {
                        SimulatorEvent::Quit => break 'running Ok(()),
                        SimulatorEvent::KeyDown { keycode, .. } => {
                            match keycode {
                                Keycode::Left => launcher.handle_input(Input::Next),
                                Keycode::Right => launcher.handle_input(Input::Prev),
                                Keycode::Down => launcher.handle_input(Input::Pressed),
                                Keycode::Up => launcher.handle_input(Input::Held),
                                _ => (),
                            };
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
