#![feature(stdsimd)]
#![no_std]
#![no_main]

mod watchdog;

#[no_mangle]
pub extern "C" fn main() {
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
