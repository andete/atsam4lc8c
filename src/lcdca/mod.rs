#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Timing Register"]
    pub tim: TIM,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x14 - Data Register Low 0"]
    pub drl0: DRL0,
    #[doc = "0x18 - Data Register High 0"]
    pub drh0: DRH0,
    #[doc = "0x1c - Data Register Low 1"]
    pub drl1: DRL1,
    #[doc = "0x20 - Data Register High 1"]
    pub drh1: DRH1,
    #[doc = "0x24 - Data Register Low 2"]
    pub drl2: DRL2,
    #[doc = "0x28 - Data Register High 2"]
    pub drh2: DRH2,
    #[doc = "0x2c - Data Register Low 3"]
    pub drl3: DRL3,
    #[doc = "0x30 - Data Register High 3"]
    pub drh3: DRH3,
    #[doc = "0x34 - Indirect Access Data Register"]
    pub iadr: IADR,
    #[doc = "0x38 - Blink Configuration Register"]
    pub bcfg: BCFG,
    #[doc = "0x3c - Circular Shift Register Configuration"]
    pub csrcfg: CSRCFG,
    #[doc = "0x40 - Character Mapping Configuration Register"]
    pub cmcfg: CMCFG,
    #[doc = "0x44 - Character Mapping Data Register"]
    pub cmdr: CMDR,
    #[doc = "0x48 - Automated Character Mapping Configuration Register"]
    pub acmcfg: ACMCFG,
    #[doc = "0x4c - Automated Character Mapping Data Register"]
    pub acmdr: ACMDR,
    #[doc = "0x50 - Automated Bit Mapping Configuration Register"]
    pub abmcfg: ABMCFG,
    #[doc = "0x54 - Automated Bit Mapping Data Register"]
    pub abmdr: ABMDR,
    #[doc = "0x58 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x5c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x60 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x64 - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Timing Register"]
pub struct TIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Register"]
pub mod tim;
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
#[doc = "Data Register Low 0"]
pub struct DRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register Low 0"]
pub mod drl0;
#[doc = "Data Register High 0"]
pub struct DRH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register High 0"]
pub mod drh0;
#[doc = "Data Register Low 1"]
pub struct DRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register Low 1"]
pub mod drl1;
#[doc = "Data Register High 1"]
pub struct DRH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register High 1"]
pub mod drh1;
#[doc = "Data Register Low 2"]
pub struct DRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register Low 2"]
pub mod drl2;
#[doc = "Data Register High 2"]
pub struct DRH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register High 2"]
pub mod drh2;
#[doc = "Data Register Low 3"]
pub struct DRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register Low 3"]
pub mod drl3;
#[doc = "Data Register High 3"]
pub struct DRH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register High 3"]
pub mod drh3;
#[doc = "Indirect Access Data Register"]
pub struct IADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Access Data Register"]
pub mod iadr;
#[doc = "Blink Configuration Register"]
pub struct BCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Blink Configuration Register"]
pub mod bcfg;
#[doc = "Circular Shift Register Configuration"]
pub struct CSRCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Circular Shift Register Configuration"]
pub mod csrcfg;
#[doc = "Character Mapping Configuration Register"]
pub struct CMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Character Mapping Configuration Register"]
pub mod cmcfg;
#[doc = "Character Mapping Data Register"]
pub struct CMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Character Mapping Data Register"]
pub mod cmdr;
#[doc = "Automated Character Mapping Configuration Register"]
pub struct ACMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Automated Character Mapping Configuration Register"]
pub mod acmcfg;
#[doc = "Automated Character Mapping Data Register"]
pub struct ACMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Automated Character Mapping Data Register"]
pub mod acmdr;
#[doc = "Automated Bit Mapping Configuration Register"]
pub struct ABMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Automated Bit Mapping Configuration Register"]
pub mod abmcfg;
#[doc = "Automated Bit Mapping Data Register"]
pub struct ABMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Automated Bit Mapping Data Register"]
pub mod abmdr;
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
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
