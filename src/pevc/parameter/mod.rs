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
#[doc = r" Value of the field"]
pub struct IGF_COUNTR {
    bits: u8,
}
impl IGF_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EVS_COUNTR {
    bits: u8,
}
impl EVS_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EVINR {
    bits: u8,
}
impl EVINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIGOUTR {
    bits: u8,
}
impl TRIGOUTR {
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
    #[doc = "Bits 0:7 - Number of Input Glitch Filters"]
    #[inline]
    pub fn igf_count(&self) -> IGF_COUNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IGF_COUNTR { bits }
    }
    #[doc = "Bits 8:15 - Number of Event Shapers"]
    #[inline]
    pub fn evs_count(&self) -> EVS_COUNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EVS_COUNTR { bits }
    }
    #[doc = "Bits 16:23 - Number of Event Inputs / Generators"]
    #[inline]
    pub fn evin(&self) -> EVINR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EVINR { bits }
    }
    #[doc = "Bits 24:31 - Number of Trigger Outputs / Channels / Users"]
    #[inline]
    pub fn trigout(&self) -> TRIGOUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIGOUTR { bits }
    }
}
