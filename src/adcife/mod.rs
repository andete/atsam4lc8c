#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x10 - Resistive Touch Screen Register"]
    pub rts: RTS,
    #[doc = "0x14 - Sequencer Configuration Register"]
    pub seqcfg: SEQCFG,
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    pub cdma_first_dma_word: CDMA_FIRST_DMA_WORD,
    #[doc = "0x1c - Timing Configuration Register"]
    pub tim: TIM,
    #[doc = "0x20 - Internal Timer Register"]
    pub itimer: ITIMER,
    #[doc = "0x24 - Window Monitor Configuration Register"]
    pub wcfg: WCFG,
    #[doc = "0x28 - Window Monitor Threshold Configuration Register"]
    pub wth: WTH,
    #[doc = "0x2c - Sequencer Last Converted Value Register"]
    pub lcv: LCV,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x34 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x38 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x3c - Calibration Register"]
    pub calib: CALIB,
    #[doc = "0x40 - Version Register"]
    pub version: VERSION,
    #[doc = "0x44 - Parameter Register"]
    pub parameter: PARAMETER,
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
#[doc = "Resistive Touch Screen Register"]
pub struct RTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resistive Touch Screen Register"]
pub mod rts;
#[doc = "Sequencer Configuration Register"]
pub struct SEQCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequencer Configuration Register"]
pub mod seqcfg;
#[doc = "Configuration Direct Memory Access Register"]
pub struct CDMA_FIRST_DMA_WORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Direct Memory Access Register"]
pub mod cdma_first_dma_word;
#[doc = "Configuration Direct Memory Access Register"]
pub struct CDMA_SECOND_DMA_WORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Direct Memory Access Register"]
pub mod cdma_second_dma_word;
#[doc = "Timing Configuration Register"]
pub struct TIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Configuration Register"]
pub mod tim;
#[doc = "Internal Timer Register"]
pub struct ITIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal Timer Register"]
pub mod itimer;
#[doc = "Window Monitor Configuration Register"]
pub struct WCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Window Monitor Configuration Register"]
pub mod wcfg;
#[doc = "Window Monitor Threshold Configuration Register"]
pub struct WTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Window Monitor Threshold Configuration Register"]
pub mod wth;
#[doc = "Sequencer Last Converted Value Register"]
pub struct LCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequencer Last Converted Value Register"]
pub mod lcv;
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
#[doc = "Calibration Register"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod calib;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
#[doc = "Parameter Register"]
pub struct PARAMETER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod parameter;
