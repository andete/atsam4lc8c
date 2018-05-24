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
    #[doc = "0x20 - Oscillator Control Register"]
    pub oscctrl0: OSCCTRL0,
    #[doc = "0x24 - PLL0 Control Register"]
    pub pll: PLL,
    #[doc = "0x28 - DFLL0 Config Register"]
    pub dfll0conf: DFLL0CONF,
    #[doc = "0x2c - DFLL Value Register"]
    pub dfll0val: DFLL0VAL,
    #[doc = "0x30 - DFLL0 Multiplier Register"]
    pub dfll0mul: DFLL0MUL,
    #[doc = "0x34 - DFLL0 Step Register"]
    pub dfll0step: DFLL0STEP,
    #[doc = "0x38 - DFLL0 Spread Spectrum Generator Control Register"]
    pub dfll0ssg: DFLL0SSG,
    #[doc = "0x3c - DFLL0 Ratio Registe"]
    pub dfll0ratio: DFLL0RATIO,
    #[doc = "0x40 - DFLL0 Synchronization Register"]
    pub dfll0sync: DFLL0SYNC,
    #[doc = "0x44 - System RC Oscillator Calibration Register"]
    pub rccr: RCCR,
    #[doc = "0x48 - 4/8/12 MHz RC Oscillator Configuration Register"]
    pub rcfastcfg: RCFASTCFG,
    #[doc = "0x4c - 4/8/12 MHz RC Oscillator Status Register"]
    pub rcfastsr: RCFASTSR,
    #[doc = "0x50 - 80 MHz RC Oscillator Register"]
    pub rc80mcr: RC80MCR,
    _reserved0: [u8; 16usize],
    #[doc = "0x64 - High Resolution Prescaler Control Register"]
    pub hrpcr: HRPCR,
    #[doc = "0x68 - Fractional Prescaler Control Register"]
    pub fpcr: FPCR,
    #[doc = "0x6c - Fractional Prescaler Multiplier Register"]
    pub fpmul: FPMUL,
    #[doc = "0x70 - Fractional Prescaler DIVIDER Register"]
    pub fpdiv: FPDIV,
    #[doc = "0x74 - Generic Clock Control"]
    pub gcctrl: [GCCTRL; 12],
    _reserved1: [u8; 820usize],
    #[doc = "0x3d8 - 4/8/12 MHz RC Oscillator Version Register"]
    pub rcfastversion: RCFASTVERSION,
    #[doc = "0x3dc - Generic Clock Prescaler Version Register"]
    pub gclkprescversion: GCLKPRESCVERSION,
    #[doc = "0x3e0 - PLL Version Register"]
    pub pllifaversion: PLLIFAVERSION,
    #[doc = "0x3e4 - Oscillator 0 Version Register"]
    pub oscifaversion: OSCIFAVERSION,
    #[doc = "0x3e8 - DFLL Version Register"]
    pub dfllifbversion: DFLLIFBVERSION,
    #[doc = "0x3ec - System RC Oscillator Version Register"]
    pub rcoscifaversion: RCOSCIFAVERSION,
    #[doc = "0x3f0 - Frequency Locked Oscillator Version Register"]
    pub floversion: FLOVERSION,
    #[doc = "0x3f4 - 80MHz RC Oscillator Version Register"]
    pub rc80mversion: RC80MVERSION,
    #[doc = "0x3f8 - Generic Clock Version Register"]
    pub gclkifversion: GCLKIFVERSION,
    #[doc = "0x3fc - SCIF Version Register"]
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
#[doc = "Oscillator Control Register"]
pub struct OSCCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Control Register"]
pub mod oscctrl0;
#[doc = "PLL0 Control Register"]
pub struct PLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Control Register"]
pub mod pll;
#[doc = "DFLL0 Config Register"]
pub struct DFLL0CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL0 Config Register"]
pub mod dfll0conf;
#[doc = "DFLL Value Register"]
pub struct DFLL0VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL Value Register"]
pub mod dfll0val;
#[doc = "DFLL0 Multiplier Register"]
pub struct DFLL0MUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL0 Multiplier Register"]
pub mod dfll0mul;
#[doc = "DFLL0 Step Register"]
pub struct DFLL0STEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL0 Step Register"]
pub mod dfll0step;
#[doc = "DFLL0 Spread Spectrum Generator Control Register"]
pub struct DFLL0SSG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL0 Spread Spectrum Generator Control Register"]
pub mod dfll0ssg;
#[doc = "DFLL0 Ratio Registe"]
pub struct DFLL0RATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL0 Ratio Registe"]
pub mod dfll0ratio;
#[doc = "DFLL0 Synchronization Register"]
pub struct DFLL0SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL0 Synchronization Register"]
pub mod dfll0sync;
#[doc = "System RC Oscillator Calibration Register"]
pub struct RCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System RC Oscillator Calibration Register"]
pub mod rccr;
#[doc = "4/8/12 MHz RC Oscillator Configuration Register"]
pub struct RCFASTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "4/8/12 MHz RC Oscillator Configuration Register"]
pub mod rcfastcfg;
#[doc = "4/8/12 MHz RC Oscillator Status Register"]
pub struct RCFASTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "4/8/12 MHz RC Oscillator Status Register"]
pub mod rcfastsr;
#[doc = "80 MHz RC Oscillator Register"]
pub struct RC80MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "80 MHz RC Oscillator Register"]
pub mod rc80mcr;
#[doc = "High Resolution Prescaler Control Register"]
pub struct HRPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Resolution Prescaler Control Register"]
pub mod hrpcr;
#[doc = "Fractional Prescaler Control Register"]
pub struct FPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Prescaler Control Register"]
pub mod fpcr;
#[doc = "Fractional Prescaler Multiplier Register"]
pub struct FPMUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Prescaler Multiplier Register"]
pub mod fpmul;
#[doc = "Fractional Prescaler DIVIDER Register"]
pub struct FPDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Prescaler DIVIDER Register"]
pub mod fpdiv;
#[doc = "Generic Clock Control"]
pub struct GCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic Clock Control"]
pub mod gcctrl;
#[doc = "4/8/12 MHz RC Oscillator Version Register"]
pub struct RCFASTVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "4/8/12 MHz RC Oscillator Version Register"]
pub mod rcfastversion;
#[doc = "Generic Clock Prescaler Version Register"]
pub struct GCLKPRESCVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic Clock Prescaler Version Register"]
pub mod gclkprescversion;
#[doc = "PLL Version Register"]
pub struct PLLIFAVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Version Register"]
pub mod pllifaversion;
#[doc = "Oscillator 0 Version Register"]
pub struct OSCIFAVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator 0 Version Register"]
pub mod oscifaversion;
#[doc = "DFLL Version Register"]
pub struct DFLLIFBVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFLL Version Register"]
pub mod dfllifbversion;
#[doc = "System RC Oscillator Version Register"]
pub struct RCOSCIFAVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System RC Oscillator Version Register"]
pub mod rcoscifaversion;
#[doc = "Frequency Locked Oscillator Version Register"]
pub struct FLOVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency Locked Oscillator Version Register"]
pub mod floversion;
#[doc = "80MHz RC Oscillator Version Register"]
pub struct RC80MVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "80MHz RC Oscillator Version Register"]
pub mod rc80mversion;
#[doc = "Generic Clock Version Register"]
pub struct GCLKIFVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic Clock Version Register"]
pub mod gclkifversion;
#[doc = "SCIF Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCIF Version Register"]
pub mod version;
