#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Descriptor Base Register"]
    pub dscr: DSCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - DMA Enable Register"]
    pub dmaen: DMAEN,
    #[doc = "0x0c - DMA Disable Register"]
    pub dmadis: DMADIS,
    #[doc = "0x10 - DMA Status Register"]
    pub dmasr: DMASR,
    #[doc = "0x14 - DMA Interrupt Enable Register"]
    pub dmaier: DMAIER,
    #[doc = "0x18 - DMA Interrupt Disable Register"]
    pub dmaidr: DMAIDR,
    #[doc = "0x1c - DMA Interrupt Mask Register"]
    pub dmaimr: DMAIMR,
    #[doc = "0x20 - DMA Interrupt Status Register"]
    pub dmaisr: DMAISR,
    _reserved1: [u8; 16usize],
    #[doc = "0x34 - Control Register"]
    pub cr: CR,
    #[doc = "0x38 - Mode Register"]
    pub mr: MR,
    #[doc = "0x3c - Status Register"]
    pub sr: SR,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: ISR,
    _reserved2: [u8; 172usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Descriptor Base Register"]
pub struct DSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Base Register"]
pub mod dscr;
#[doc = "DMA Enable Register"]
pub struct DMAEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Enable Register"]
pub mod dmaen;
#[doc = "DMA Disable Register"]
pub struct DMADIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Disable Register"]
pub mod dmadis;
#[doc = "DMA Status Register"]
pub struct DMASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register"]
pub mod dmasr;
#[doc = "DMA Interrupt Enable Register"]
pub struct DMAIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Enable Register"]
pub mod dmaier;
#[doc = "DMA Interrupt Disable Register"]
pub struct DMAIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Disable Register"]
pub mod dmaidr;
#[doc = "DMA Interrupt Mask Register"]
pub struct DMAIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Mask Register"]
pub mod dmaimr;
#[doc = "DMA Interrupt Status Register"]
pub struct DMAISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Status Register"]
pub mod dmaisr;
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
