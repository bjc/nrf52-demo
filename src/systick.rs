use crate::log;

use adafruit_nrf52_bluefruit_le::Led;
//use core::cell::RefCell;
//use core::ops::DerefMut;
//use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::{syst::SystClkSource, SYST};

pub fn start(syst: &mut SYST, mut led: Led) -> impl FnMut() {
    log!("Starting SysTick.");
    cortex_m::interrupt::free(|_cs| {
        syst.set_clock_source(SystClkSource::Core);
        let tp10ms = match SYST::get_ticks_per_10ms() {
            0 => {
                log!("setting default tp10ms");
                10_240
            }
            other => {
                log!("setting tp10ms to {}", other);
                other
            }
        };

        log!("tp10ms: {}", tp10ms);

        syst.set_reload(tp10ms);
        syst.clear_current();
        syst.enable_interrupt();
        syst.enable_counter();
    });
    move || systick_handler(&mut led)
}

fn systick_handler(led: &mut Led) {
    static mut ON: bool = false;
    let on = unsafe { &mut ON };

    *on = !*on;
    if *on {
        led.enable();
    } else {
        led.disable();
    }
}
