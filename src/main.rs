#![no_std]
#![no_main]

use arduino_hal::hal::I2c;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
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
    ufmt::uwriteln!(&mut serial, "Init display interface\n").unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    ufmt::uwriteln!(&mut serial, "Init display\n").unwrap();

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    let mut screen = Screen::new(display);

    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut ix = 1;
    let mut iy = 1;

    loop {
        arduino_hal::delay_ms(10);

        screen.draw_ball(&x, &y, BinaryColor::Off);

        x += ix;
        y += iy;
        if !(1..=126).contains(&x) {
            ix = -ix;
        }
        if !(1..=62).contains(&y) {
            iy = -iy;
        }

        screen.draw_ball(&x, &y, BinaryColor::On);
        screen.flush();
    }
}

struct Screen<CLOCK> {
    display: Ssd1306<
        I2CInterface<I2c<CLOCK>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl<CLOCK> Screen<CLOCK> {
    fn new(
        display: Ssd1306<
            I2CInterface<I2c<CLOCK>>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
    ) -> Self {
        Screen { display }
    }

    fn draw_ball(&mut self, x: &i32, y: &i32, c: BinaryColor) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(2)
            .stroke_color(c)
            .build();
        Rectangle::new(Point::new(*x, *y), Size::new(1, 1))
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();
    }

    fn flush(&mut self) {
        self.display.flush().unwrap();
    }
}
