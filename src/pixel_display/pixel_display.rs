use embedded_graphics::image::Image;
use embedded_graphics::mono_font::iso_8859_14::FONT_4X6;

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::*, prelude::*, text::Text, Drawable,
};

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};
use tinybmp::Bmp;

pub enum DisplayMode {
    Real,
    Simulated,
}

pub enum DisplayOutput {
    Real(Canvas, RGBMatrix),
    Simulator(SimulatorDisplay<Rgb888>, Window),
}

pub struct PixelDisplay {
    pub output: DisplayOutput,
    rows: i32,
    cols: i32,
}

impl PixelDisplay {
    pub fn draw_text(&mut self, text_str: &str, point: Point) {
        let text = Text::new(
            text_str,
            point,
            MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE),
        );

        match self.output {
            DisplayOutput::Real(ref mut c, _) => {
                text.draw(c).ok();
            }
            DisplayOutput::Simulator(ref mut s, _) => {
                text.draw(s).ok();
            }
        };
    }

    pub fn draw_image(&mut self, image: Image<'_, Bmp<'_, Rgb888>>, _point: Point) {
        match self.output {
            DisplayOutput::Real(ref mut c, _) => image.draw(c).ok(),
            DisplayOutput::Simulator(ref mut s, _) => image.draw(s).ok(),
        };
    }

    pub fn update(&mut self) {
        match self.output {
            DisplayOutput::Real(ref mut c, _) => c.fill(0, 0, 0),
            DisplayOutput::Simulator(ref mut s, _) => s.clear(Rgb888::BLACK).unwrap(),
        };
    }
}

impl PixelDisplay {
    pub fn new(rows: u32, cols: u32, display_type: DisplayMode) -> Self {
        match display_type {
            DisplayMode::Real => {
                let mut config = RGBMatrixConfig::default();
                config.rows = rows as usize;
                config.cols = cols as usize;
                let (matrix, canvas) =
                    RGBMatrix::new(config, 0).expect("Matrix initialization failed");

                PixelDisplay {
                    output: DisplayOutput::Real(*canvas, matrix),
                    rows: rows.try_into().unwrap(),
                    cols: cols.try_into().unwrap(),
                }
            }
            DisplayMode::Simulated => {
                let simulator = SimulatorDisplay::<Rgb888>::new(Size::new(cols, rows));
                let output_settings = OutputSettingsBuilder::new().scale(10).build();
                let window = Window::new("Simulator", &output_settings);

                PixelDisplay {
                    output: DisplayOutput::Simulator(simulator, window),
                    rows: rows.try_into().unwrap(),
                    cols: cols.try_into().unwrap(),
                }
            }
        }
    }
}
