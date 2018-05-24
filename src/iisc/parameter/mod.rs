#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAMETER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATR {
    #[doc = "I2S format, stereo with IWS low for left channel"]
    I2S,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FORMATR {
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
            FORMATR::I2S => false,
            FORMATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORMATR {
        match value {
            false => FORMATR::I2S,
            i => FORMATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline]
    pub fn is_i2s(&self) -> bool {
        *self == FORMATR::I2S
    }
}
#[doc = r" Value of the field"]
pub struct NBCHANR {
    bits: u8,
}
impl NBCHANR {
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
    #[doc = "Bit 7 - Data protocol format"]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Maximum number of channels - 1"]
    #[inline]
    pub fn nbchan(&self) -> NBCHANR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBCHANR { bits }
    }
}
