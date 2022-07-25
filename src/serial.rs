use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt;

const SERIAL_IO_PORT_COM1: u16 = 0x3F8;

lazy_static! {
	pub static ref SERIAL1: Mutex<SerialPort> = {
		let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT_COM1) };
		serial_port.init();
		Mutex::new(serial_port)
	};
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
