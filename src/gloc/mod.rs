#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr0: CR,
    #[doc = "0x04 - Truth Register"]
    pub truth0: TRUTH,
    #[doc = "0x08 - Control Register"]
    pub cr1: CR,
    #[doc = "0x0c - Truth Register"]
    pub truth1: TRUTH,
    _reserved0: [u8; 40usize],
    #[doc = "0x38 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0x3c - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Truth Register"]
pub struct TRUTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Truth Register"]
pub mod truth;
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
