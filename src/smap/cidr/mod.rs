#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VERSIONR {
    bits: u8,
}
impl VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPROCR {
    bits: u8,
}
impl EPROCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NVPSIZR {
    bits: u8,
}
impl NVPSIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NVPSIZ2R {
    bits: u8,
}
impl NVPSIZ2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SRAMSIZR {
    bits: u8,
}
impl SRAMSIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ARCHR {
    bits: u8,
}
impl ARCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NVPTYPR {
    bits: u8,
}
impl NVPTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTR {
    bits: bool,
}
impl EXTR {
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
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline]
    pub fn version(&self) -> VERSIONR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERSIONR { bits }
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline]
    pub fn eproc(&self) -> EPROCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPROCR { bits }
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline]
    pub fn nvpsiz(&self) -> NVPSIZR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NVPSIZR { bits }
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline]
    pub fn nvpsiz2(&self) -> NVPSIZ2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NVPSIZ2R { bits }
    }
    #[doc = "Bits 16:20 - Internal SRAM Size"]
    #[inline]
    pub fn sramsiz(&self) -> SRAMSIZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRAMSIZR { bits }
    }
    #[doc = "Bits 21:27 - Architecture Identifier"]
    #[inline]
    pub fn arch(&self) -> ARCHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ARCHR { bits }
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline]
    pub fn nvptyp(&self) -> NVPTYPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NVPTYPR { bits }
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline]
    pub fn ext(&self) -> EXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXTR { bits }
    }
}
