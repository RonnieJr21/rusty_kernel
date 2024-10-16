#![no_std]
#![no_main]

mod vga_buffer;
static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}