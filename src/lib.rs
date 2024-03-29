#![no_std]

use cortex_m::asm::delay;
use embedded_hal::digital::v2::OutputPin;
use smart_leds_trait::{SmartLedsWrite, RGB8};

pub struct Ws2812<P: OutputPin> {
    pin: P,
}

impl<P: OutputPin> Ws2812<P> {
    pub fn new(pin: P) -> Ws2812<P> {
        Ws2812 { pin: pin }
    }
    // These timings are experimantally confirmed
    fn write_byte(&mut self, data: u8) {
        let mut bitmask: u8 = 0x80;
        while bitmask != 0 {
            self.pin.set_high().ok();
            if data & bitmask != 0 {
                delay(10);
                self.pin.set_low().ok();
            } else {
                self.pin.set_low().ok();
                delay(8);
            }
            bitmask >>= 1;
            delay(1);
        }
    }
}

impl<P> SmartLedsWrite for Ws2812<P>
where
    P: OutputPin,
{
    type Color = RGB8;
    type Error = ();
    fn write<T, I>(&mut self, iterator: T) -> Result<(), ()>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        for item in iterator {
            let color: Self::Color = item.into();
            self.write_byte(color.g);
            self.write_byte(color.r);
            self.write_byte(color.b);
        }
        Ok(())
    }
}
