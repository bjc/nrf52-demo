use crate::cdc::CDC0;

use core::cell::RefCell;
use core::fmt::Write;
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
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut cdc) = CDC0.0.borrow(cs).borrow_mut().deref_mut() {
            write!(cdc, "Starting RTC.\r\n").unwrap();

            clock.set_lfclk_src_rc().start_lfclk();

            // Try to set clock to 8Hz.
            match rtc.set_prescaler(4_095) {
                Ok(()) => write!(cdc, "set prescaler!\r\n").unwrap(),
                Err(x) => write!(cdc, "couldn't set prescaler: {:?}\r\n", x).unwrap(),
            }
            // Make sure event is cleared before start.
            let _ = rtc.get_event_triggered(RtcInterrupt::Tick, true);

            rtc.enable_interrupt(RtcInterrupt::Tick, nvic);

            *RTC.borrow(cs).borrow_mut() = Some(rtc.enable_counter());
        }
    })
}

#[interrupt]
fn RTC1() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut rtc), Some(ref mut cdc)) = (
            RTC.borrow(cs).borrow_mut().deref_mut(),
            CDC0.0.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let _ = rtc.get_event_triggered(RtcInterrupt::Tick, true);
            write!(cdc, ".").unwrap();
        }
    });
}
