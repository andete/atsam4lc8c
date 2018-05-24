#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OVIMR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `OVIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVIMR {
    #[doc = "Overrun Interrupt for Channel j Disabled"]
    _0,
    #[doc = "Overrun Interrupt for Channel j Enabled"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl OVIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            OVIMR::_0 => 0,
            OVIMR::_1 => 1,
            OVIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> OVIMR {
        match value {
            0 => OVIMR::_0,
            1 => OVIMR::_1,
            i => OVIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVIMR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Overrun Interrupt Mask"]
    #[inline]
    pub fn ovim(&self) -> OVIMR {
        OVIMR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
