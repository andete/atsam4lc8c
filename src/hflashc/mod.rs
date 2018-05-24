#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Controller Control Register"]
    pub fcr: FCR,
    #[doc = "0x04 - Flash Controller Command Register"]
    pub fcmd: FCMD,
    #[doc = "0x08 - Flash Controller Status Register"]
    pub fsr: FSR,
    #[doc = "0x0c - Flash Controller Parameter Register"]
    pub fpr: FPR,
    #[doc = "0x10 - Flash Controller Version Register"]
    pub version: VERSION,
    #[doc = "0x14 - Flash Controller General Purpose Fuse Register High"]
    pub fgpfrhi: FGPFRHI,
    #[doc = "0x18 - Flash Controller General Purpose Fuse Register Low"]
    pub fgpfrlo: FGPFRLO,
}
#[doc = "Flash Controller Control Register"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Control Register"]
pub mod fcr;
#[doc = "Flash Controller Command Register"]
pub struct FCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Command Register"]
pub mod fcmd;
#[doc = "Flash Controller Status Register"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Status Register"]
pub mod fsr;
#[doc = "Flash Controller Parameter Register"]
pub struct FPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Parameter Register"]
pub mod fpr;
#[doc = "Flash Controller Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Version Register"]
pub mod version;
#[doc = "Flash Controller General Purpose Fuse Register High"]
pub struct FGPFRHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller General Purpose Fuse Register High"]
pub mod fgpfrhi;
#[doc = "Flash Controller General Purpose Fuse Register Low"]
pub struct FGPFRLO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller General Purpose Fuse Register Low"]
pub mod fgpfrlo;
