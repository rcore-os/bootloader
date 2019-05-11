#[cfg(not(any(feature = "vga_320x200", feature = "vga_1024x768")))]
pub use self::vga_text_80x25::*;

#[cfg(any(feature = "vga_320x200", feature = "vga_1024x768"))]
pub use self::vga_320x200::*;

mod vga_320x200;
mod vga_text_80x25;
