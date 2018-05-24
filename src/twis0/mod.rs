#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - NBYTES Register"]
    pub nbytes: NBYTES,
    #[doc = "0x08 - Timing Register"]
    pub tr: TR,
    #[doc = "0x0c - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x10 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x14 - Packet Error Check Register"]
    pub pecr: PECR,
    #[doc = "0x18 - Status Register"]
    pub sr: SR,
    #[doc = "0x1c - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x20 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x24 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x28 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x2c - Parameter Register"]
    pub pr: PR,
    #[doc = "0x30 - Version Register"]
    pub vr: VR,
    #[doc = "0x34 - HS-mode Timing Register"]
    pub hstr: HSTR,
    #[doc = "0x38 - Slew Rate Register"]
    pub srr: SRR,
    #[doc = "0x3c - HS-mode Slew Rate Register"]
    pub hssrr: HSSRR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "NBYTES Register"]
pub struct NBYTES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NBYTES Register"]
pub mod nbytes;
#[doc = "Timing Register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Register"]
pub mod tr;
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
#[doc = "Packet Error Check Register"]
pub struct PECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet Error Check Register"]
pub mod pecr;
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
#[doc = "Status Clear Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "Parameter Register"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod pr;
#[doc = "Version Register"]
pub struct VR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod vr;
#[doc = "HS-mode Timing Register"]
pub struct HSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HS-mode Timing Register"]
pub mod hstr;
#[doc = "Slew Rate Register"]
pub struct SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slew Rate Register"]
pub mod srr;
#[doc = "HS-mode Slew Rate Register"]
pub struct HSSRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HS-mode Slew Rate Register"]
pub mod hssrr;
