#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Clock Control"]
    pub mcctrl: MCCTRL,
    #[doc = "0x04 - CPU Clock Select"]
    pub cpusel: CPUSEL,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - PBA Clock Select"]
    pub pbasel: PBASEL,
    #[doc = "0x10 - PBB Clock Select"]
    pub pbbsel: PBBSEL,
    #[doc = "0x14 - PBC Clock Select"]
    pub pbcsel: PBCSEL,
    #[doc = "0x18 - PBD Clock Select"]
    pub pbdsel: PBDSEL,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - CPU Mask"]
    pub cpumask: CPUMASK,
    #[doc = "0x24 - HSB Mask"]
    pub hsbmask: HSBMASK,
    #[doc = "0x28 - PBA Mask"]
    pub pbamask: PBAMASK,
    #[doc = "0x2c - PBB Mask"]
    pub pbbmask: PBBMASK,
    #[doc = "0x30 - PBC Mask"]
    pub pbcmask: PBCMASK,
    #[doc = "0x34 - PBD Mask"]
    pub pbdmask: PBDMASK,
    _reserved2: [u8; 8usize],
    #[doc = "0x40 - PBA Divided Clock Mask"]
    pub pbadivmask: PBADIVMASK,
    _reserved3: [u8; 16usize],
    #[doc = "0x54 - Clock Failure Detector Control"]
    pub cfdctrl: CFDCTRL,
    #[doc = "0x58 - Unlock Register"]
    pub unlock: UNLOCK,
    _reserved4: [u8; 100usize],
    #[doc = "0xc0 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0xc4 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0xc8 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0xcc - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0xd0 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0xd4 - Status Register"]
    pub sr: SR,
    _reserved5: [u8; 136usize],
    #[doc = "0x160 - Peripheral Power Control Register"]
    pub ppcr: PPCR,
    _reserved6: [u8; 28usize],
    #[doc = "0x180 - Reset Cause Register"]
    pub rcause: RCAUSE,
    #[doc = "0x184 - Wake Cause Register"]
    pub wcause: WCAUSE,
    #[doc = "0x188 - Asynchronous Wake Enable"]
    pub awen: AWEN,
    _reserved7: [u8; 4usize],
    #[doc = "0x190 - Obsvervability"]
    pub obs: OBS,
    #[doc = "0x194 - Fast Sleep Register"]
    pub fastsleep: FASTSLEEP,
    _reserved8: [u8; 608usize],
    #[doc = "0x3f8 - Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x3fc - Version Register"]
    pub version: VERSION,
}
#[doc = "Main Clock Control"]
pub struct MCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main Clock Control"]
pub mod mcctrl;
#[doc = "CPU Clock Select"]
pub struct CPUSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "PBA Clock Select"]
pub struct PBASEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBA Clock Select"]
pub mod pbasel;
#[doc = "PBB Clock Select"]
pub struct PBBSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBB Clock Select"]
pub mod pbbsel;
#[doc = "PBC Clock Select"]
pub struct PBCSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBC Clock Select"]
pub mod pbcsel;
#[doc = "PBD Clock Select"]
pub struct PBDSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBD Clock Select"]
pub mod pbdsel;
#[doc = "CPU Mask"]
pub struct CPUMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Mask"]
pub mod cpumask;
#[doc = "HSB Mask"]
pub struct HSBMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSB Mask"]
pub mod hsbmask;
#[doc = "PBA Mask"]
pub struct PBAMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBA Mask"]
pub mod pbamask;
#[doc = "PBB Mask"]
pub struct PBBMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBB Mask"]
pub mod pbbmask;
#[doc = "PBC Mask"]
pub struct PBCMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBC Mask"]
pub mod pbcmask;
#[doc = "PBD Mask"]
pub struct PBDMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBD Mask"]
pub mod pbdmask;
#[doc = "PBA Divided Clock Mask"]
pub struct PBADIVMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBA Divided Clock Mask"]
pub mod pbadivmask;
#[doc = "Clock Failure Detector Control"]
pub struct CFDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "Unlock Register"]
pub struct UNLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unlock Register"]
pub mod unlock;
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
#[doc = "Peripheral Power Control Register"]
pub struct PPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Power Control Register"]
pub mod ppcr;
#[doc = "Reset Cause Register"]
pub struct RCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Cause Register"]
pub mod rcause;
#[doc = "Wake Cause Register"]
pub struct WCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Cause Register"]
pub mod wcause;
#[doc = "Asynchronous Wake Enable"]
pub struct AWEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous Wake Enable"]
pub mod awen;
#[doc = "Obsvervability"]
pub struct OBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Obsvervability"]
pub mod obs;
#[doc = "Fast Sleep Register"]
pub struct FASTSLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fast Sleep Register"]
pub mod fastsleep;
#[doc = "Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod config;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
