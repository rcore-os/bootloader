//! Provides boot information to the kernel.

#![deny(improper_ctypes)]

pub use self::memory_map::*;

mod memory_map;

/// This structure represents the information that the bootloader passes to the kernel.
///
/// The information is passed as an argument to the entry point:
///
/// ```ignore
/// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
///    // […]
/// }
/// ```
///
/// Note that no type checking occurs for the entry point function, so be careful to
/// use the correct argument types. To ensure that the entry point function has the correct
/// signature, use the [`entry_point`] macro.
#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    /// A map of the physical memory regions of the underlying machine.
    ///
    /// The bootloader queries this information from the BIOS/UEFI firmware and translates this
    /// information to Rust types. It also marks any memory regions that the bootloader uses in
    /// the memory map before passing it to the kernel. Regions marked as usable can be freely
    /// used by the kernel.
    pub memory_map: MemoryMap,
    /// The virtual address of the recursively mapped level 4 page table.
    #[cfg(feature = "recursive_page_table")]
    pub recursive_page_table_addr: u64,
    /// The offset into the virtual address space where the physical memory is mapped.
    ///
    /// Physical addresses can be converted to virtual addresses by adding this offset to them.
    ///
    /// The mapping of the physical memory allows to access arbitrary physical frames. Accessing
    /// frames that are also mapped at other virtual addresses can easily break memory safety and
    /// cause undefined behavior. Only frames reported as `USABLE` by the memory map in the `BootInfo`
    /// can be safely accessed.
    #[cfg(feature = "map_physical_memory")]
    pub physical_memory_offset: u64,
    /// The VBE mode information
    pub vbe_info: VbeModeInfo,
}

/// The VBE mode information
/// https://wiki.osdev.org/User:Omarrx024/VESA_Tutorial
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VbeModeInfo {
    _1: [u8; 6],
    window_size: u16,
    segment_a: u16,
    segment_b: u16,
    _2: u32,
    pitch: u16,
    width: u16,
    height: u16,
    _3: [u8; 3],
    bpp: u8,
    _4: [u8; 14],
    framebuffer: u32,
}

extern "C" {
    fn _improper_ctypes_check(_boot_info: BootInfo);
}
