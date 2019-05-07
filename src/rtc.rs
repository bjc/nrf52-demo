use crate::{log, logstr};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use nrf52832_hal::{
    clocks::LfOscStopped,
    rtc::{RtcInterrupt, Started, Stopped},
    target::{interrupt, NVIC, RTC1},
    Clocks, Rtc,
};

static RTC: Mutex<RefCell<Option<Rtc<RTC1, Started>>>> = Mutex::new(RefCell::new(None));

pub fn start<H, L>(
    mut rtc: Rtc<RTC1, Stopped>,
    clock: Clocks<H, L, LfOscStopped>,
    nvic: &mut NVIC,
) {
    log!("Starting RTC.");
    cortex_m::interrupt::free(|cs| {
        clock.set_lfclk_src_rc().start_lfclk();

        // Try to set clock to 8Hz.
        match rtc.set_prescaler(4_095) {
            Ok(()) => log!("set prescaler!"),
            Err(x) => log!("couldn't set prescaler: {:?}", x),
        }
        // Make sure event is cleared before start.
        let _ = rtc.get_event_triggered(RtcInterrupt::Tick, true);

        rtc.enable_interrupt(RtcInterrupt::Tick, nvic);

        *RTC.borrow(cs).borrow_mut() = Some(rtc.enable_counter());
    })
}

#[interrupt]
fn RTC1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut rtc) = RTC.borrow(cs).borrow_mut().deref_mut() {
            let _ = rtc.get_event_triggered(RtcInterrupt::Tick, true);
        }
    });

    logstr!(".");
}
