#![no_std]
#![no_main]

mod uart;

use core::arch::global_asm;
use core::panic::PanicInfo;

use core::fmt::Write;

global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut uart = uart::init();
    let _ = uart.write_str("karna");
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
