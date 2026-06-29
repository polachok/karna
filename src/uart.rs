use arm_pl011_uart::{DataBits, LineConfig, Parity, StopBits, Uart, UniqueMmioPointer};
use core::ptr::NonNull;

const UART0_BASE: usize = 0x0900_0000;

pub fn init() -> Uart<'static> {
    let uart_pointer = unsafe { UniqueMmioPointer::new(NonNull::new(UART0_BASE as *mut _).unwrap()) };
    let mut uart = Uart::new(uart_pointer);
    let line_config = LineConfig {
        data_bits: DataBits::Bits8,
        parity: Parity::None,
        stop_bits: StopBits::One,
   };
   uart.enable(line_config, 115_200, 24_000_000).unwrap();
   uart
}
