#[macro_use]
pub mod vga;

pub mod macros;
pub mod interrupts;

use x86_64::{
    registers::{
        model_specific::Efer,
        control::{
            Cr0,
            Cr0Flags
        }
    },

    instructions::interrupts::without_interrupts
};

use vga::Character;

use crate::kflagset;

pub fn init(multiboot_addr: usize) -> Result<(), &'static str> {
    kflagset!("Attempted to initialize arch::x86_64 twice!");

    // Clear the screen
    printfill!(Character::as_default_whitespace());

    println!("[ OK ] Begin initialization...");

    without_interrupts(|| {
        // Setup the Multiboot1 struct
        let multiboot = unsafe { multiboot2::load(multiboot_addr) };

        unsafe {
            // Enable NXE bit
            Efer::write_raw(Efer::read_raw() | 1 << 11);

            // Enable write protect bit
            Cr0::update(|flags| {
                flags.toggle(Cr0Flags::WRITE_PROTECT);
            })
        }
    });

    println!("[ OK ] Completed initialization!");

    Ok(())
}
