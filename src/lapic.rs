//! Local APIC
//!
//! Migrate from: [xv6](https://github.com/mit-pdos/xv6-public/blob/master/lapic.c)

use x86_64::instructions::port::Port;
use core::ptr::{read_volatile, write_volatile};

/// Start additional processor running entry code at `addr`.
/// See Appendix B of MultiProcessor Specification.
///
/// The entry point `addr` must be 4K aligned.
/// This function will access memory: 0x467, 0xfee00000.
pub unsafe fn start_ap(apic_id: u8, addr: u16) {
    const CMOS_PORT: u16 = 0x70;
    const CMOS_RETURN: u16 = 0x71;
    const ICRLO: usize = 0x0300 / 4;    // Interrupt Command
    const ICRHI: usize = 0x0310 / 4;    // Interrupt Command [63:32]
    const INIT: u32 = 0x00000500;       // INIT/RESET
    const STARTUP: u32 = 0x00000600;    // Startup IPI
    const DELIVS: u32 = 0x00001000;     // Delivery status
    const ASSERT: u32 = 0x00004000;     // Assert interrupt (vs deassert)
    const DEASSERT: u32 = 0x00000000;
    const LEVEL: u32 = 0x00008000;      // Level triggered
    const BCAST: u32 = 0x00080000;      // Send to all APICs, including self.
    const BUSY: u32 = 0x00001000;
    const FIXED: u32 = 0x00000000;

    // "The BSP must initialize CMOS shutdown code to 0AH
    // and the warm reset vector (DWORD based at 40:67) to point at
    // the AP startup code prior to the [universal startup algorithm]."
    Port::new(CMOS_PORT).write(0xFu16); // offset 0xF is shutdown code
    Port::new(CMOS_RETURN).write(0xAu16);

    let wrv = (0x40 << 4 | 0x67) as *mut u16;  // Warm reset vector
    *wrv = 0;
    *wrv.offset(1) = addr >> 4;

    // "Universal startup algorithm."
    // Send INIT (level-triggered) interrupt to reset other CPU.
    write(ICRHI, (apic_id as u32) << 24);
    write(ICRLO, INIT | LEVEL | ASSERT);
    microdelay(200);
    write(ICRLO, INIT | LEVEL);
    microdelay(10000);

    // Send startup IPI (twice!) to enter code.
    // Regular hardware is supposed to only accept a STARTUP
    // when it is in the halted state due to an INIT.  So the second
    // should be ignored, but it is part of the official Intel algorithm.
    for _ in 0..2 {
        write(ICRHI, (apic_id as u32) << 24);
        write(ICRLO, STARTUP | (addr >> 12) as u32);
        microdelay(200);
    }
}

unsafe fn write(index: usize, value: u32) {
    const BASE: usize = 0xfee00000;
    const ID: *const u32 = (BASE + 0x20) as *const u32;

    write_volatile((BASE as *mut u32).offset(index as isize), value);
    read_volatile(ID); // wait for write to finish, by reading
}

fn microdelay(_ms: usize) {

}