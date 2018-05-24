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
    #[doc = "0x14 - Status Register"]
    pub sr: SR,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: UNLOCK,
    #[doc = "0x1c - Power Mode Control Register"]
    pub pmcon: PMCON,
    _reserved0: [u8; 8usize],
    #[doc = "0x28 - Backup Wake up Cause Register"]
    pub bkupwcause: BKUPWCAUSE,
    #[doc = "0x2c - Backup Wake up Enable Register"]
    pub bkupwen: BKUPWEN,
    #[doc = "0x30 - Backup Pin Muxing Register"]
    pub bkuppmux: BKUPPMUX,
    #[doc = "0x34 - Input Output Retention Register"]
    pub ioret: IORET,
    _reserved1: [u8; 8usize],
    #[doc = "0x40 - Bypass Register"]
    pub bpr: BPR,
    #[doc = "0x44 - Factory Word Run PS Register"]
    pub fwrunps: FWRUNPS,
    #[doc = "0x48 - Factory Word Power Save PS Register"]
    pub fwpsaveps: FWPSAVEPS,
    _reserved2: [u8; 176usize],
    #[doc = "0xfc - Version Register"]
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
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Unlock Register"]
pub struct UNLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "Power Mode Control Register"]
pub struct PMCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Mode Control Register"]
pub mod pmcon;
#[doc = "Backup Wake up Cause Register"]
pub struct BKUPWCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Wake up Cause Register"]
pub mod bkupwcause;
#[doc = "Backup Wake up Enable Register"]
pub struct BKUPWEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Wake up Enable Register"]
pub mod bkupwen;
#[doc = "Backup Pin Muxing Register"]
pub struct BKUPPMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Pin Muxing Register"]
pub mod bkuppmux;
#[doc = "Input Output Retention Register"]
pub struct IORET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Output Retention Register"]
pub mod ioret;
#[doc = "Bypass Register"]
pub struct BPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bypass Register"]
pub mod bpr;
#[doc = "Factory Word Run PS Register"]
pub struct FWRUNPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Factory Word Run PS Register"]
pub mod fwrunps;
#[doc = "Factory Word Power Save PS Register"]
pub struct FWPSAVEPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Factory Word Power Save PS Register"]
pub mod fwpsaveps;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
