use core::fmt;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;
use spin::Mutex;

mod attribute;

pub use attribute::{Attribute, Color};

mod writer;

use writer::Writer;

pub use writer::Character;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = { Mutex::new(Writer::new()) };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printat {
    ($($arg:tt)*) => ($crate::arch::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printfill {
    ($char:expr) => ({
        $crate::arch::vga::WRITER.lock().fill($char)
    });

    ($char:expr, $persist:expr) => ({
        let mut writer = $crate::arch::vga::WRITER.lock();
        writer.fill($char);

        if $persist {
            writer.set_attribute($char.attr);
        }
    });
}
