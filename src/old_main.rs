use crate::{cdc::CDC0, rtc, tick, timer};

use adafruit_nrf52_bluefruit_le::{prelude::*, Board};
use core::fmt::Write;
use core::ops::DerefMut;
use nrf52832_hal::Temp;

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
        b.TIMER4.constrain(),
        &mut b.NVIC,
        b.leds.red,
        Temp::new(b.TEMP),
    );

    rtc::start(b.RTC1.constrain(), b.CLOCK.constrain(), &mut b.NVIC);

    cortex_m::interrupt::free(move |cs| {
        if let Some(ref mut cdc) = CDC0.0.borrow(cs).borrow_mut().deref_mut() {
            write!(cdc, "Going into busy loop.\r\n").unwrap();
        }
    });
}
