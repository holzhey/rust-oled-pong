#![no_std]
#![no_main]

use arduino_hal::I2c;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use panic_halt as _;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

#[arduino_hal::entry]
fn main() -> ! {
    let mut screen = Screen::new();
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

    struct Screen {
        display:
            Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    }

    impl Screen {
        fn new() -> Self {
            let dp = arduino_hal::Peripherals::take().unwrap();
            let pins = arduino_hal::pins!(dp);
            let i2c = arduino_hal::I2c::new(
                dp.TWI,
                pins.a4.into_pull_up_input(),
                pins.a5.into_pull_up_input(),
                50000,
            );
            let interface = I2CDisplayInterface::new(i2c);
            let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode();
            display.init().unwrap();
            display.set_pixel(1, 1, true);
            display.flush().unwrap();
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
}
