#![no_std]
#![no_main]

mod log;
mod macros;
mod rtc;
mod systick;
mod timer;

#[allow(unused_imports)]
use panic_abort;

use adafruit_nrf52_bluefruit_le::Board;
use clint::HandlerArray;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use nrf52832_hal::{target::interrupt, Clocks, Rtc, Temp, Timer};

static HANDLERS: HandlerArray = HandlerArray::new();

#[entry]
fn main() -> ! {
    let mut b = Board::take().unwrap();

    log::setup(b.cdc);

    log!("Disabling LEDs.");
    b.leds.red.disable();
    b.leds.blue.disable();

    let mut systick_handler = systick::start(&mut b.SYST, b.leds.blue);

    let mut rtc_handler = rtc::start(Rtc::new(b.RTC1), Clocks::new(b.CLOCK), &mut b.NVIC);

    let mut timer_handler = timer::start(
        Timer::new(b.TIMER4),
        &mut b.NVIC,
        b.leds.red,
        Temp::new(b.TEMP),
    );

    HANDLERS.with_overrides(|hs| {
        hs.register(0, &mut systick_handler);
        hs.register(1, &mut rtc_handler);
        hs.register(2, &mut timer_handler);

        log!("Going into busy loop.");
        loop {
            // SysTick runs very, very slowly during wfi, to enable low
            // power states. It will fire, but it's not frequent.
            asm::wfi();
        }
    });

    unreachable!();
}

#[exception]
fn SysTick() {
    HANDLERS.call(0);
}

#[interrupt]
fn RTC1() {
    HANDLERS.call(1);
}

#[interrupt]
fn TIMER4() {
    HANDLERS.call(2);
}
