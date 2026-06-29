#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    loop {
        unsafe { core::arch::asm!("wfe") }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfe") }
    }
}
