#![no_std] // disable std lib to run bare-metal exe
#![no_main] // ignore the defult entry point chain (crt0)

use core::panic::PanicInfo;

// This function is called on panic
// std lib includes its own panic handler but is disabled,
// so we write our own
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define our own entry point
#[no_mangle] // this attribute ensures the Rust compiler outputs a function named '_start'
pub extern "C" fn _start() -> ! {
    loop{}
}
