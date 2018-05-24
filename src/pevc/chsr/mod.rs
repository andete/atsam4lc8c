#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CHSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSR {
    #[doc = "Channel j Disabled"]
    _0,
    #[doc = "Channel j Enabled"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CHSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CHSR::_0 => 0,
            CHSR::_1 => 1,
            CHSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CHSR {
        match value {
            0 => CHSR::_0,
            1 => CHSR::_1,
            i => CHSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHSR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Channel Status"]
    #[inline]
    pub fn chs(&self) -> CHSR {
        CHSR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
