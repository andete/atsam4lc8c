#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBSTA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VBUSRQR {
    bits: bool,
}
impl VBUSRQR {
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
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "undocumented"]
    FULL,
    #[doc = "undocumented"]
    HIGH,
    #[doc = "undocumented"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::FULL => 0,
            SPEEDR::HIGH => 1,
            SPEEDR::LOW => 2,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::FULL,
            1 => SPEEDR::HIGH,
            2 => SPEEDR::LOW,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == SPEEDR::FULL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SPEEDR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SPEEDR::LOW
    }
}
#[doc = r" Value of the field"]
pub struct CLKUSABLER {
    bits: bool,
}
impl CLKUSABLER {
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
pub struct SUSPENDR {
    bits: bool,
}
impl SUSPENDR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 9 - VBus Request"]
    #[inline]
    pub fn vbusrq(&self) -> VBUSRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSRQR { bits }
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - USB Clock Usable"]
    #[inline]
    pub fn clkusable(&self) -> CLKUSABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKUSABLER { bits }
    }
    #[doc = "Bit 16 - Suspend module state"]
    #[inline]
    pub fn suspend(&self) -> SUSPENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSPENDR { bits }
    }
}
