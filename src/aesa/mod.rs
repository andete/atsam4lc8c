#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x08 - Data Buffer Pointer Register"]
    pub databufptr: DATABUFPTR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Key Register"]
    pub key: [KEY; 8],
    #[doc = "0x40 - Initialization Vector Register"]
    pub initvect: [INITVECT; 4],
    #[doc = "0x50 - Input Data Register"]
    pub idata: IDATA,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - Output Data Register"]
    pub odata: ODATA,
    _reserved2: [u8; 12usize],
    #[doc = "0x70 - DRNG Seed Register"]
    pub drngseed: DRNGSEED,
    _reserved3: [u8; 132usize],
    #[doc = "0xf8 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Mode Register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mode;
#[doc = "Data Buffer Pointer Register"]
pub struct DATABUFPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Buffer Pointer Register"]
pub mod databufptr;
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
#[doc = "Key Register"]
pub struct KEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Register"]
pub mod key;
#[doc = "Initialization Vector Register"]
pub struct INITVECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initialization Vector Register"]
pub mod initvect;
#[doc = "Input Data Register"]
pub struct IDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Data Register"]
pub mod idata;
#[doc = "Output Data Register"]
pub struct ODATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data Register"]
pub mod odata;
#[doc = "DRNG Seed Register"]
pub struct DRNGSEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRNG Seed Register"]
pub mod drngseed;
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
