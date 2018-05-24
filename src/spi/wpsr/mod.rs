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
#[doc = "Possible values of the field `WPVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPVSR {
    #[doc = "The Write Protection has blocked a Write access to a protected register (since the last read)."]
    WRITE_WITH_WP,
    #[doc = "Software Reset has been performed while Write Protection was enabled (since the last read or since the last write access on MR, IER, IDR or CSRx)."]
    SWRST_WITH_WP,
    #[doc = "Write accesses have been detected on MR (while a chip select was active) or on CSRi (while the Chip Select \"i\" was active) since the last read."]
    UNEXPECTED_WRITE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WPVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WPVSR::WRITE_WITH_WP => 1,
            WPVSR::SWRST_WITH_WP => 2,
            WPVSR::UNEXPECTED_WRITE => 4,
            WPVSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WPVSR {
        match value {
            1 => WPVSR::WRITE_WITH_WP,
            2 => WPVSR::SWRST_WITH_WP,
            4 => WPVSR::UNEXPECTED_WRITE,
            i => WPVSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_WITH_WP`"]
    #[inline]
    pub fn is_write_with_wp(&self) -> bool {
        *self == WPVSR::WRITE_WITH_WP
    }
    #[doc = "Checks if the value of the field is `SWRST_WITH_WP`"]
    #[inline]
    pub fn is_swrst_with_wp(&self) -> bool {
        *self == WPVSR::SWRST_WITH_WP
    }
    #[doc = "Checks if the value of the field is `UNEXPECTED_WRITE`"]
    #[inline]
    pub fn is_unexpected_write(&self) -> bool {
        *self == WPVSR::UNEXPECTED_WRITE
    }
}
#[doc = r" Value of the field"]
pub struct WPVSRCR {
    bits: u8,
}
impl WPVSRCR {
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
    #[doc = "Bits 0:2 - Write Protection Violation Status"]
    #[inline]
    pub fn wpvs(&self) -> WPVSR {
        WPVSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Write Protection Violation Source"]
    #[inline]
    pub fn wpvsrc(&self) -> WPVSRCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WPVSRCR { bits }
    }
}
