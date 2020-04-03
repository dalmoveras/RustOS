#![no_std]

use core::panic::PanicInfo;
//Func to be called on panic situations
#[panic_handler]
fn panic(_info: &PanicInfo)-> ! {
    loop {}
}

fn main() {}
