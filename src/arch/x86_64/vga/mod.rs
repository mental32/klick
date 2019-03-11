use core::fmt;
use core::fmt::Write;
use core::sync::atomic::AtomicUsize;

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
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_at(col: usize, row: usize, args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        let _col = writer.col;
        let _row = writer.row;

        writer.col = col;
        writer.row = row;

        writer.write_fmt(args).unwrap();

        writer.col = _col;
        writer.row = _row;
    });
}

pub static LOGLEVEL: AtomicUsize = AtomicUsize::new(0);

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printat {
    ($col:expr, $row:expr, $($arg:tt)*) => ($crate::arch::vga::_print_at($col, $row, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! climb {
    ($f:expr) => ({
        use core::sync::atomic::Ordering;
        $crate::arch::vga::LOGLEVEL.fetch_add(1, Ordering::SeqCst);
        $crate::utils::wrap($f);
        $crate::arch::vga::LOGLEVEL.fetch_sub(1, Ordering::SeqCst);
    })
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => ({
        use core::sync::atomic::Ordering;
        let level = $crate::arch::vga::LOGLEVEL.load(Ordering::SeqCst);

        match level {
            0 => {print!("=>");},
            _ => {print!(" ->");},
        }

        print!(" ");

        print!("{}\n", format_args!($($arg)*));
    })
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
            writer.attr = $char.attr;
        }
    });
}
