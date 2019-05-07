use crate::log;

use adafruit_nrf52_bluefruit_le::Led;
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use cortex_m_rt::exception;

static LED: Mutex<RefCell<Option<Led>>> = Mutex::new(RefCell::new(None));

pub fn start(syst: &mut SYST, led: Led) {
    log!("Starting SysTick.");
    cortex_m::interrupt::free(|cs| {
        *LED.borrow(cs).borrow_mut() = Some(led);

        syst.set_clock_source(SystClkSource::Core);
        let tp10ms = match SYST::get_ticks_per_10ms() {
            0 => {
                log!("setting default tp10ms");
                // Clock varies during WFI, from what I can
                // tell. This is slightly more than a secondish.
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
}

#[exception]
fn SysTick() {
    static mut ON: bool = false;

    cortex_m::interrupt::free(|cs| {
        *ON = !*ON;
        if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
            if *ON {
                led.enable();
            } else {
                led.disable();
            }
        }
    });
}
