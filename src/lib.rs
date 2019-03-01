#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

pub mod arch;

use arch::vga::{Color, Character};

#[no_mangle]
pub extern fn kmain() -> ! {

    arch::init().unwrap();

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
