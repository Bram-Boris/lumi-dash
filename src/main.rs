use chrono::prelude::*;

use embedded_graphics::mono_font::iso_8859_14::FONT_4X6;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    pixelcolor::Rgb888,
    prelude::*,
    text::{Alignment, Text},
    Drawable,
};

#[cfg(feature = "real")]
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};

#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

struct PixelDisplay<T>
where
    T: DrawTarget,
{
    canvas: T,
    rows: i32,
    cols: i32,
}

impl<T> PixelDisplay<T>
where
    T: DrawTarget<Color = Rgb888>,
{
    fn draw_text(&mut self, text_str: &str) {
        let text = Text::with_alignment(
            text_str,
            Point::new(self.cols / 2, self.rows / 2),
            MonoTextStyle::new(&FONT_4X6, Rgb888::RED),
            Alignment::Center,
        );

        text.draw(&mut self.canvas).ok();
    }

    fn update(&mut self) {
        self.canvas.clear(Rgb888::BLACK).ok();
        let local: DateTime<Local> = Local::now();
        self.draw_text(format!("{}", local.format("%R")).as_str());
    }
}

#[cfg(feature = "simulator")]
impl<C> PixelDisplay<SimulatorDisplay<C>>
where
    C: PixelColor + From<BinaryColor>,
{
    fn new(rows: u32, cols: u32) -> (Self, Window) {
        let simulator = SimulatorDisplay::<C>::new(Size::new(cols, rows));

        let output_settings = OutputSettingsBuilder::new().scale(10).build();

        let window = Window::new("Simulator", &output_settings);

        (
            PixelDisplay {
                canvas: simulator,
                rows: rows.try_into().unwrap(),
                cols: cols.try_into().unwrap(),
            },
            window,
        )
    }
}

#[cfg(feature = "real")]
impl PixelDisplay<Canvas>
where
    Canvas: DrawTarget,
{
    fn new(config: RGBMatrixConfig) -> (Self, RGBMatrix) {
        let rows = i32::try_from(config.rows).expect("Such large display!");
        let cols = i32::try_from(config.cols).expect("Such large display!");

        let (matrix, canvas) = RGBMatrix::new(config, 0).expect("Matrix initialization failed");

        (
            PixelDisplay {
                canvas: *canvas,
                rows,
                cols,
            },
            matrix,
        )
    }

    fn clear(&mut self) {
        self.canvas.fill(0, 0, 0);
    }
}

#[cfg(feature = "simulator")]
fn main() -> Result<(), core::convert::Infallible> {
    use embedded_graphics_simulator::SimulatorEvent;

    let (mut display, mut window) = PixelDisplay::<SimulatorDisplay<Rgb888>>::new(32, 64);

    'running: loop {
        display.update();
        window.update(&display.canvas);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
    }
}

#[cfg(feature = "real")]
fn main() -> Result<(), core::convert::Infallible> {
    let config: RGBMatrixConfig = argh::from_env();

    let (mut display, mut matrix) = PixelDisplay::<Canvas>::new(config);

    'running: loop {
        display.update();
        display.canvas = *matrix.update_on_vsync(Box::new(display.canvas));
    }
}
