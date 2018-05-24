#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FWPSAVEPS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct WREGLEVELR {
    bits: u8,
}
impl WREGLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WBIASR {
    bits: u8,
}
impl WBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WLATDELR {
    bits: u8,
}
impl WLATDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RREGLEVELR {
    bits: u8,
}
impl RREGLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RBIASR {
    bits: u8,
}
impl RBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RLATDELR {
    bits: u8,
}
impl RLATDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BREGLEVELR {
    bits: u8,
}
impl BREGLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct POR18DISR {
    bits: bool,
}
impl POR18DISR {
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
pub struct FWSASR {
    bits: bool,
}
impl FWSASR {
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
    #[doc = "Bits 0:3 - Wait mode Regulator Level"]
    #[inline]
    pub fn wreglevel(&self) -> WREGLEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WREGLEVELR { bits }
    }
    #[doc = "Bits 4:7 - Bias in wait mode"]
    #[inline]
    pub fn wbias(&self) -> WBIASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WBIASR { bits }
    }
    #[doc = "Bits 8:12 - Flash Latdel in wait mode"]
    #[inline]
    pub fn wlatdel(&self) -> WLATDELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WLATDELR { bits }
    }
    #[doc = "Bits 13:16 - Retention mode Regulator Level"]
    #[inline]
    pub fn rreglevel(&self) -> RREGLEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RREGLEVELR { bits }
    }
    #[doc = "Bits 17:20 - Bias in Retention mode"]
    #[inline]
    pub fn rbias(&self) -> RBIASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RBIASR { bits }
    }
    #[doc = "Bits 21:25 - Flash Latdel in Retention mode"]
    #[inline]
    pub fn rlatdel(&self) -> RLATDELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RLATDELR { bits }
    }
    #[doc = "Bits 26:29 - Backup mode Regulator Level"]
    #[inline]
    pub fn breglevel(&self) -> BREGLEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BREGLEVELR { bits }
    }
    #[doc = "Bit 30 - POR 18 Disable"]
    #[inline]
    pub fn por18dis(&self) -> POR18DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POR18DISR { bits }
    }
    #[doc = "Bit 31 - Flash Wait State Automatic Switching"]
    #[inline]
    pub fn fwsas(&self) -> FWSASR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FWSASR { bits }
    }
}
