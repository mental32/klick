#![feature(lang_items)]
#![no_std]

extern crate multiboot2;
use core::panic::PanicInfo;

use lazy_static::lazy_static;
use spin::Mutex;

pub mod macros;
pub mod utils;
pub mod arch;

lazy_static! {
    pub static ref KFLAGS: Mutex<u32> = { Mutex::new(0) };
}

pub mod arch;

use arch::vga::{Color, Character};

#[no_mangle]
pub extern "C" fn kmain(multiboot_addr: usize) -> ! {

    arch::init(multiboot_addr).unwrap();

    hlt!()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printfill!(Character::as_whitespace(Color::White, Color::Blue), true);
    printat!(0, 0, "{:#?}", info);
    hlt!()
}

#[lang = "eh_personality"]
#[no_mangle] 
pub extern fn eh_personality() {}
