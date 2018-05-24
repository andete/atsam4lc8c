#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Status Register"]
    pub sr: SR,
    #[doc = "0x08 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x0c - Address Register"]
    pub addr: ADDR,
    #[doc = "0x10 - Length Register"]
    pub length: LENGTH,
    #[doc = "0x14 - Data Register"]
    pub data: DATA,
    _reserved0: [u8; 16usize],
    #[doc = "0x28 - VERSION register"]
    pub version: VERSION,
    _reserved1: [u8; 196usize],
    #[doc = "0xf0 - Chip ID Register"]
    pub cidr: CIDR,
    #[doc = "0xf4 - Chip ID Extension Register"]
    pub exid: EXID,
    _reserved2: [u8; 4usize],
    #[doc = "0xfc - AP Identification register"]
    pub idr: IDR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
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
#[doc = "Address Register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address Register"]
pub mod addr;
#[doc = "Length Register"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length Register"]
pub mod length;
#[doc = "Data Register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register"]
pub mod data;
#[doc = "VERSION register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VERSION register"]
pub mod version;
#[doc = "Chip ID Register"]
pub struct CIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "Chip ID Extension Register"]
pub struct EXID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Extension Register"]
pub mod exid;
#[doc = "AP Identification register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AP Identification register"]
pub mod idr;
