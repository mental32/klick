pub mod macros;

pub mod interrupts;

#[macro_use]
pub mod vga;

use vga::Character;

pub fn init() -> Result<(), &'static str> {

    // Clear the screen
    printfill!(Character::as_default_whitespace());

    // Reload GDT

    // Load IDT

    // Load allocator

    // Initialize hardware

    Ok(())
}
