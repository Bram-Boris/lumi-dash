mod apps;
mod modules;
mod pixel_display;

use crate::pixel_display::pixel_display::PixelDisplay;
#[cfg(feature = "simulator")]
use embedded_graphics_simulator::SimulatorEvent;

#[cfg(feature = "simulator")]
fn main() -> Result<(), core::convert::Infallible> {
    use crate::{
        apps::{app::App, main_menu::MainMenu},
        pixel_display::pixel_display::{DisplayMode, DisplayOutput},
    };

    let mut apps = Vec::<Box<dyn App>>::new();
    apps.push(Box::new(MainMenu::new()));
    let mut pixel_display = PixelDisplay::new(32, 64, DisplayMode::Simulated);

    'running: loop {
        pixel_display.update();

        apps.iter().for_each(|app| {
            app.draw(&mut pixel_display);
        });

        match pixel_display.output {
            DisplayOutput::Real(ref mut c, ref mut m) => {
                *c = *m.update_on_vsync(Box::new(c.clone()))
            }
            DisplayOutput::Simulator(ref s, ref mut w) => {
                w.update(&s);
                if w.events().any(|e| e == SimulatorEvent::Quit) {
                    break 'running Ok(());
                }
            }
        }
    }
}
