#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LEVEL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FLEVELR {
    bits: u16,
}
impl FLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RLEVELR {
    bits: u8,
}
impl RLEVELR {
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
    #[doc = "Bits 0:11 - Fractional Sensor Level"]
    #[inline]
    pub fn flevel(&self) -> FLEVELR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FLEVELR { bits }
    }
    #[doc = "Bits 12:19 - Integer Sensor Level"]
    #[inline]
    pub fn rlevel(&self) -> RLEVELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RLEVELR { bits }
    }
}
