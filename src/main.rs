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

pub mod sprites;

static STYLE_ON: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_width(2)
    .stroke_color(BinaryColor::On)
    .build();

static STYLE_OFF: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_width(2)
    .stroke_color(BinaryColor::Off)
    .build();

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

    let mut ball = sprites::Ball::default();

    loop {
        arduino_hal::delay_ms(5);
        draw_ball(ball.get_x(), ball.get_y(), &mut display, STYLE_OFF);
        ball.update();
        draw_ball(ball.get_x(), ball.get_y(), &mut display, STYLE_ON);
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
