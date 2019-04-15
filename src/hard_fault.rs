use cortex_m::asm;
use cortex_m_rt::{exception, ExceptionFrame};

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    let mut b = Board::steal();
    b.leds.red.enable();
    b.leds.blue.disable();
    write!(b.cdc, "!!! Hard fault: {:?}\r\n", ef).unwrap();

    loop {
        asm::wfi();
    }
}

#[exception]
fn DefaultHandler(n: i16) {
    let mut b = unsafe { Board::steal() };
    write!(b.cdc, "in default handler: {}.\r\n", n).unwrap();
}
