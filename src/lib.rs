#![doc = "Peripheral access API for ATSAM4LC8C microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn HFLASHC_INTREQ();
    fn PDCA_INTREQ_0();
    fn PDCA_INTREQ_1();
    fn PDCA_INTREQ_2();
    fn PDCA_INTREQ_3();
    fn PDCA_INTREQ_4();
    fn PDCA_INTREQ_5();
    fn PDCA_INTREQ_6();
    fn PDCA_INTREQ_7();
    fn PDCA_INTREQ_8();
    fn PDCA_INTREQ_9();
    fn PDCA_INTREQ_10();
    fn PDCA_INTREQ_11();
    fn PDCA_INTREQ_12();
    fn PDCA_INTREQ_13();
    fn PDCA_INTREQ_14();
    fn PDCA_INTREQ_15();
    fn CRCCU_INTREQ();
    fn USBC_INTREQ();
    fn PEVC_INTREQ_0();
    fn PEVC_INTREQ_1();
    fn AESA_INTREQ();
    fn PM_INTREQ();
    fn SCIF_INTREQ();
    fn FREQM_INTREQ();
    fn GPIO_INTREQ_0();
    fn GPIO_INTREQ_1();
    fn GPIO_INTREQ_2();
    fn GPIO_INTREQ_3();
    fn GPIO_INTREQ_4();
    fn GPIO_INTREQ_5();
    fn GPIO_INTREQ_6();
    fn GPIO_INTREQ_7();
    fn GPIO_INTREQ_8();
    fn GPIO_INTREQ_9();
    fn GPIO_INTREQ_10();
    fn GPIO_INTREQ_11();
    fn BPM_INTREQ();
    fn BSCIF_INTREQ();
    fn AST_INTREQ_0();
    fn AST_INTREQ_1();
    fn AST_INTREQ_2();
    fn AST_INTREQ_3();
    fn AST_INTREQ_4();
    fn WDT_INTREQ();
    fn EIC_INTREQ_0();
    fn EIC_INTREQ_1();
    fn EIC_INTREQ_2();
    fn EIC_INTREQ_3();
    fn EIC_INTREQ_4();
    fn EIC_INTREQ_5();
    fn EIC_INTREQ_6();
    fn EIC_INTREQ_7();
    fn IISC_INTREQ();
    fn SPI_INTREQ();
    fn TC0_INTREQ_0();
    fn TC0_INTREQ_1();
    fn TC0_INTREQ_2();
    fn TC1_INTREQ_0();
    fn TC1_INTREQ_1();
    fn TC1_INTREQ_2();
    fn TWIM0_INTREQ();
    fn TWIS0_INTREQ();
    fn TWIM1_INTREQ();
    fn TWIS1_INTREQ();
    fn USART0_INTREQ();
    fn USART1_INTREQ();
    fn USART2_INTREQ();
    fn USART3_INTREQ();
    fn ADCIFE_INTREQ();
    fn DACC_INTREQ();
    fn ACIFC_INTREQ();
    fn ABDACB_INTREQ();
    fn TRNG_INTREQ();
    fn PARC_INTREQ();
    fn CATB_INTREQ();
    fn TWIM2_INTREQ();
    fn TWIM3_INTREQ();
    fn LCDCA_INTREQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 80] = [
    Vector {
        _handler: HFLASHC_INTREQ,
    },
    Vector {
        _handler: PDCA_INTREQ_0,
    },
    Vector {
        _handler: PDCA_INTREQ_1,
    },
    Vector {
        _handler: PDCA_INTREQ_2,
    },
    Vector {
        _handler: PDCA_INTREQ_3,
    },
    Vector {
        _handler: PDCA_INTREQ_4,
    },
    Vector {
        _handler: PDCA_INTREQ_5,
    },
    Vector {
        _handler: PDCA_INTREQ_6,
    },
    Vector {
        _handler: PDCA_INTREQ_7,
    },
    Vector {
        _handler: PDCA_INTREQ_8,
    },
    Vector {
        _handler: PDCA_INTREQ_9,
    },
    Vector {
        _handler: PDCA_INTREQ_10,
    },
    Vector {
        _handler: PDCA_INTREQ_11,
    },
    Vector {
        _handler: PDCA_INTREQ_12,
    },
    Vector {
        _handler: PDCA_INTREQ_13,
    },
    Vector {
        _handler: PDCA_INTREQ_14,
    },
    Vector {
        _handler: PDCA_INTREQ_15,
    },
    Vector {
        _handler: CRCCU_INTREQ,
    },
    Vector {
        _handler: USBC_INTREQ,
    },
    Vector {
        _handler: PEVC_INTREQ_0,
    },
    Vector {
        _handler: PEVC_INTREQ_1,
    },
    Vector {
        _handler: AESA_INTREQ,
    },
    Vector {
        _handler: PM_INTREQ,
    },
    Vector {
        _handler: SCIF_INTREQ,
    },
    Vector {
        _handler: FREQM_INTREQ,
    },
    Vector {
        _handler: GPIO_INTREQ_0,
    },
    Vector {
        _handler: GPIO_INTREQ_1,
    },
    Vector {
        _handler: GPIO_INTREQ_2,
    },
    Vector {
        _handler: GPIO_INTREQ_3,
    },
    Vector {
        _handler: GPIO_INTREQ_4,
    },
    Vector {
        _handler: GPIO_INTREQ_5,
    },
    Vector {
        _handler: GPIO_INTREQ_6,
    },
    Vector {
        _handler: GPIO_INTREQ_7,
    },
    Vector {
        _handler: GPIO_INTREQ_8,
    },
    Vector {
        _handler: GPIO_INTREQ_9,
    },
    Vector {
        _handler: GPIO_INTREQ_10,
    },
    Vector {
        _handler: GPIO_INTREQ_11,
    },
    Vector {
        _handler: BPM_INTREQ,
    },
    Vector {
        _handler: BSCIF_INTREQ,
    },
    Vector {
        _handler: AST_INTREQ_0,
    },
    Vector {
        _handler: AST_INTREQ_1,
    },
    Vector {
        _handler: AST_INTREQ_2,
    },
    Vector {
        _handler: AST_INTREQ_3,
    },
    Vector {
        _handler: AST_INTREQ_4,
    },
    Vector {
        _handler: WDT_INTREQ,
    },
    Vector {
        _handler: EIC_INTREQ_0,
    },
    Vector {
        _handler: EIC_INTREQ_1,
    },
    Vector {
        _handler: EIC_INTREQ_2,
    },
    Vector {
        _handler: EIC_INTREQ_3,
    },
    Vector {
        _handler: EIC_INTREQ_4,
    },
    Vector {
        _handler: EIC_INTREQ_5,
    },
    Vector {
        _handler: EIC_INTREQ_6,
    },
    Vector {
        _handler: EIC_INTREQ_7,
    },
    Vector {
        _handler: IISC_INTREQ,
    },
    Vector {
        _handler: SPI_INTREQ,
    },
    Vector {
        _handler: TC0_INTREQ_0,
    },
    Vector {
        _handler: TC0_INTREQ_1,
    },
    Vector {
        _handler: TC0_INTREQ_2,
    },
    Vector {
        _handler: TC1_INTREQ_0,
    },
    Vector {
        _handler: TC1_INTREQ_1,
    },
    Vector {
        _handler: TC1_INTREQ_2,
    },
    Vector {
        _handler: TWIM0_INTREQ,
    },
    Vector {
        _handler: TWIS0_INTREQ,
    },
    Vector {
        _handler: TWIM1_INTREQ,
    },
    Vector {
        _handler: TWIS1_INTREQ,
    },
    Vector {
        _handler: USART0_INTREQ,
    },
    Vector {
        _handler: USART1_INTREQ,
    },
    Vector {
        _handler: USART2_INTREQ,
    },
    Vector {
        _handler: USART3_INTREQ,
    },
    Vector {
        _handler: ADCIFE_INTREQ,
    },
    Vector {
        _handler: DACC_INTREQ,
    },
    Vector {
        _handler: ACIFC_INTREQ,
    },
    Vector {
        _handler: ABDACB_INTREQ,
    },
    Vector {
        _handler: TRNG_INTREQ,
    },
    Vector {
        _handler: PARC_INTREQ,
    },
    Vector {
        _handler: CATB_INTREQ,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: TWIM2_INTREQ,
    },
    Vector {
        _handler: TWIM3_INTREQ,
    },
    Vector {
        _handler: LCDCA_INTREQ,
    },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - HFLASHC_INTREQ"]
    HFLASHC_INTREQ,
    #[doc = "1 - PDCA_INTREQ_0"]
    PDCA_INTREQ_0,
    #[doc = "2 - PDCA_INTREQ_1"]
    PDCA_INTREQ_1,
    #[doc = "3 - PDCA_INTREQ_2"]
    PDCA_INTREQ_2,
    #[doc = "4 - PDCA_INTREQ_3"]
    PDCA_INTREQ_3,
    #[doc = "5 - PDCA_INTREQ_4"]
    PDCA_INTREQ_4,
    #[doc = "6 - PDCA_INTREQ_5"]
    PDCA_INTREQ_5,
    #[doc = "7 - PDCA_INTREQ_6"]
    PDCA_INTREQ_6,
    #[doc = "8 - PDCA_INTREQ_7"]
    PDCA_INTREQ_7,
    #[doc = "9 - PDCA_INTREQ_8"]
    PDCA_INTREQ_8,
    #[doc = "10 - PDCA_INTREQ_9"]
    PDCA_INTREQ_9,
    #[doc = "11 - PDCA_INTREQ_10"]
    PDCA_INTREQ_10,
    #[doc = "12 - PDCA_INTREQ_11"]
    PDCA_INTREQ_11,
    #[doc = "13 - PDCA_INTREQ_12"]
    PDCA_INTREQ_12,
    #[doc = "14 - PDCA_INTREQ_13"]
    PDCA_INTREQ_13,
    #[doc = "15 - PDCA_INTREQ_14"]
    PDCA_INTREQ_14,
    #[doc = "16 - PDCA_INTREQ_15"]
    PDCA_INTREQ_15,
    #[doc = "17 - CRCCU_INTREQ"]
    CRCCU_INTREQ,
    #[doc = "18 - USBC_INTREQ"]
    USBC_INTREQ,
    #[doc = "19 - PEVC_INTREQ_0"]
    PEVC_INTREQ_0,
    #[doc = "20 - PEVC_INTREQ_1"]
    PEVC_INTREQ_1,
    #[doc = "21 - AESA_INTREQ"]
    AESA_INTREQ,
    #[doc = "22 - PM_INTREQ"]
    PM_INTREQ,
    #[doc = "23 - SCIF_INTREQ"]
    SCIF_INTREQ,
    #[doc = "24 - FREQM_INTREQ"]
    FREQM_INTREQ,
    #[doc = "25 - GPIO_INTREQ_0"]
    GPIO_INTREQ_0,
    #[doc = "26 - GPIO_INTREQ_1"]
    GPIO_INTREQ_1,
    #[doc = "27 - GPIO_INTREQ_2"]
    GPIO_INTREQ_2,
    #[doc = "28 - GPIO_INTREQ_3"]
    GPIO_INTREQ_3,
    #[doc = "29 - GPIO_INTREQ_4"]
    GPIO_INTREQ_4,
    #[doc = "30 - GPIO_INTREQ_5"]
    GPIO_INTREQ_5,
    #[doc = "31 - GPIO_INTREQ_6"]
    GPIO_INTREQ_6,
    #[doc = "32 - GPIO_INTREQ_7"]
    GPIO_INTREQ_7,
    #[doc = "33 - GPIO_INTREQ_8"]
    GPIO_INTREQ_8,
    #[doc = "34 - GPIO_INTREQ_9"]
    GPIO_INTREQ_9,
    #[doc = "35 - GPIO_INTREQ_10"]
    GPIO_INTREQ_10,
    #[doc = "36 - GPIO_INTREQ_11"]
    GPIO_INTREQ_11,
    #[doc = "37 - BPM_INTREQ"]
    BPM_INTREQ,
    #[doc = "38 - BSCIF_INTREQ"]
    BSCIF_INTREQ,
    #[doc = "39 - AST_INTREQ_0"]
    AST_INTREQ_0,
    #[doc = "40 - AST_INTREQ_1"]
    AST_INTREQ_1,
    #[doc = "41 - AST_INTREQ_2"]
    AST_INTREQ_2,
    #[doc = "42 - AST_INTREQ_3"]
    AST_INTREQ_3,
    #[doc = "43 - AST_INTREQ_4"]
    AST_INTREQ_4,
    #[doc = "44 - WDT_INTREQ"]
    WDT_INTREQ,
    #[doc = "45 - EIC_INTREQ_0"]
    EIC_INTREQ_0,
    #[doc = "46 - EIC_INTREQ_1"]
    EIC_INTREQ_1,
    #[doc = "47 - EIC_INTREQ_2"]
    EIC_INTREQ_2,
    #[doc = "48 - EIC_INTREQ_3"]
    EIC_INTREQ_3,
    #[doc = "49 - EIC_INTREQ_4"]
    EIC_INTREQ_4,
    #[doc = "50 - EIC_INTREQ_5"]
    EIC_INTREQ_5,
    #[doc = "51 - EIC_INTREQ_6"]
    EIC_INTREQ_6,
    #[doc = "52 - EIC_INTREQ_7"]
    EIC_INTREQ_7,
    #[doc = "53 - IISC_INTREQ"]
    IISC_INTREQ,
    #[doc = "54 - SPI_INTREQ"]
    SPI_INTREQ,
    #[doc = "55 - TC0_INTREQ_0"]
    TC0_INTREQ_0,
    #[doc = "56 - TC0_INTREQ_1"]
    TC0_INTREQ_1,
    #[doc = "57 - TC0_INTREQ_2"]
    TC0_INTREQ_2,
    #[doc = "58 - TC1_INTREQ_0"]
    TC1_INTREQ_0,
    #[doc = "59 - TC1_INTREQ_1"]
    TC1_INTREQ_1,
    #[doc = "60 - TC1_INTREQ_2"]
    TC1_INTREQ_2,
    #[doc = "61 - TWIM0_INTREQ"]
    TWIM0_INTREQ,
    #[doc = "62 - TWIS0_INTREQ"]
    TWIS0_INTREQ,
    #[doc = "63 - TWIM1_INTREQ"]
    TWIM1_INTREQ,
    #[doc = "64 - TWIS1_INTREQ"]
    TWIS1_INTREQ,
    #[doc = "65 - USART0_INTREQ"]
    USART0_INTREQ,
    #[doc = "66 - USART1_INTREQ"]
    USART1_INTREQ,
    #[doc = "67 - USART2_INTREQ"]
    USART2_INTREQ,
    #[doc = "68 - USART3_INTREQ"]
    USART3_INTREQ,
    #[doc = "69 - ADCIFE_INTREQ"]
    ADCIFE_INTREQ,
    #[doc = "70 - DACC_INTREQ"]
    DACC_INTREQ,
    #[doc = "71 - ACIFC_INTREQ"]
    ACIFC_INTREQ,
    #[doc = "72 - ABDACB_INTREQ"]
    ABDACB_INTREQ,
    #[doc = "73 - TRNG_INTREQ"]
    TRNG_INTREQ,
    #[doc = "74 - PARC_INTREQ"]
    PARC_INTREQ,
    #[doc = "75 - CATB_INTREQ"]
    CATB_INTREQ,
    #[doc = "77 - TWIM2_INTREQ"]
    TWIM2_INTREQ,
    #[doc = "78 - TWIM3_INTREQ"]
    TWIM3_INTREQ,
    #[doc = "79 - LCDCA_INTREQ"]
    LCDCA_INTREQ,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::HFLASHC_INTREQ => 0,
            Interrupt::PDCA_INTREQ_0 => 1,
            Interrupt::PDCA_INTREQ_1 => 2,
            Interrupt::PDCA_INTREQ_2 => 3,
            Interrupt::PDCA_INTREQ_3 => 4,
            Interrupt::PDCA_INTREQ_4 => 5,
            Interrupt::PDCA_INTREQ_5 => 6,
            Interrupt::PDCA_INTREQ_6 => 7,
            Interrupt::PDCA_INTREQ_7 => 8,
            Interrupt::PDCA_INTREQ_8 => 9,
            Interrupt::PDCA_INTREQ_9 => 10,
            Interrupt::PDCA_INTREQ_10 => 11,
            Interrupt::PDCA_INTREQ_11 => 12,
            Interrupt::PDCA_INTREQ_12 => 13,
            Interrupt::PDCA_INTREQ_13 => 14,
            Interrupt::PDCA_INTREQ_14 => 15,
            Interrupt::PDCA_INTREQ_15 => 16,
            Interrupt::CRCCU_INTREQ => 17,
            Interrupt::USBC_INTREQ => 18,
            Interrupt::PEVC_INTREQ_0 => 19,
            Interrupt::PEVC_INTREQ_1 => 20,
            Interrupt::AESA_INTREQ => 21,
            Interrupt::PM_INTREQ => 22,
            Interrupt::SCIF_INTREQ => 23,
            Interrupt::FREQM_INTREQ => 24,
            Interrupt::GPIO_INTREQ_0 => 25,
            Interrupt::GPIO_INTREQ_1 => 26,
            Interrupt::GPIO_INTREQ_2 => 27,
            Interrupt::GPIO_INTREQ_3 => 28,
            Interrupt::GPIO_INTREQ_4 => 29,
            Interrupt::GPIO_INTREQ_5 => 30,
            Interrupt::GPIO_INTREQ_6 => 31,
            Interrupt::GPIO_INTREQ_7 => 32,
            Interrupt::GPIO_INTREQ_8 => 33,
            Interrupt::GPIO_INTREQ_9 => 34,
            Interrupt::GPIO_INTREQ_10 => 35,
            Interrupt::GPIO_INTREQ_11 => 36,
            Interrupt::BPM_INTREQ => 37,
            Interrupt::BSCIF_INTREQ => 38,
            Interrupt::AST_INTREQ_0 => 39,
            Interrupt::AST_INTREQ_1 => 40,
            Interrupt::AST_INTREQ_2 => 41,
            Interrupt::AST_INTREQ_3 => 42,
            Interrupt::AST_INTREQ_4 => 43,
            Interrupt::WDT_INTREQ => 44,
            Interrupt::EIC_INTREQ_0 => 45,
            Interrupt::EIC_INTREQ_1 => 46,
            Interrupt::EIC_INTREQ_2 => 47,
            Interrupt::EIC_INTREQ_3 => 48,
            Interrupt::EIC_INTREQ_4 => 49,
            Interrupt::EIC_INTREQ_5 => 50,
            Interrupt::EIC_INTREQ_6 => 51,
            Interrupt::EIC_INTREQ_7 => 52,
            Interrupt::IISC_INTREQ => 53,
            Interrupt::SPI_INTREQ => 54,
            Interrupt::TC0_INTREQ_0 => 55,
            Interrupt::TC0_INTREQ_1 => 56,
            Interrupt::TC0_INTREQ_2 => 57,
            Interrupt::TC1_INTREQ_0 => 58,
            Interrupt::TC1_INTREQ_1 => 59,
            Interrupt::TC1_INTREQ_2 => 60,
            Interrupt::TWIM0_INTREQ => 61,
            Interrupt::TWIS0_INTREQ => 62,
            Interrupt::TWIM1_INTREQ => 63,
            Interrupt::TWIS1_INTREQ => 64,
            Interrupt::USART0_INTREQ => 65,
            Interrupt::USART1_INTREQ => 66,
            Interrupt::USART2_INTREQ => 67,
            Interrupt::USART3_INTREQ => 68,
            Interrupt::ADCIFE_INTREQ => 69,
            Interrupt::DACC_INTREQ => 70,
            Interrupt::ACIFC_INTREQ => 71,
            Interrupt::ABDACB_INTREQ => 72,
            Interrupt::TRNG_INTREQ => 73,
            Interrupt::PARC_INTREQ => 74,
            Interrupt::CATB_INTREQ => 75,
            Interrupt::TWIM2_INTREQ => 77,
            Interrupt::TWIM3_INTREQ => 78,
            Interrupt::LCDCA_INTREQ => 79,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Audio Bitstream DAC"]
pub struct ABDACB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ABDACB {}
impl ABDACB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const abdacb::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for ABDACB {
    type Target = abdacb::RegisterBlock;
    fn deref(&self) -> &abdacb::RegisterBlock {
        unsafe { &*ABDACB::ptr() }
    }
}
#[doc = "Audio Bitstream DAC"]
pub mod abdacb;
#[doc = "Analog Comparator Interface"]
pub struct ACIFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACIFC {}
impl ACIFC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acifc::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for ACIFC {
    type Target = acifc::RegisterBlock;
    fn deref(&self) -> &acifc::RegisterBlock {
        unsafe { &*ACIFC::ptr() }
    }
}
#[doc = "Analog Comparator Interface"]
pub mod acifc;
#[doc = "ADC controller interface"]
pub struct ADCIFE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADCIFE {}
impl ADCIFE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adcife::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for ADCIFE {
    type Target = adcife::RegisterBlock;
    fn deref(&self) -> &adcife::RegisterBlock {
        unsafe { &*ADCIFE::ptr() }
    }
}
#[doc = "ADC controller interface"]
pub mod adcife;
#[doc = "Advanced Encryption Standard"]
pub struct AESA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AESA {}
impl AESA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aesa::RegisterBlock {
        1074462720 as *const _
    }
}
impl Deref for AESA {
    type Target = aesa::RegisterBlock;
    fn deref(&self) -> &aesa::RegisterBlock {
        unsafe { &*AESA::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub mod aesa;
#[doc = "Asynchronous Timer"]
pub struct AST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AST {}
impl AST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ast::RegisterBlock {
        1074726912 as *const _
    }
}
impl Deref for AST {
    type Target = ast::RegisterBlock;
    fn deref(&self) -> &ast::RegisterBlock {
        unsafe { &*AST::ptr() }
    }
}
#[doc = "Asynchronous Timer"]
pub mod ast;
#[doc = "Backup Power Manager"]
pub struct BPM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPM {}
impl BPM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bpm::RegisterBlock {
        1074724864 as *const _
    }
}
impl Deref for BPM {
    type Target = bpm::RegisterBlock;
    fn deref(&self) -> &bpm::RegisterBlock {
        unsafe { &*BPM::ptr() }
    }
}
#[doc = "Backup Power Manager"]
pub mod bpm;
#[doc = "Backup System Control Interface"]
pub struct BSCIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSCIF {}
impl BSCIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bscif::RegisterBlock {
        1074725888 as *const _
    }
}
impl Deref for BSCIF {
    type Target = bscif::RegisterBlock;
    fn deref(&self) -> &bscif::RegisterBlock {
        unsafe { &*BSCIF::ptr() }
    }
}
#[doc = "Backup System Control Interface"]
pub mod bscif;
#[doc = "Capacitive Touch Module B"]
pub struct CATB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CATB {}
impl CATB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const catb::RegisterBlock {
        1074200576 as *const _
    }
}
impl Deref for CATB {
    type Target = catb::RegisterBlock;
    fn deref(&self) -> &catb::RegisterBlock {
        unsafe { &*CATB::ptr() }
    }
}
#[doc = "Capacitive Touch Module B"]
pub mod catb;
#[doc = "Chip ID Registers"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const chipid::RegisterBlock {
        1074660352 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    fn deref(&self) -> &chipid::RegisterBlock {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip ID Registers"]
pub mod chipid;
#[doc = "CRC Calculation Unit"]
pub struct CRCCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRCCU {}
impl CRCCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crccu::RegisterBlock {
        1074413568 as *const _
    }
}
impl Deref for CRCCU {
    type Target = crccu::RegisterBlock;
    fn deref(&self) -> &crccu::RegisterBlock {
        unsafe { &*CRCCU::ptr() }
    }
}
#[doc = "CRC Calculation Unit"]
pub mod crccu;
#[doc = "DAC Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dacc::RegisterBlock {
        1073987584 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    fn deref(&self) -> &dacc::RegisterBlock {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "DAC Controller"]
pub mod dacc;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eic::RegisterBlock {
        1074728960 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &eic::RegisterBlock {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Flash Controller"]
pub struct HFLASHC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HFLASHC {}
impl HFLASHC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hflashc::RegisterBlock {
        1074397184 as *const _
    }
}
impl Deref for HFLASHC {
    type Target = hflashc::RegisterBlock;
    fn deref(&self) -> &hflashc::RegisterBlock {
        unsafe { &*HFLASHC::ptr() }
    }
}
#[doc = "Flash Controller"]
pub mod hflashc;
#[doc = "Frequency Meter"]
pub struct FREQM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FREQM {}
impl FREQM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const freqm::RegisterBlock {
        1074662400 as *const _
    }
}
impl Deref for FREQM {
    type Target = freqm::RegisterBlock;
    fn deref(&self) -> &freqm::RegisterBlock {
        unsafe { &*FREQM::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub mod freqm;
#[doc = "Glue Logic Controller"]
pub struct GLOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GLOC {}
impl GLOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gloc::RegisterBlock {
        1074135040 as *const _
    }
}
impl Deref for GLOC {
    type Target = gloc::RegisterBlock;
    fn deref(&self) -> &gloc::RegisterBlock {
        unsafe { &*GLOC::ptr() }
    }
}
#[doc = "Glue Logic Controller"]
pub mod gloc;
#[doc = "General-Purpose Input/Output Controller"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General-Purpose Input/Output Controller"]
pub mod gpio;
#[doc = "Cortex M I&D Cache Controller"]
pub struct HCACHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HCACHE {}
impl HCACHE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hcache::RegisterBlock {
        1074398208 as *const _
    }
}
impl Deref for HCACHE {
    type Target = hcache::RegisterBlock;
    fn deref(&self) -> &hcache::RegisterBlock {
        unsafe { &*HCACHE::ptr() }
    }
}
#[doc = "Cortex M I&D Cache Controller"]
pub mod hcache;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hmatrix::RegisterBlock {
        1074401280 as *const _
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    fn deref(&self) -> &hmatrix::RegisterBlock {
        unsafe { &*HMATRIX::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Inter-IC Sound (I2S) Controller"]
pub struct IISC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IISC {}
impl IISC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iisc::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for IISC {
    type Target = iisc::RegisterBlock;
    fn deref(&self) -> &iisc::RegisterBlock {
        unsafe { &*IISC::ptr() }
    }
}
#[doc = "Inter-IC Sound (I2S) Controller"]
pub mod iisc;
#[doc = "LCD Controller"]
pub struct LCDCA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCDCA {}
impl LCDCA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcdca::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for LCDCA {
    type Target = lcdca::RegisterBlock;
    fn deref(&self) -> &lcdca::RegisterBlock {
        unsafe { &*LCDCA::ptr() }
    }
}
#[doc = "LCD Controller"]
pub mod lcdca;
#[doc = "Parallel Capture"]
pub struct PARC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PARC {}
impl PARC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const parc::RegisterBlock {
        1074184192 as *const _
    }
}
impl Deref for PARC {
    type Target = parc::RegisterBlock;
    fn deref(&self) -> &parc::RegisterBlock {
        unsafe { &*PARC::ptr() }
    }
}
#[doc = "Parallel Capture"]
pub mod parc;
#[doc = "Peripheral DMA Controller"]
pub struct PDCA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDCA {}
impl PDCA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdca::RegisterBlock {
        1074405376 as *const _
    }
}
impl Deref for PDCA {
    type Target = pdca::RegisterBlock;
    fn deref(&self) -> &pdca::RegisterBlock {
        unsafe { &*PDCA::ptr() }
    }
}
#[doc = "Peripheral DMA Controller"]
pub mod pdca;
#[doc = "Peripheral Event Controller"]
pub struct PEVC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PEVC {}
impl PEVC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pevc::RegisterBlock {
        1074421760 as *const _
    }
}
impl Deref for PEVC {
    type Target = pevc::RegisterBlock;
    fn deref(&self) -> &pevc::RegisterBlock {
        unsafe { &*PEVC::ptr() }
    }
}
#[doc = "Peripheral Event Controller"]
pub mod pevc;
#[doc = "Pico UART"]
pub struct PICOUART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PICOUART {}
impl PICOUART {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const picouart::RegisterBlock {
        1074729984 as *const _
    }
}
impl Deref for PICOUART {
    type Target = picouart::RegisterBlock;
    fn deref(&self) -> &picouart::RegisterBlock {
        unsafe { &*PICOUART::ptr() }
    }
}
#[doc = "Pico UART"]
pub mod picouart;
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pm::RegisterBlock {
        1074659328 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    fn deref(&self) -> &pm::RegisterBlock {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "System Control Interface"]
pub struct SCIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCIF {}
impl SCIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scif::RegisterBlock {
        1074661376 as *const _
    }
}
impl Deref for SCIF {
    type Target = scif::RegisterBlock;
    fn deref(&self) -> &scif::RegisterBlock {
        unsafe { &*SCIF::ptr() }
    }
}
#[doc = "System Control Interface"]
pub mod scif;
#[doc = "System Manager Access Port"]
pub struct SMAP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMAP {}
impl SMAP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smap::RegisterBlock {
        1074409472 as *const _
    }
}
impl Deref for SMAP {
    type Target = smap::RegisterBlock;
    fn deref(&self) -> &smap::RegisterBlock {
        unsafe { &*SMAP::ptr() }
    }
}
#[doc = "System Manager Access Port"]
pub mod smap;
#[doc = "Serial Peripheral Interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &spi::RegisterBlock {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi;
#[doc = "Timer/Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod tc0;
#[doc = "Timer/Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng::RegisterBlock {
        1074167808 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &trng::RegisterBlock {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Two-wire Master Interface 0"]
pub struct TWIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0 {}
impl TWIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for TWIM0 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM0::ptr() }
    }
}
#[doc = "Two-wire Master Interface 0"]
pub mod twim0;
#[doc = "Two-wire Master Interface 1"]
pub struct TWIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1 {}
impl TWIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for TWIM1 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM1::ptr() }
    }
}
#[doc = "Two-wire Master Interface 2"]
pub struct TWIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM2 {}
impl TWIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1074233344 as *const _
    }
}
impl Deref for TWIM2 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM2::ptr() }
    }
}
#[doc = "Two-wire Master Interface 3"]
pub struct TWIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM3 {}
impl TWIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1074249728 as *const _
    }
}
impl Deref for TWIM3 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM3::ptr() }
    }
}
#[doc = "Two-wire Slave Interface 0"]
pub struct TWIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0 {}
impl TWIS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0::RegisterBlock {
        1073841152 as *const _
    }
}
impl Deref for TWIS0 {
    type Target = twis0::RegisterBlock;
    fn deref(&self) -> &twis0::RegisterBlock {
        unsafe { &*TWIS0::ptr() }
    }
}
#[doc = "Two-wire Slave Interface 0"]
pub mod twis0;
#[doc = "Two-wire Slave Interface 1"]
pub struct TWIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1 {}
impl TWIS1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0::RegisterBlock {
        1073857536 as *const _
    }
}
impl Deref for TWIS1 {
    type Target = twis0::RegisterBlock;
    fn deref(&self) -> &twis0::RegisterBlock {
        unsafe { &*TWIS1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "USB 2.0 Interface"]
pub struct USBC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBC {}
impl USBC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usbc::RegisterBlock {
        1074417664 as *const _
    }
}
impl Deref for USBC {
    type Target = usbc::RegisterBlock;
    fn deref(&self) -> &usbc::RegisterBlock {
        unsafe { &*USBC::ptr() }
    }
}
#[doc = "USB 2.0 Interface"]
pub mod usbc;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1074727936 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ABDACB"]
    pub ABDACB: ABDACB,
    #[doc = "ACIFC"]
    pub ACIFC: ACIFC,
    #[doc = "ADCIFE"]
    pub ADCIFE: ADCIFE,
    #[doc = "AESA"]
    pub AESA: AESA,
    #[doc = "AST"]
    pub AST: AST,
    #[doc = "BPM"]
    pub BPM: BPM,
    #[doc = "BSCIF"]
    pub BSCIF: BSCIF,
    #[doc = "CATB"]
    pub CATB: CATB,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "CRCCU"]
    pub CRCCU: CRCCU,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "HFLASHC"]
    pub HFLASHC: HFLASHC,
    #[doc = "FREQM"]
    pub FREQM: FREQM,
    #[doc = "GLOC"]
    pub GLOC: GLOC,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "HCACHE"]
    pub HCACHE: HCACHE,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "IISC"]
    pub IISC: IISC,
    #[doc = "LCDCA"]
    pub LCDCA: LCDCA,
    #[doc = "PARC"]
    pub PARC: PARC,
    #[doc = "PDCA"]
    pub PDCA: PDCA,
    #[doc = "PEVC"]
    pub PEVC: PEVC,
    #[doc = "PICOUART"]
    pub PICOUART: PICOUART,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "SCIF"]
    pub SCIF: SCIF,
    #[doc = "SMAP"]
    pub SMAP: SMAP,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "TWIM0"]
    pub TWIM0: TWIM0,
    #[doc = "TWIM1"]
    pub TWIM1: TWIM1,
    #[doc = "TWIM2"]
    pub TWIM2: TWIM2,
    #[doc = "TWIM3"]
    pub TWIM3: TWIM3,
    #[doc = "TWIS0"]
    pub TWIS0: TWIS0,
    #[doc = "TWIS1"]
    pub TWIS1: TWIS1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USBC"]
    pub USBC: USBC,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ABDACB: ABDACB {
                _marker: PhantomData,
            },
            ACIFC: ACIFC {
                _marker: PhantomData,
            },
            ADCIFE: ADCIFE {
                _marker: PhantomData,
            },
            AESA: AESA {
                _marker: PhantomData,
            },
            AST: AST {
                _marker: PhantomData,
            },
            BPM: BPM {
                _marker: PhantomData,
            },
            BSCIF: BSCIF {
                _marker: PhantomData,
            },
            CATB: CATB {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            CRCCU: CRCCU {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            HFLASHC: HFLASHC {
                _marker: PhantomData,
            },
            FREQM: FREQM {
                _marker: PhantomData,
            },
            GLOC: GLOC {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            HCACHE: HCACHE {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            IISC: IISC {
                _marker: PhantomData,
            },
            LCDCA: LCDCA {
                _marker: PhantomData,
            },
            PARC: PARC {
                _marker: PhantomData,
            },
            PDCA: PDCA {
                _marker: PhantomData,
            },
            PEVC: PEVC {
                _marker: PhantomData,
            },
            PICOUART: PICOUART {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            SCIF: SCIF {
                _marker: PhantomData,
            },
            SMAP: SMAP {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            TWIM0: TWIM0 {
                _marker: PhantomData,
            },
            TWIM1: TWIM1 {
                _marker: PhantomData,
            },
            TWIM2: TWIM2 {
                _marker: PhantomData,
            },
            TWIM3: TWIM3 {
                _marker: PhantomData,
            },
            TWIS0: TWIS0 {
                _marker: PhantomData,
            },
            TWIS1: TWIS1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USBC: USBC {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
