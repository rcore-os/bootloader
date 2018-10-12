# xv6 x86_64 entryother.S

# Each non-boot CPU ("AP") is started up in response to a STARTUP
# IPI from the boot CPU.  Section B.4.2 of the Multi-Processor
# Specification says that the AP will start in real mode with CS:IP
# set to XY00:0000, where XY is an 8-bit value sent with the
# STARTUP. Thus this code must start at a 4096-byte boundary.
#
# Because this code sets DS to zero, it must sit
# at an address in the low 2^16 bytes.

.section .boot_ap, "awx"
.intel_syntax noprefix
.code16
.global ap_start

ap_start:
    cli

    xor     ax, ax
    mov     ds, ax
    mov     es, ax
    mov     ss, ax

    # set protected mode bit
    mov     eax, cr0
    or      al, 1
    mov     cr0, eax

    # set stack pointer
    mov     esp, 0x7000

    call    enable_paging

    # load the 64-bit GDT
    lgdt    [gdt_64_pointer]

    # jump to long mode
    push 0x0
    push 0x8
    lea eax, [ap_start64]
    push eax
    retf

.code64
ap_start64:
    # load 0 into all data segment registers
    xor     ax, ax
    mov     ss, ax
    mov     ds, ax
    mov     es, ax
    mov     fs, ax
    mov     gs, ax

    call    other_main
