#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub mcfg: [MCFG; 16],
    #[doc = "0x40 - Slave Configuration Register"]
    pub scfg: [SCFG; 16],
    #[doc = "0x80 - Priority Register A for Slave"]
    pub pras0: PRAS,
    #[doc = "0x84 - Priority Register B for Slave"]
    pub prbs0: PRBS,
    #[doc = "0x88 - Priority Register A for Slave"]
    pub pras1: PRAS,
    #[doc = "0x8c - Priority Register B for Slave"]
    pub prbs1: PRBS,
    #[doc = "0x90 - Priority Register A for Slave"]
    pub pras2: PRAS,
    #[doc = "0x94 - Priority Register B for Slave"]
    pub prbs2: PRBS,
    #[doc = "0x98 - Priority Register A for Slave"]
    pub pras3: PRAS,
    #[doc = "0x9c - Priority Register B for Slave"]
    pub prbs3: PRBS,
    #[doc = "0xa0 - Priority Register A for Slave"]
    pub pras4: PRAS,
    #[doc = "0xa4 - Priority Register B for Slave"]
    pub prbs4: PRBS,
    #[doc = "0xa8 - Priority Register A for Slave"]
    pub pras5: PRAS,
    #[doc = "0xac - Priority Register B for Slave"]
    pub prbs5: PRBS,
    #[doc = "0xb0 - Priority Register A for Slave"]
    pub pras6: PRAS,
    #[doc = "0xb4 - Priority Register B for Slave"]
    pub prbs6: PRBS,
    #[doc = "0xb8 - Priority Register A for Slave"]
    pub pras7: PRAS,
    #[doc = "0xbc - Priority Register B for Slave"]
    pub prbs7: PRBS,
    #[doc = "0xc0 - Priority Register A for Slave"]
    pub pras8: PRAS,
    #[doc = "0xc4 - Priority Register B for Slave"]
    pub prbs8: PRBS,
    #[doc = "0xc8 - Priority Register A for Slave"]
    pub pras9: PRAS,
    #[doc = "0xcc - Priority Register B for Slave"]
    pub prbs9: PRBS,
    #[doc = "0xd0 - Priority Register A for Slave"]
    pub pras10: PRAS,
    #[doc = "0xd4 - Priority Register B for Slave"]
    pub prbs10: PRBS,
    #[doc = "0xd8 - Priority Register A for Slave"]
    pub pras11: PRAS,
    #[doc = "0xdc - Priority Register B for Slave"]
    pub prbs11: PRBS,
    #[doc = "0xe0 - Priority Register A for Slave"]
    pub pras12: PRAS,
    #[doc = "0xe4 - Priority Register B for Slave"]
    pub prbs12: PRBS,
    #[doc = "0xe8 - Priority Register A for Slave"]
    pub pras13: PRAS,
    #[doc = "0xec - Priority Register B for Slave"]
    pub prbs13: PRBS,
    #[doc = "0xf0 - Priority Register A for Slave"]
    pub pras14: PRAS,
    #[doc = "0xf4 - Priority Register B for Slave"]
    pub prbs14: PRBS,
    #[doc = "0xf8 - Priority Register A for Slave"]
    pub pras15: PRAS,
    #[doc = "0xfc - Priority Register B for Slave"]
    pub prbs15: PRBS,
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved0: [u8; 12usize],
    #[doc = "0x110 - Special Function Register"]
    pub sfr: [SFR; 16],
}
#[doc = "Master Configuration Register"]
pub struct MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "Slave Configuration Register"]
pub struct SCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "Priority Register A for Slave"]
pub struct PRAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave"]
pub mod pras;
#[doc = "Priority Register B for Slave"]
pub struct PRBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register B for Slave"]
pub mod prbs;
#[doc = "Master Remap Control Register"]
pub struct MRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "Special Function Register"]
pub struct SFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Special Function Register"]
pub mod sfr;
