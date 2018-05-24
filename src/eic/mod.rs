#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x04 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x0c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x10 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x14 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x18 - Edge Register"]
    pub edge: EDGE,
    #[doc = "0x1c - Level Register"]
    pub level: LEVEL,
    #[doc = "0x20 - Filter Register"]
    pub filter: FILTER,
    _reserved0: [u8; 4usize],
    #[doc = "0x28 - Asynchronous Register"]
    pub async: ASYNC,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Enable Register"]
    pub en: EN,
    #[doc = "0x34 - Disable Register"]
    pub dis: DIS,
    #[doc = "0x38 - Control Register"]
    pub ctrl: CTRL,
    _reserved2: [u8; 960usize],
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
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
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Mode Register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mode;
#[doc = "Edge Register"]
pub struct EDGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Edge Register"]
pub mod edge;
#[doc = "Level Register"]
pub struct LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Level Register"]
pub mod level;
#[doc = "Filter Register"]
pub struct FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter Register"]
pub mod filter;
#[doc = "Asynchronous Register"]
pub struct ASYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous Register"]
pub mod async;
#[doc = "Enable Register"]
pub struct EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Register"]
pub mod en;
#[doc = "Disable Register"]
pub struct DIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Register"]
pub mod dis;
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
