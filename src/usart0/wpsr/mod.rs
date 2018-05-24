#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WPSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `WPV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPVR {
    #[doc = "No Write Protect Violation has occurred since the last read of the WPSR register"]
    _0,
    #[doc = "A Write Protect Violation has occurred since the last read of the WPSR register. If this violation is an unauthorized attempt to write a protected register, the associated violation is reported into field WPVSRC"]
    _1,
}
impl WPVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WPVR::_0 => false,
            WPVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPVR {
        match value {
            false => WPVR::_0,
            true => WPVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPVR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WPVSRCR {
    bits: u16,
}
impl WPVSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline]
    pub fn wpv(&self) -> WPVR {
        WPVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:23 - Write Protect Violation Source"]
    #[inline]
    pub fn wpvsrc(&self) -> WPVSRCR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WPVSRCR { bits }
    }
}
