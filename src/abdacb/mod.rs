#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Sample Data Register 0"]
    pub sdr0: SDR0,
    #[doc = "0x08 - Sample Data Register 1"]
    pub sdr1: SDR1,
    #[doc = "0x0c - Volume Control Register 0"]
    pub vcr0: VCR0,
    #[doc = "0x10 - Volume Control Register 1"]
    pub vcr1: VCR1,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x20 - Status Register"]
    pub sr: SR,
    #[doc = "0x24 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x28 - Parameter Register"]
    pub parameter: PARAMETER,
    #[doc = "0x2c - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Sample Data Register 0"]
pub struct SDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample Data Register 0"]
pub mod sdr0;
#[doc = "Sample Data Register 1"]
pub struct SDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample Data Register 1"]
pub mod sdr1;
#[doc = "Volume Control Register 0"]
pub struct VCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Volume Control Register 0"]
pub mod vcr0;
#[doc = "Volume Control Register 1"]
pub struct VCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Volume Control Register 1"]
pub mod vcr1;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
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
