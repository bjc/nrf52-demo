#![no_std]
#![no_main]
#![feature(trait_alias)]

#[allow(unused_imports)]
use panic_abort;

use adafruit_nrf52_bluefruit_le::Board;
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    //nrf52_demo::new_main::setup(b);
    nrf52_demo::old_main::setup(Board::take().unwrap());

    loop {
        // SysTick runs very, very slowly during wfi, to enable low
        // power states. It will fire, but it's not frequent.
        asm::wfi();
    }

    // TODO: may be able to get rid of static lifetime stuff by adding
    // an equivalent to thread::join here, after the non-terminating
    // loop. This should be cleaner, since it would allow you to do
    // clean up. But it would also require putting the original
    // handlers back on drop.
}
