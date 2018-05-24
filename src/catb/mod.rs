#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Counter Control Register"]
    pub cntcr: CNTCR,
    #[doc = "0x08 - Sensor Idle Level"]
    pub idle: IDLE,
    #[doc = "0x0c - Sensor Relative Level"]
    pub level: LEVEL,
    #[doc = "0x10 - Sensor Raw Value"]
    pub raw: RAW,
    #[doc = "0x14 - Filter Timing Register"]
    pub timing: TIMING,
    #[doc = "0x18 - Threshold Register"]
    pub thresh: THRESH,
    #[doc = "0x1c - Pin Selection Register"]
    pub pinsel: PINSEL,
    #[doc = "0x20 - Direct Memory Access Register"]
    pub dma: DMA,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - Status Clear Register"]
    pub scr: SCR,
    _reserved0: [u8; 8usize],
    #[doc = "0x40 - In-Touch Status Register"]
    pub intch: [INTCH; 1],
    _reserved1: [u8; 12usize],
    #[doc = "0x50 - In-Touch Status Clear Register"]
    pub intchclr: [INTCHCLR; 1],
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - Out-of-Touch Status Register"]
    pub outtch: [OUTTCH; 1],
    _reserved3: [u8; 12usize],
    #[doc = "0x70 - Out-of-Touch Status Clear Register"]
    pub outtchclr: [OUTTCHCLR; 1],
    _reserved4: [u8; 132usize],
    #[doc = "0xf8 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Counter Control Register"]
pub struct CNTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Control Register"]
pub mod cntcr;
#[doc = "Sensor Idle Level"]
pub struct IDLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sensor Idle Level"]
pub mod idle;
#[doc = "Sensor Relative Level"]
pub struct LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sensor Relative Level"]
pub mod level;
#[doc = "Sensor Raw Value"]
pub struct RAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sensor Raw Value"]
pub mod raw;
#[doc = "Filter Timing Register"]
pub struct TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter Timing Register"]
pub mod timing;
#[doc = "Threshold Register"]
pub struct THRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Threshold Register"]
pub mod thresh;
#[doc = "Pin Selection Register"]
pub struct PINSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Selection Register"]
pub mod pinsel;
#[doc = "Direct Memory Access Register"]
pub struct DMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct Memory Access Register"]
pub mod dma;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
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
#[doc = "Status Clear Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "In-Touch Status Register"]
pub struct INTCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "In-Touch Status Register"]
pub mod intch;
#[doc = "In-Touch Status Clear Register"]
pub struct INTCHCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "In-Touch Status Clear Register"]
pub mod intchclr;
#[doc = "Out-of-Touch Status Register"]
pub struct OUTTCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Out-of-Touch Status Register"]
pub mod outtch;
#[doc = "Out-of-Touch Status Clear Register"]
pub struct OUTTCHCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Out-of-Touch Status Clear Register"]
pub mod outtchclr;
#[doc = "Parameter Register"]
pub struct PARAMETER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
