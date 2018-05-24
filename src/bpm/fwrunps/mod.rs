#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FWRUNPS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct REGLEVELR {
    bits: u8,
}
impl REGLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `REGTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGTYPER {
    #[doc = "undocumented"]
    NORMAL,
    #[doc = "undocumented"]
    LP,
    #[doc = "undocumented"]
    XULP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REGTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REGTYPER::NORMAL => 0,
            REGTYPER::LP => 1,
            REGTYPER::XULP => 2,
            REGTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REGTYPER {
        match value {
            0 => REGTYPER::NORMAL,
            1 => REGTYPER::LP,
            2 => REGTYPER::XULP,
            i => REGTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == REGTYPER::NORMAL
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline]
    pub fn is_lp(&self) -> bool {
        *self == REGTYPER::LP
    }
    #[doc = "Checks if the value of the field is `XULP`"]
    #[inline]
    pub fn is_xulp(&self) -> bool {
        *self == REGTYPER::XULP
    }
}
#[doc = "Possible values of the field `REFTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFTYPER {
    #[doc = "undocumented"]
    BOTH,
    #[doc = "undocumented"]
    BG,
    #[doc = "undocumented"]
    LPBG,
    #[doc = "undocumented"]
    INTERNAL,
}
impl REFTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFTYPER::BOTH => 0,
            REFTYPER::BG => 1,
            REFTYPER::LPBG => 2,
            REFTYPER::INTERNAL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFTYPER {
        match value {
            0 => REFTYPER::BOTH,
            1 => REFTYPER::BG,
            2 => REFTYPER::LPBG,
            3 => REFTYPER::INTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == REFTYPER::BOTH
    }
    #[doc = "Checks if the value of the field is `BG`"]
    #[inline]
    pub fn is_bg(&self) -> bool {
        *self == REFTYPER::BG
    }
    #[doc = "Checks if the value of the field is `LPBG`"]
    #[inline]
    pub fn is_lpbg(&self) -> bool {
        *self == REFTYPER::LPBG
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self == REFTYPER::INTERNAL
    }
}
#[doc = r" Value of the field"]
pub struct FLASHLATDELR {
    bits: u8,
}
impl FLASHLATDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FLASHBIASR {
    bits: u8,
}
impl FLASHBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FPPWR {
    bits: bool,
}
impl FPPWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RC115R {
    bits: u8,
}
impl RC115R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCFASTR {
    bits: u8,
}
impl RCFASTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Regulator Voltage Level"]
    #[inline]
    pub fn reglevel(&self) -> REGLEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGLEVELR { bits }
    }
    #[doc = "Bits 4:5 - Regulator Type"]
    #[inline]
    pub fn regtype(&self) -> REGTYPER {
        REGTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Reference Type"]
    #[inline]
    pub fn reftype(&self) -> REFTYPER {
        REFTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Flash Latch Delay Value"]
    #[inline]
    pub fn flashlatdel(&self) -> FLASHLATDELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLASHLATDELR { bits }
    }
    #[doc = "Bits 13:16 - Flash Bias Value"]
    #[inline]
    pub fn flashbias(&self) -> FLASHBIASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLASHBIASR { bits }
    }
    #[doc = "Bit 17 - Flash Pico Power Mode"]
    #[inline]
    pub fn fppw(&self) -> FPPWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FPPWR { bits }
    }
    #[doc = "Bits 18:24 - RC 115KHZ Calibration Value"]
    #[inline]
    pub fn rc115(&self) -> RC115R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RC115R { bits }
    }
    #[doc = "Bits 25:31 - RCFAST Calibration Value"]
    #[inline]
    pub fn rcfast(&self) -> RCFASTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCFASTR { bits }
    }
}
