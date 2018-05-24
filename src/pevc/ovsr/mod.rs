#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OVSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `OVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSR {
    #[doc = "No Overrun occured on Channel j"]
    _0,
    #[doc = "Overrun occured on Channel j"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl OVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            OVSR::_0 => 0,
            OVSR::_1 => 1,
            OVSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> OVSR {
        match value {
            0 => OVSR::_0,
            1 => OVSR::_1,
            i => OVSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVSR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Overrun Interrupt Status"]
    #[inline]
    pub fn ovs(&self) -> OVSR {
        OVSR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
