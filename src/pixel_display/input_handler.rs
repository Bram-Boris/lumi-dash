use crate::apps::launcher::Input;
use rppal::gpio::Gpio;
use rppal::gpio::Level;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::time::Instant;
pub struct InputHandler {}

const GPIO_PIN_CLK: u8 = 25;
const GPIO_PIN_DAT: u8 = 8;
const GPIO_PIN_SW: u8 = 7;

impl InputHandler {
    pub fn start(tx: Sender<Input>) {
        thread::spawn(move || {
            let gpio = Gpio::new().unwrap();
            gpio.get(GPIO_PIN_CLK).unwrap().into_input_pullup();
            gpio.get(GPIO_PIN_DAT).unwrap().into_input_pullup();
            gpio.get(GPIO_PIN_SW).unwrap().into_input_pullup();

            let mut last_clk_state = Level::High;
            let mut pressed = Instant::now();
            let mut last_sw_state = Level::High;

            loop {
                match gpio.get(GPIO_PIN_CLK) {
                    Ok(p) => {
                        if last_clk_state != p.read() {
                            if gpio.get(GPIO_PIN_DAT).unwrap().read() != p.read() {
                                tx.send(Input::Next).unwrap();
                            } else {
                                tx.send(Input::Prev).unwrap();
                            }
                        }
                        last_clk_state = p.read()
                    }
                    Err(_) => {}
                }

                let state = gpio.get(GPIO_PIN_SW).unwrap().read();
                if last_sw_state == Level::High && state == Level::Low {
                    pressed = Instant::now();
                } else if last_sw_state == Level::Low && state == Level::High {
                    if pressed.elapsed().as_millis() > 75 && pressed.elapsed().as_millis() < 500 {
                        tx.send(Input::Pressed).unwrap();
                    } else if pressed.elapsed().as_millis() > 1000 {
                        tx.send(Input::Held).unwrap();
                    }
                }

                last_sw_state = state;

                thread::sleep(Duration::from_millis(1));
            }
        });
    }
}
