#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    _reserved1: [u8; 16usize],
    #[doc = "0x20 - Maintenance Register 0"]
    pub maint0: MAINT0,
    #[doc = "0x24 - Maintenance Register 1"]
    pub maint1: MAINT1,
    #[doc = "0x28 - Monitor Configuration Register"]
    pub mcfg: MCFG,
    #[doc = "0x2c - Monitor Enable Register"]
    pub men: MEN,
    #[doc = "0x30 - Monitor Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x34 - Monitor Status Register"]
    pub msr: MSR,
    _reserved2: [u8; 196usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
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
#[doc = "Maintenance Register 0"]
pub struct MAINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maintenance Register 0"]
pub mod maint0;
#[doc = "Maintenance Register 1"]
pub struct MAINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maintenance Register 1"]
pub mod maint1;
#[doc = "Monitor Configuration Register"]
pub struct MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor Configuration Register"]
pub mod mcfg;
#[doc = "Monitor Enable Register"]
pub struct MEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor Enable Register"]
pub mod men;
#[doc = "Monitor Control Register"]
pub struct MCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor Control Register"]
pub mod mctrl;
#[doc = "Monitor Status Register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor Status Register"]
pub mod msr;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
