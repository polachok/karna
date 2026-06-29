#![no_std]
#![no_main]

mod uart;

use core::arch::global_asm;
use core::panic::PanicInfo;

use core::fmt::Write;

global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(dtb_addr: usize) -> ! {
    let mut uart = uart::init();
    let _ = uart.write_str("karna\n");
    let _ = writeln!(uart, "dtb @ {:#x}", dtb_addr);
    let ptr = dtb_addr as *const u8;
    writeln!(uart, "{}", unsafe { *ptr.offset(0) });
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
