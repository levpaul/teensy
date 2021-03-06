#![feature(stdsimd)]
#![no_std]
#![no_main]
mod port;
mod sim;
mod watchdog;

#[no_mangle]
pub extern "C" fn main() {
    let (wdog, sim, pin) = unsafe {
        (
            watchdog::Watchdog::new(),
            sim::Sim::new(),
            port::Port::new(port::PortName::C).pin(13),
        )
    };

    wdog.disable();
    sim.enable_clock(sim::Clock::PortC);

    let mut gpio = pin.make_gpio();

    gpio.output();
    gpio.high();

    loop {}
}

#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF,
];
