#![no_std] // don't link the Rust std lib
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    // the entry point
    loop {}
}

//Func to be called on panic situations
#[panic_handler]
fn panic(_info: &PanicInfo)-> ! {
    loop {}
}