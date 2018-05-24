#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register Channel"]
    pub ccr0: CCR,
    #[doc = "0x04 - Channel Mode Register Channel"]
    pub cmr0_capture: CMR_CAPTURE,
    #[doc = "0x08 - Stepper Motor Mode Register"]
    pub smmr0: SMMR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Counter Value Channel"]
    pub cv0: CV,
    #[doc = "0x14 - Register A Channel"]
    pub ra0: RA,
    #[doc = "0x18 - Register B Channel"]
    pub rb0: RB,
    #[doc = "0x1c - Register C Channel"]
    pub rc0: RC,
    #[doc = "0x20 - Status Register Channel"]
    pub sr0: SR,
    #[doc = "0x24 - Interrupt Enable Register Channel"]
    pub ier0: IER,
    #[doc = "0x28 - Interrupt Disable Register Channel"]
    pub idr0: IDR,
    #[doc = "0x2c - Interrupt Mask Register Channel"]
    pub imr0: IMR,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Channel Control Register Channel"]
    pub ccr1: CCR,
    #[doc = "0x44 - Channel Mode Register Channel"]
    pub cmr1_capture: CMR_CAPTURE,
    #[doc = "0x48 - Stepper Motor Mode Register"]
    pub smmr1: SMMR,
    _reserved2: [u8; 4usize],
    #[doc = "0x50 - Counter Value Channel"]
    pub cv1: CV,
    #[doc = "0x54 - Register A Channel"]
    pub ra1: RA,
    #[doc = "0x58 - Register B Channel"]
    pub rb1: RB,
    #[doc = "0x5c - Register C Channel"]
    pub rc1: RC,
    #[doc = "0x60 - Status Register Channel"]
    pub sr1: SR,
    #[doc = "0x64 - Interrupt Enable Register Channel"]
    pub ier1: IER,
    #[doc = "0x68 - Interrupt Disable Register Channel"]
    pub idr1: IDR,
    #[doc = "0x6c - Interrupt Mask Register Channel"]
    pub imr1: IMR,
    _reserved3: [u8; 16usize],
    #[doc = "0x80 - Channel Control Register Channel"]
    pub ccr2: CCR,
    #[doc = "0x84 - Channel Mode Register Channel"]
    pub cmr2_capture: CMR_CAPTURE,
    #[doc = "0x88 - Stepper Motor Mode Register"]
    pub smmr2: SMMR,
    _reserved4: [u8; 4usize],
    #[doc = "0x90 - Counter Value Channel"]
    pub cv2: CV,
    #[doc = "0x94 - Register A Channel"]
    pub ra2: RA,
    #[doc = "0x98 - Register B Channel"]
    pub rb2: RB,
    #[doc = "0x9c - Register C Channel"]
    pub rc2: RC,
    #[doc = "0xa0 - Status Register Channel"]
    pub sr2: SR,
    #[doc = "0xa4 - Interrupt Enable Register Channel"]
    pub ier2: IER,
    #[doc = "0xa8 - Interrupt Disable Register Channel"]
    pub idr2: IDR,
    #[doc = "0xac - Interrupt Mask Register Channel"]
    pub imr2: IMR,
    _reserved5: [u8; 16usize],
    #[doc = "0xc0 - TC Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - TC Block Mode Register"]
    pub bmr: BMR,
    _reserved6: [u8; 28usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    _reserved7: [u8; 16usize],
    #[doc = "0xf8 - Features Register"]
    pub features: FEATURES,
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Channel Control Register Channel"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register Channel"]
pub mod ccr;
#[doc = "Channel Mode Register Channel"]
pub struct CMR_CAPTURE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register Channel"]
pub mod cmr_capture;
#[doc = "Channel Mode Register Channel"]
pub struct CMR_WAVEFORM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mode Register Channel"]
pub mod cmr_waveform;
#[doc = "Stepper Motor Mode Register"]
pub struct SMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "Counter Value Channel"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value Channel"]
pub mod cv;
#[doc = "Register A Channel"]
pub struct RA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register A Channel"]
pub mod ra;
#[doc = "Register B Channel"]
pub struct RB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register B Channel"]
pub mod rb;
#[doc = "Register C Channel"]
pub struct RC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register C Channel"]
pub mod rc;
#[doc = "Status Register Channel"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register Channel"]
pub mod sr;
#[doc = "Interrupt Enable Register Channel"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register Channel"]
pub mod ier;
#[doc = "Interrupt Disable Register Channel"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register Channel"]
pub mod idr;
#[doc = "Interrupt Mask Register Channel"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register Channel"]
pub mod imr;
#[doc = "TC Block Control Register"]
pub struct BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TC Block Control Register"]
pub mod bcr;
#[doc = "TC Block Mode Register"]
pub struct BMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TC Block Mode Register"]
pub mod bmr;
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Features Register"]
pub struct FEATURES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Features Register"]
pub mod features;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
