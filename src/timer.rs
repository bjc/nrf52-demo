use crate::cdc::CDC0;

use adafruit_nrf52_bluefruit_le::{prelude::*, Led};
use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use embedded_hal::timer::Cancel;
use nrf52832_hal::{
    target::{interrupt, NVIC, TIMER4},
    uarte::UarteExt,
    Temp, Timer, Uarte,
};

static LED: Mutex<RefCell<Option<Led>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<Timer<TIMER4>>>> = Mutex::new(RefCell::new(None));
static TEMP: Mutex<RefCell<Option<Temp>>> = Mutex::new(RefCell::new(None));

pub fn start(timer: Timer<TIMER4>, mut nvic: &mut NVIC, led: Led, temp: Temp) {
    cortex_m::interrupt::free(move |cs| {
        if let Some(ref mut cdc) = CDC0.0.borrow(cs).borrow_mut().deref_mut() {
            write!(cdc, "Starting TIMER4.\r\n").unwrap();
        }
        *LED.borrow(cs).borrow_mut() = Some(led);
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
        *TEMP.borrow(cs).borrow_mut() = Some(temp);

        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            timer.start(1_000_000 as u32); // Timer is set to 1MHz in nrf52's hal, so this is 1 second.
            timer.enable_interrupt(&mut nvic);
        }
    });
}

fn log_temp<T>(_temp: &mut Temp, _cdc: &mut Uarte<T>)
where
    T: UarteExt,
{
}

fn toggle_red<T>(led: &mut Led, _cdc: &mut Uarte<T>)
where
    T: UarteExt,
{
    static mut ON: bool = false;
    let on = unsafe { &mut ON };

    *on = !*on;
    if *on {
        led.enable();
    } else {
        led.disable();
    }
}

#[interrupt]
fn TIMER4() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut led), Some(ref mut cdc), Some(ref mut temp)) = (
            LED.borrow(cs).borrow_mut().deref_mut(),
            CDC0.0.borrow(cs).borrow_mut().deref_mut(),
            TEMP.borrow(cs).borrow_mut().deref_mut(),
        ) {
            log_temp(temp, cdc);
            toggle_red(led, cdc);
        }

        // TODO: shouldn't be bit twiddling here, but the HAL doesn't have
        // a way of clearing the comparator.
        //b.TIMER4.events_compare[0].write(|w| w);

        // A side effect of cancel is to clear the pending event. There
        // should be another API for this.
        if let Some(ref mut t) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            t.cancel().unwrap();
            t.start(1_000_000 as u32);
        }
    });
}
