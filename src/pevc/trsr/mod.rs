#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRSR {
    #[doc = "Channel j did not send out an Event in the past"]
    _0,
    #[doc = "Channel j did send out an Event in the past"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl TRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            TRSR::_0 => 0,
            TRSR::_1 => 1,
            TRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> TRSR {
        match value {
            0 => TRSR::_0,
            1 => TRSR::_1,
            i => TRSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRSR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Trigger Interrupt Status"]
    #[inline]
    pub fn trs(&self) -> TRSR {
        TRSR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
