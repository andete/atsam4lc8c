#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr_lin_mode: CR_LIN_MODE,
    #[doc = "0x04 - Mode Register"]
    pub mr_spi_mode: MR_SPI_MODE,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier_lin_mode: IER_LIN_MODE,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub idr_lin_mode: IDR_LIN_MODE,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub imr_lin_mode: IMR_LIN_MODE,
    #[doc = "0x14 - Channel Status Register"]
    pub csr_lin_mode: CSR_LIN_MODE,
    #[doc = "0x18 - Receiver Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmitter Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: TTGR,
    _reserved0: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: NER,
    _reserved1: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub ifr: IFR,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: LINBRR,
    _reserved2: [u8; 132usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved3: [u8; 16usize],
    #[doc = "0xfc - Version Register"]
    pub version: VERSION,
}
#[doc = "Control Register"]
pub struct CR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr_lin_mode;
#[doc = "Control Register"]
pub struct CR_SPI_MASTER_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr_spi_master_mode;
#[doc = "Control Register"]
pub struct CR_USART_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr_usart_mode;
#[doc = "Mode Register"]
pub struct MR_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr_spi_mode;
#[doc = "Mode Register"]
pub struct MR_USART_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr_usart_mode;
#[doc = "Interrupt Enable Register"]
pub struct IER_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier_lin_mode;
#[doc = "Interrupt Enable Register"]
pub struct IER_SPI_SLAVE_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier_spi_slave_mode;
#[doc = "Interrupt Enable Register"]
pub struct IER_USART_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier_usart_mode;
#[doc = "Interrupt Disable Register"]
pub struct IDR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr_lin_mode;
#[doc = "Interrupt Disable Register"]
pub struct IDR_SPI_SLAVE_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr_spi_slave_mode;
#[doc = "Interrupt Disable Register"]
pub struct IDR_USART_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr_usart_mode;
#[doc = "Interrupt Mask Register"]
pub struct IMR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr_lin_mode;
#[doc = "Interrupt Mask Register"]
pub struct IMR_SPI_SLAVE_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr_spi_slave_mode;
#[doc = "Interrupt Mask Register"]
pub struct IMR_USART_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr_usart_mode;
#[doc = "Channel Status Register"]
pub struct CSR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr_lin_mode;
#[doc = "Channel Status Register"]
pub struct CSR_SPI_SLAVE_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr_spi_slave_mode;
#[doc = "Channel Status Register"]
pub struct CSR_USART_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr_usart_mode;
#[doc = "Receiver Holding Register"]
pub struct RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "Transmitter Holding Register"]
pub struct THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Holding Register"]
pub mod thr;
#[doc = "Baud Rate Generator Register"]
pub struct BRGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "Receiver Time-out Register"]
pub struct RTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "Transmitter Timeguard Register"]
pub struct TTGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FI DI Ratio Register"]
pub struct FIDI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "Number of Errors Register"]
pub struct NER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "IrDA Filter Register"]
pub struct IFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IrDA Filter Register"]
pub mod ifr;
#[doc = "Manchester Configuration Register"]
pub struct MAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "LIN Mode Register"]
pub struct LINMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LIN Identifier Register"]
pub struct LINIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LIN Baud Rate Register"]
pub struct LINBRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
