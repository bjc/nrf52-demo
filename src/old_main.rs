use crate::{cdc::CDC0, rtc, tick, timer};

use adafruit_nrf52_bluefruit_le::Board;
use core::fmt::Write;
use core::ops::DerefMut;
use nrf52832_hal::{Clocks, Rtc, Temp, Timer};

pub fn setup(mut b: Board) {
    let mut cdc = b.cdc;
    write!(cdc, "Old setup()\r\n").unwrap();
    cortex_m::interrupt::free(move |cs| {
        *CDC0.0.borrow(cs).borrow_mut() = Some(cdc);
        if let Some(ref mut cdc) = CDC0.0.borrow(cs).borrow_mut().deref_mut() {
            write!(cdc, "Disabling LEDs.\r\n").unwrap();
        }
    });

    b.leds.red.disable();
    b.leds.blue.disable();

    // systick -1?
    tick::start(&mut b.SYST, b.leds.blue);

    // timer 27?
    timer::start(
        Timer::new(b.TIMER4),
        &mut b.NVIC,
        b.leds.red,
        Temp::new(b.TEMP),
    );

    rtc::start(Rtc::new(b.RTC1), Clocks::new(b.CLOCK), &mut b.NVIC);

    cortex_m::interrupt::free(move |cs| {
        if let Some(ref mut cdc) = CDC0.0.borrow(cs).borrow_mut().deref_mut() {
            write!(cdc, "Going into busy loop.\r\n").unwrap();
        }
    });
}
