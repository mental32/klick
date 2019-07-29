#![feature(lang_items)]
#![no_std]

extern crate multiboot2;

use core::panic::PanicInfo;

pub mod macros;
pub mod utils;
pub mod arch;
pub mod sys;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use arch::vga::{
        Color,
        Character
    };

    printfill!(Character::whitespace(Color::White, Color::Blue), true);
    printat!(0, 0, "{:#?}", info);
    hlt!()
}

#[lang = "eh_personality"]
#[no_mangle] 
pub extern fn eh_personality() {}
