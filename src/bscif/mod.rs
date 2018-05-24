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
    #[doc = "0x14 - Power and Clocks Status Register"]
    pub pclksr: PCLKSR,
    #[doc = "0x18 - Unlock Register"]
    pub unlock: UNLOCK,
    #[doc = "0x1c - Chip Specific Configuration Register"]
    pub cscr: CSCR,
    #[doc = "0x20 - Oscillator 32 Control Register"]
    pub oscctrl32: OSCCTRL32,
    #[doc = "0x24 - 32 kHz RC Oscillator Control Register"]
    pub rc32kcr: RC32KCR,
    #[doc = "0x28 - 32kHz RC Oscillator Tuning Register"]
    pub rc32ktune: RC32KTUNE,
    #[doc = "0x2c - BOD33 Control Register"]
    pub bod33ctrl: BOD33CTRL,
    #[doc = "0x30 - BOD33 Level Register"]
    pub bod33level: BOD33LEVEL,
    #[doc = "0x34 - BOD33 Sampling Control Register"]
    pub bod33sampling: BOD33SAMPLING,
    #[doc = "0x38 - BOD18 Control Register"]
    pub bod18ctrl: BOD18CTRL,
    #[doc = "0x3c - BOD18 Level Register"]
    pub bod18level: BOD18LEVEL,
    _reserved0: [u8; 4usize],
    #[doc = "0x44 - Voltage Regulator Configuration Register"]
    pub vregcr: VREGCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x4c - Normal Mode Control and Status Register"]
    pub vregncsr: VREGNCSR,
    #[doc = "0x50 - LP Mode Control and Status Register"]
    pub vreglpcsr: VREGLPCSR,
    _reserved2: [u8; 4usize],
    #[doc = "0x58 - 1MHz RC Clock Configuration Register"]
    pub rc1mcr: RC1MCR,
    #[doc = "0x5c - Bandgap Calibration Register"]
    pub bgcr: BGCR,
    #[doc = "0x60 - Bandgap Control Register"]
    pub bgctrl: BGCTRL,
    #[doc = "0x64 - Bandgap Status Register"]
    pub bgsr: BGSR,
    _reserved3: [u8; 16usize],
    #[doc = "0x78 - Backup Register"]
    pub br: [BR; 4],
    _reserved4: [u8; 860usize],
    #[doc = "0x3e4 - Backup Register Interface Version Register"]
    pub brifbversion: BRIFBVERSION,
    #[doc = "0x3e8 - BGREFIFB Version Register"]
    pub bgrefifbversion: BGREFIFBVERSION,
    #[doc = "0x3ec - VREGIFA Version Register"]
    pub vregifgversion: VREGIFGVERSION,
    #[doc = "0x3f0 - BODIFC Version Register"]
    pub bodifcversion: BODIFCVERSION,
    #[doc = "0x3f4 - 32 kHz RC Oscillator Version Register"]
    pub rc32kifbversion: RC32KIFBVERSION,
    #[doc = "0x3f8 - 32 KHz Oscillator Version Register"]
    pub osc32ifaversion: OSC32IFAVERSION,
    #[doc = "0x3fc - BSCIF Version Register"]
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
#[doc = "Power and Clocks Status Register"]
pub struct PCLKSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and Clocks Status Register"]
pub mod pclksr;
#[doc = "Unlock Register"]
pub struct UNLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "Chip Specific Configuration Register"]
pub struct CSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Specific Configuration Register"]
pub mod cscr;
#[doc = "Oscillator 32 Control Register"]
pub struct OSCCTRL32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator 32 Control Register"]
pub mod oscctrl32;
#[doc = "32 kHz RC Oscillator Control Register"]
pub struct RC32KCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 kHz RC Oscillator Control Register"]
pub mod rc32kcr;
#[doc = "32kHz RC Oscillator Tuning Register"]
pub struct RC32KTUNE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32kHz RC Oscillator Tuning Register"]
pub mod rc32ktune;
#[doc = "BOD33 Control Register"]
pub struct BOD33CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD33 Control Register"]
pub mod bod33ctrl;
#[doc = "BOD33 Level Register"]
pub struct BOD33LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD33 Level Register"]
pub mod bod33level;
#[doc = "BOD33 Sampling Control Register"]
pub struct BOD33SAMPLING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD33 Sampling Control Register"]
pub mod bod33sampling;
#[doc = "BOD18 Control Register"]
pub struct BOD18CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD18 Control Register"]
pub mod bod18ctrl;
#[doc = "BOD18 Level Register"]
pub struct BOD18LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD18 Level Register"]
pub mod bod18level;
#[doc = "Voltage Regulator Configuration Register"]
pub struct VREGCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Regulator Configuration Register"]
pub mod vregcr;
#[doc = "Normal Mode Control and Status Register"]
pub struct VREGNCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Normal Mode Control and Status Register"]
pub mod vregncsr;
#[doc = "LP Mode Control and Status Register"]
pub struct VREGLPCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LP Mode Control and Status Register"]
pub mod vreglpcsr;
#[doc = "1MHz RC Clock Configuration Register"]
pub struct RC1MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1MHz RC Clock Configuration Register"]
pub mod rc1mcr;
#[doc = "Bandgap Calibration Register"]
pub struct BGCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bandgap Calibration Register"]
pub mod bgcr;
#[doc = "Bandgap Control Register"]
pub struct BGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bandgap Control Register"]
pub mod bgctrl;
#[doc = "Bandgap Status Register"]
pub struct BGSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bandgap Status Register"]
pub mod bgsr;
#[doc = "Backup Register"]
pub struct BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Register"]
pub mod br;
#[doc = "Backup Register Interface Version Register"]
pub struct BRIFBVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Register Interface Version Register"]
pub mod brifbversion;
#[doc = "BGREFIFB Version Register"]
pub struct BGREFIFBVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BGREFIFB Version Register"]
pub mod bgrefifbversion;
#[doc = "VREGIFA Version Register"]
pub struct VREGIFGVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREGIFA Version Register"]
pub mod vregifgversion;
#[doc = "BODIFC Version Register"]
pub struct BODIFCVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BODIFC Version Register"]
pub mod bodifcversion;
#[doc = "32 kHz RC Oscillator Version Register"]
pub struct RC32KIFBVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 kHz RC Oscillator Version Register"]
pub mod rc32kifbversion;
#[doc = "32 KHz Oscillator Version Register"]
pub struct OSC32IFAVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 KHz Oscillator Version Register"]
pub mod osc32ifaversion;
#[doc = "BSCIF Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BSCIF Version Register"]
pub mod version;
