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
use cortex_m::asm;
use cortex_m_rt::entry;
use nrf52832_hal::{Clocks, Rtc, Temp, Timer};

#[entry]
fn main() -> ! {
    let mut b = Board::take().unwrap();

    log::setup(b.cdc);

    log!("Disabling LEDs.");
    b.leds.red.disable();
    b.leds.blue.disable();

    // systick -1?
    systick::start(&mut b.SYST, b.leds.blue);

    // timer 27?
    timer::start(
        Timer::new(b.TIMER4),
        &mut b.NVIC,
        b.leds.red,
        Temp::new(b.TEMP),
    );

    rtc::start(Rtc::new(b.RTC1), Clocks::new(b.CLOCK), &mut b.NVIC);

    log!("Going into busy loop.");
    loop {
        // SysTick runs very, very slowly during wfi, to enable low
        // power states. It will fire, but it's not frequent.
        asm::wfi();
    }

    // TODO: may be able to get rid of static lifetime stuff by adding
    // an equivalent to thread::join here, after the non-terminating
    // loop. This should be cleaner, since it would allow you to do
    // clean up. But it would also require putting the original
    // handlers back on drop.
}
