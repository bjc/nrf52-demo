use crate::{log, logstr};

use nrf52832_hal::{
    clocks::LfOscStopped,
    rtc::{RtcInterrupt, Started, Stopped},
    target::{NVIC, RTC1},
    Clocks, Rtc,
};

pub fn start<H, L>(
    mut rtc: Rtc<RTC1, Stopped>,
    clock: Clocks<H, L, LfOscStopped>,
    nvic: &mut NVIC,
) -> impl FnMut() {
    log!("Starting RTC.");
    cortex_m::interrupt::free(|_cs| {
        clock.set_lfclk_src_rc().start_lfclk();

        // Try to set clock to 8Hz.
        match rtc.set_prescaler(4_095) {
            Ok(()) => log!("set prescaler!"),
            Err(x) => log!("couldn't set prescaler: {:?}", x),
        }
        // Make sure event is cleared before start.
        let _ = rtc.get_event_triggered(RtcInterrupt::Tick, true);

        rtc.enable_interrupt(RtcInterrupt::Tick, nvic);
    });
    let mut rtc1 = rtc.enable_counter();
    move || rtc1_handler(&mut rtc1)
}

fn rtc1_handler(rtc1: &mut Rtc<RTC1, Started>) {
    let _ = rtc1.get_event_triggered(RtcInterrupt::Tick, true);
    logstr!(".");
}
