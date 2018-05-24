#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct APIDVR {
    bits: u8,
}
impl APIDVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct APIDR {
    bits: u8,
}
impl APIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLSSR {
    bits: bool,
}
impl CLSSR {
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
pub struct ICR {
    bits: u8,
}
impl ICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCR {
    bits: u8,
}
impl CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVISIONR {
    bits: u8,
}
impl REVISIONR {
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
    #[doc = "Bits 0:3 - AP Identification Variant"]
    #[inline]
    pub fn apidv(&self) -> APIDVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APIDVR { bits }
    }
    #[doc = "Bits 4:7 - AP Identification"]
    #[inline]
    pub fn apid(&self) -> APIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APIDR { bits }
    }
    #[doc = "Bit 16 - Class"]
    #[inline]
    pub fn clss(&self) -> CLSSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLSSR { bits }
    }
    #[doc = "Bits 17:23 - JEP-106 Identity Code"]
    #[inline]
    pub fn ic(&self) -> ICR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICR { bits }
    }
    #[doc = "Bits 24:27 - JEP-106 Continuation Code"]
    #[inline]
    pub fn cc(&self) -> CCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCR { bits }
    }
    #[doc = "Bits 28:31 - Revision"]
    #[inline]
    pub fn revision(&self) -> REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVISIONR { bits }
    }
}
