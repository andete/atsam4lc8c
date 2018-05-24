#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRIMR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMR {
    #[doc = "Trigger j Interrupt Disabled"]
    _0,
    #[doc = "Trigger j Interrupt Enabled"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            TRIMR::_0 => 0,
            TRIMR::_1 => 1,
            TRIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> TRIMR {
        match value {
            0 => TRIMR::_0,
            1 => TRIMR::_1,
            i => TRIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIMR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Trigger Interrupt Mask"]
    #[inline]
    pub fn trim(&self) -> TRIMR {
        TRIMR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
