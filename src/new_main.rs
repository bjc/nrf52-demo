use adafruit_nrf52_bluefruit_le::Board;
use core::fmt::Write;
//use cortex_m::singleton;
use nrf52832_hal::target::interrupt;

struct ClosureHandler(&'static Fn() -> ());
impl ClosureHandler {
    fn call(&self) {
        self.0();
    }
}
unsafe impl Sync for ClosureHandler {}

static mut TIMER0_HANDLER: &ClosureHandler = &ClosureHandler(&|| {});

pub fn setup(mut b: Board) {
    write!(b.cdc, "New setup().\r\n").unwrap();
    //static mut X: u32 = 0;
    //let x: &'static mut u32 = singleton!(: u32 = 0).unwrap();

    // Doesn't work because FOO is static, I think? So anything inside
    // has to be constant. Lazy static may work?
    //static FOO: ClosureHandler = ClosureHandler(&move || red.enable());

    //let red: &'static mut Led = singleton!(: Led = b.leds.red).unwrap();
    //let foo = ClosureHandler(&move || red.enable());

    // static FOO: ClosureHandler = ClosureHandler(&move || unsafe { X += 1 });
    // static FOO: ClosureHandler = ClosureHandler(&move || unsafe { x += 1 });

    //unsafe {
    //TIMER0_HANDLER = &FOO;

    //TIMER0_HANDLER = singleton!(: ClosureHandler = foo).unwrap();
    //}
}

#[interrupt]
fn TIMER0() {
    unsafe {
        TIMER0_HANDLER.call();
    }
}
