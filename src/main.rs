#![no_std]
#![no_main]

mod fdt;
mod uart;

use core::arch::global_asm;
use core::panic::PanicInfo;

use core::fmt::Write;

global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(dtb_addr: *const u8) -> ! {
    let mut uart = uart::init();
    let _ = uart.write_str("karna\n");

    let hw = unsafe { fdt::init(dtb_addr) };
    let _ = writeln!(uart, "hw model {}", hw.model());
    for cpu in hw.cpus() {
        let _ = writeln!(uart, " cpu {}: {}", cpu.id, cpu.compatible);
    }
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
