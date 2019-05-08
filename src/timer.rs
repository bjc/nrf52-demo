use crate::log;

use adafruit_nrf52_bluefruit_le::{prelude::*, Led};
use embedded_hal::timer::Cancel;
use nrf52832_hal::{
    target::{NVIC, TIMER4},
    Temp, Timer,
};

pub fn start(
    mut timer: Timer<TIMER4>,
    mut nvic: &mut NVIC,
    mut led: Led,
    mut temp: Temp,
) -> impl FnMut() {
    log!("Starting TIMER4.");
    cortex_m::interrupt::free(|_cs| {
        timer.start(1_000_000 as u32); // Timer is set to 1MHz in nrf52's hal, so this is 1 second.
        timer.enable_interrupt(&mut nvic);
    });
    move || timer_handler(&mut led, &mut timer, &mut temp)
}

fn log_temp(temp: &mut Temp) {
    log!("Temp is {}°C", temp.measure())
}

fn toggle_red(led: &mut Led) {
    static mut ON: bool = false;
    let on = unsafe { &mut ON };

    *on = !*on;
    if *on {
        led.enable();
    } else {
        led.disable();
    }
}

fn timer_handler(led: &mut Led, timer: &mut Timer<TIMER4>, temp: &mut Temp) {
    log_temp(temp);
    toggle_red(led);

    // TODO: shouldn't be bit twiddling here, but the HAL doesn't have
    // a way of clearing the comparator.
    //b.TIMER4.events_compare[0].write(|w| w);

    // A side effect of cancel is to clear the pending event. There
    // should be another API for this.
    timer.cancel().unwrap();
    timer.start(1_000_000 as u32);
}
