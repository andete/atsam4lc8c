#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Status Register"]
    pub chsr: CHSR,
    #[doc = "0x04 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x08 - Channel Disable Register"]
    pub chdr: CHDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Software Event"]
    pub sev: SEV,
    #[doc = "0x14 - Channel / User Busy"]
    pub busy: BUSY,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Trigger Interrupt Mask Enable Register"]
    pub trier: TRIER,
    #[doc = "0x24 - Trigger Interrupt Mask Disable Register"]
    pub tridr: TRIDR,
    #[doc = "0x28 - Trigger Interrupt Mask Register"]
    pub trimr: TRIMR,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Trigger Status Register"]
    pub trsr: TRSR,
    #[doc = "0x34 - Trigger Status Clear Register"]
    pub trscr: TRSCR,
    _reserved3: [u8; 8usize],
    #[doc = "0x40 - Overrun Interrupt Mask Enable Register"]
    pub ovier: OVIER,
    #[doc = "0x44 - Overrun Interrupt Mask Disable Register"]
    pub ovidr: OVIDR,
    #[doc = "0x48 - Overrun Interrupt Mask Register"]
    pub ovimr: OVIMR,
    _reserved4: [u8; 4usize],
    #[doc = "0x50 - Overrun Status Register"]
    pub ovsr: OVSR,
    #[doc = "0x54 - Overrun Status Clear Register"]
    pub ovscr: OVSCR,
    _reserved5: [u8; 168usize],
    #[doc = "0x100 - Channel Multiplexer"]
    pub chmx: [CHMX; 19],
    _reserved6: [u8; 180usize],
    #[doc = "0x200 - Event Shaper"]
    pub evs: [EVS; 31],
    _reserved7: [u8; 132usize],
    #[doc = "0x300 - Input Glitch Filter Divider Register"]
    pub igfdr: IGFDR,
    _reserved8: [u8; 244usize],
    #[doc = "0x3f8 - Parameter"]
    pub parameter: PARAMETER,
    #[doc = "0x3fc - Version"]
    pub version: VERSION,
}
#[doc = "Channel Status Register"]
pub struct CHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "Channel Enable Register"]
pub struct CHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "Channel Disable Register"]
pub struct CHDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "Software Event"]
pub struct SEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Event"]
pub mod sev;
#[doc = "Channel / User Busy"]
pub struct BUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel / User Busy"]
pub mod busy;
#[doc = "Trigger Interrupt Mask Enable Register"]
pub struct TRIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Interrupt Mask Enable Register"]
pub mod trier;
#[doc = "Trigger Interrupt Mask Disable Register"]
pub struct TRIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Interrupt Mask Disable Register"]
pub mod tridr;
#[doc = "Trigger Interrupt Mask Register"]
pub struct TRIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Interrupt Mask Register"]
pub mod trimr;
#[doc = "Trigger Status Register"]
pub struct TRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Status Register"]
pub mod trsr;
#[doc = "Trigger Status Clear Register"]
pub struct TRSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Status Clear Register"]
pub mod trscr;
#[doc = "Overrun Interrupt Mask Enable Register"]
pub struct OVIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Interrupt Mask Enable Register"]
pub mod ovier;
#[doc = "Overrun Interrupt Mask Disable Register"]
pub struct OVIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Interrupt Mask Disable Register"]
pub mod ovidr;
#[doc = "Overrun Interrupt Mask Register"]
pub struct OVIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Interrupt Mask Register"]
pub mod ovimr;
#[doc = "Overrun Status Register"]
pub struct OVSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Status Register"]
pub mod ovsr;
#[doc = "Overrun Status Clear Register"]
pub struct OVSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Status Clear Register"]
pub mod ovscr;
#[doc = "Channel Multiplexer"]
pub struct CHMX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Multiplexer"]
pub mod chmx;
#[doc = "Event Shaper"]
pub struct EVS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Shaper"]
pub mod evs;
#[doc = "Input Glitch Filter Divider Register"]
pub struct IGFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Glitch Filter Divider Register"]
pub mod igfdr;
#[doc = "Parameter"]
pub struct PARAMETER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter"]
pub mod parameter;
#[doc = "Version"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version"]
pub mod version;
