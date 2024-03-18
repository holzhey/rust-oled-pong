#![no_std]
#![no_main]

use core::fmt::Debug;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

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

        draw_ball(x, y, &mut display, style_off);

        x += ix;
        y += iy;
        if !(1..=126).contains(&x) {
            ix = -ix;
        }
        if !(1..=62).contains(&y) {
            iy = -iy;
        }

        draw_ball(x, y, &mut display, style_on);
        display.flush().unwrap();
    }

    fn draw_ball<D: DrawTarget<Color = BinaryColor>>(
        x: i32,
        y: i32,
        display: &mut D,
        style: PrimitiveStyle<BinaryColor>,
    ) where
        <D as embedded_graphics::draw_target::DrawTarget>::Error: Debug,
    {
        Rectangle::new(Point::new(x, y), Size::new(1, 1))
            .into_styled(style)
            .draw(display)
            .unwrap();
    }
}
