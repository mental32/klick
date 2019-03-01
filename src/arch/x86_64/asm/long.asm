extern kmain

global __start

section .text

bits 64

__start:

    ; Empty all segment registers
    ; Most of them are ignored but the three that aren't shouldn't contain garbage
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Call into Rust
    call kmain

.spin:
	hlt
	jmp .spin
