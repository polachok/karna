use core::ptr::write_volatile;
use core::ptr::read_volatile;
 
const UART0_BASE: usize = 0x0900_0000;
const UART_DR: *mut u32 = UART0_BASE as *mut u32; // offset 0x00
const UART_FR: *const u32 = (UART0_BASE + 0x18) as *const u32; // offset 0x18
 
const FR_TXFF: u32 = 1 << 5; // transmit FIFO full
 
/// Write a single byte to the UART, waiting if the FIFO is full.
pub fn putb(byte: u8) {
    unsafe {
        // Spin while the transmit FIFO is full, so we don't drop bytes.
        while read_volatile(UART_FR) & FR_TXFF != 0 {}
        write_volatile(UART_DR, byte as u32);
    }
}
 
/// Write a string to the UART, translating newlines to CRLF so terminals
/// move to column zero on a new line instead of just dropping down.
pub fn puts(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            putb(b'\r');
        }
        putb(byte);
    }
}

