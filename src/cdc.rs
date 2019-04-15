use core::cell::RefCell;
use core::fmt;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use nrf52832_hal::{target::UARTE0, Uarte};

pub struct CDC(pub Mutex<RefCell<Option<Uarte<UARTE0>>>>);

impl fmt::Write for CDC {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        cortex_m::interrupt::free(move |cs| {
            if let Some(ref mut cdc) = *self.0.borrow(cs).borrow_mut().deref_mut() {
                cdc.write_str(s)
            } else {
                Err(fmt::Error)
            }
        })
    }
}

pub static CDC0: CDC = CDC(Mutex::new(RefCell::new(None)));
