#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Clock Waveform Generator Register"]
    pub cwgr: CWGR,
    #[doc = "0x08 - SMBus Timing Register"]
    pub smbtr: SMBTR,
    #[doc = "0x0c - Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x10 - Next Command Register"]
    pub ncmdr: NCMDR,
    #[doc = "0x14 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x18 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x1c - Status Register"]
    pub sr: SR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x2c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x30 - Parameter Register"]
    pub pr: PR,
    #[doc = "0x34 - Version Register"]
    pub vr: VR,
    #[doc = "0x38 - HS-mode Clock Waveform Generator"]
    pub hscwgr: HSCWGR,
    #[doc = "0x3c - Slew Rate Register"]
    pub srr: SRR,
    #[doc = "0x40 - HS-mode Slew Rate Register"]
    pub hssrr: HSSRR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Clock Waveform Generator Register"]
pub struct CWGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
#[doc = "SMBus Timing Register"]
pub struct SMBTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMBus Timing Register"]
pub mod smbtr;
#[doc = "Command Register"]
pub struct CMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "Next Command Register"]
pub struct NCMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Next Command Register"]
pub mod ncmdr;
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
#[doc = "HS-mode Clock Waveform Generator"]
pub struct HSCWGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HS-mode Clock Waveform Generator"]
pub mod hscwgr;
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
