#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub sr: SR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt Status Clear Register"]
    pub icr: ICR,
    #[doc = "0x24 - Test Register"]
    pub tr: TR,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0x34 - Version Register"]
    pub version: VERSION,
    _reserved2: [u8; 72usize],
    #[doc = "0x80 - Window configuration Register"]
    pub confw: [CONFW; 4],
    _reserved3: [u8; 64usize],
    #[doc = "0xd0 - AC Configuration Register"]
    pub conf: [CONF; 8],
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
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
#[doc = "Interrupt Status Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Clear Register"]
pub mod icr;
#[doc = "Test Register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Register"]
pub mod tr;
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
#[doc = "Window configuration Register"]
pub struct CONFW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Window configuration Register"]
pub mod confw;
#[doc = "AC Configuration Register"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AC Configuration Register"]
pub mod conf;
