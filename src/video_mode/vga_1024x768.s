.section .boot, "awx"
.intel_syntax noprefix
.code16

config_video_mode:
    mov bx, 0x4119	# VBE mode number mode(0x4118): 1024x768, 24bpp ; notice that bits 0-13 contain the mode number and bit 14 (LFB) is set and bit 15 (DM) is clear.

try_mode:
    sub bx, 1
    mov ax, 0x4F02	# set VBE mode
    int 0x10		# call VBE BIOS
    cmp ax, 0x004F  # success
    jne try_mode

    mov ax, 0x4F01  # get mode info
    mov cx, bx      
    lea di, es:[_vbe_info] # output ModeInfoBlock at es:di
    int 0x10        # call VBE BIOS

    ret

.code32

vga_map_frame_buffer:
    mov eax, 0xa0000
    or eax, (1 | 2)
vga_map_frame_buffer_loop:
    mov ecx, eax
    shr ecx, 12
    mov [_p1 + ecx * 8], eax

    add eax, 4096
    cmp eax, 0xa0000 + 320 * 200
    jl vga_map_frame_buffer_loop

    ret

# print a string and a newline
# IN
#   esi: points at zero-terminated String
vga_println:
    ret

# print a string
vga_print:
    ret
