use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use nrf52832_hal::{target::UARTE0, Uarte};

static CDC0: Mutex<RefCell<Option<Uarte<UARTE0>>>> = Mutex::new(RefCell::new(None));

pub fn setup(cdc: Uarte<UARTE0>) {
    cortex_m::interrupt::free(|cs| {
        *CDC0.borrow(cs).borrow_mut() = Some(cdc);
    });
}

pub fn write_fmt(args: core::fmt::Arguments, nl: bool) {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut cdc) = CDC0.borrow(cs).borrow_mut().deref_mut() {
            core::fmt::write(cdc, args).unwrap();
            if nl {
                cdc.write_str("\r\n").unwrap();
            }
        }
    });
}
