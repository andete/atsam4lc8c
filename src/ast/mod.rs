#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Counter Value"]
    pub cv: CV,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Wake Enable Register"]
    pub wer: WER,
    #[doc = "0x20 - Alarm Register 0"]
    pub ar0: AR0,
    #[doc = "0x24 - Alarm Register 1"]
    pub ar1: AR1,
    _reserved0: [u8; 8usize],
    #[doc = "0x30 - Periodic Interval Register 0"]
    pub pir0: PIR0,
    #[doc = "0x34 - Periodic Interval Register 1"]
    pub pir1: PIR1,
    _reserved1: [u8; 8usize],
    #[doc = "0x40 - Clock Control Register"]
    pub clock: CLOCK,
    #[doc = "0x44 - Digital Tuner Register"]
    pub dtr: DTR,
    #[doc = "0x48 - Event Enable Register"]
    pub eve: EVE,
    #[doc = "0x4c - Event Disable Register"]
    pub evd: EVD,
    #[doc = "0x50 - Event Mask Register"]
    pub evm: EVM,
    #[doc = "0x54 - Calendar Value"]
    pub calv: CALV,
    _reserved2: [u8; 152usize],
    #[doc = "0xf0 - Parameter Register"]
    pub parameter: PARAMETER,
    _reserved3: [u8; 8usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Counter Value"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value"]
pub mod cv;
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
#[doc = "Wake Enable Register"]
pub struct WER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Enable Register"]
pub mod wer;
#[doc = "Alarm Register 0"]
pub struct AR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Register 0"]
pub mod ar0;
#[doc = "Alarm Register 1"]
pub struct AR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Register 1"]
pub mod ar1;
#[doc = "Periodic Interval Register 0"]
pub struct PIR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Periodic Interval Register 0"]
pub mod pir0;
#[doc = "Periodic Interval Register 1"]
pub struct PIR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Periodic Interval Register 1"]
pub mod pir1;
#[doc = "Clock Control Register"]
pub struct CLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clock;
#[doc = "Digital Tuner Register"]
pub struct DTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Tuner Register"]
pub mod dtr;
#[doc = "Event Enable Register"]
pub struct EVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable Register"]
pub mod eve;
#[doc = "Event Disable Register"]
pub struct EVD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Disable Register"]
pub mod evd;
#[doc = "Event Mask Register"]
pub struct EVM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Mask Register"]
pub mod evm;
#[doc = "Calendar Value"]
pub struct CALV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calendar Value"]
pub mod calv;
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
