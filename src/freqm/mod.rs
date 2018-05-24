#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Mode register"]
    pub mode: MODE,
    #[doc = "0x08 - Status register"]
    pub status: STATUS,
    #[doc = "0x0c - Value register"]
    pub value: VALUE,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Diable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved0: [u8; 984usize],
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Mode register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode register"]
pub mod mode;
#[doc = "Status register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod status;
#[doc = "Value register"]
pub struct VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value register"]
pub mod value;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Diable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Diable Register"]
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
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
