#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Clear Register"]
    pub clr: CLR,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x10 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x14 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x18 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x1c - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved0: [u8; 988usize],
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Clear Register"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Register"]
pub mod clr;
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
