#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

//Func to be called on panic situations
#[panic_handler]
fn panic(_info: &PanicInfo)-> ! {
    loop {}
}