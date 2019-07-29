#[macro_use]
pub mod vga;

pub mod macros;
// pub mod interrupts;

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

pub fn init(multiboot_addr: usize) -> Result<(), &'static str> {
    // Clear the screen
    printfill!(Character::default_whitespace());

    log!("[ OK ] Begin initialization...");

    without_interrupts(|| {
        // Setup the Multiboot1 struct
        let multiboot = unsafe { multiboot2::load(multiboot_addr) };

        unsafe {
            // Enable NXE bit
            Efer::write_raw(Efer::read_raw() | 1 << 11);

            // Enable write protect bit
            Cr0::update(|flags| {
                flags.toggle(Cr0Flags::WRITE_PROTECT);
            });

            climb!(|| {
                log!("CR0 = {:#?}", Cr0::read());
                log!("EFER = {:#?}", Efer::read());
            });
        }

        // interrupts::init(&mut {

        }
    });

    log!("[ OK ] Completed initialization!");

    Ok(())
}
