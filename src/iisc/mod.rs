#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x10 - Status Set Register"]
    pub ssr: SSR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x20 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x24 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x28 - Version Register"]
    pub version: VERSION,
    #[doc = "0x2c - Parameter Register"]
    pub parameter: PARAMETER,
}
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
#[doc = "Status Clear Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "Status Set Register"]
pub struct SSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Set Register"]
pub mod ssr;
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
#[doc = "Receive Holding Register"]
pub struct RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Transmit Holding Register"]
pub struct THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
#[doc = "Parameter Register"]
pub struct PARAMETER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod parameter;
