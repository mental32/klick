use ::arch;

#[no_mangle]
pub extern "C" fn kmain(multiboot_addr: usize) -> ! {
    arch::init(multiboot_addr)
        .and_then(|| hlt!());
}
