# klick

A minimalist Unix-like kernel, written in Rust!

Klick is a Unix-like microkernel imitating Linux design and following in the footsteps of redox and minix.

The kernel inherits, by design, the stability and security benefits of a microkernel design, 
also it takes both a Unix "everything is a file" and a Redox "everything is a URI" approach.

## Compiling

### Requirements

The following are required to compile and run the kernel

 - GRUB
 - Xorriso
 - Qemu/Bochs
 - Xargo
 - NASM

### Make

The Makefile should handle most if not all of the work needed to compile.

 - `make` Will compile the kernel and place it in `build/` as `klick-(ARCH).bin`
 - `make iso` Will use grub-mkrescue to construct an iso, it's placed in `build/` as `klick-(ARCH).iso`
 - `make run` Will use Qemu to run the iso file
 - `make clean` Will remove `build/` and `target/` (produced by cargo)

## Progress

> Until every item is checked I strongly suggest against using the kernel.
>
> I'll be probably be archived for educational purposes once "complete" anyway.


### Kernel

#### Meta

 - [ ] \(C\)Make system (or any build system)
 - [ ] Documentation
 - [ ] Book

#### Compatability

 - [ ] target ARM systems
 - [ ] target x86 systems
 - [ ] target x86_64 systems

#### Bootstrap

 - [ ] Setup stack
 - [ ] Check Multiboot & store Multiboot information structure
 - [ ] Check CPUID
 - [ ] Enable Paging
 - [ ] Enable Long mode
 - [ ] Load GDT (static entries)
 - [ ] Load IDT, map basic exception handlers

#### Runtime

 - [ ] System allocator
 - [ ] Memory manager
 - [ ] Multitasking (pre-emptive)
 - [ ] Multitasking (co-operative)
 - [ ] Virtual Filesystem

#### Drivers

 - [ ] VGA Mode
 - [ ] VESA Mode
 - [ ] FAT 16/32
 - [ ] EXT 2/3/4
