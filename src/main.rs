#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::{Baseline, Text},
};
use panic_halt as _;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Init I2C\n").unwrap();
    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Init OLED\n").unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let style_on = PrimitiveStyleBuilder::new()
        .stroke_width(2)
        .stroke_color(BinaryColor::On)
        .build();
    let style_off = PrimitiveStyleBuilder::new()
        .stroke_width(2)
        .stroke_color(BinaryColor::Off)
        .build();

    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut ix = 1;
    let mut iy = 1;

    loop {
        arduino_hal::delay_ms(10);

        Rectangle::new(Point::new(x, y), Size::new(1, 1))
            .into_styled(style_off)
            .draw(&mut display)
            .unwrap();

        x += ix;
        y += iy;
        if !(1..=126).contains(&x) {
            ix = -ix;
        }
        if !(1..=62).contains(&y) {
            iy = -iy;
        }

        Rectangle::new(Point::new(x, y), Size::new(1, 1))
            .into_styled(style_on)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
    }
}
