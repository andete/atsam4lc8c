#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::UESTA1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXINIR {
    bits: bool,
}
impl TXINIR {
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
pub struct RXOUTIR {
    bits: bool,
}
impl RXOUTIR {
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
pub struct RXSTPIR {
    bits: bool,
}
impl RXSTPIR {
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
pub struct NAKOUTIR {
    bits: bool,
}
impl NAKOUTIR {
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
pub struct NAKINIR {
    bits: bool,
}
impl NAKINIR {
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
pub struct STALLEDIR {
    bits: bool,
}
impl STALLEDIR {
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
pub struct DTSEQR {
    bits: u8,
}
impl DTSEQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RAMACERIR {
    bits: bool,
}
impl RAMACERIR {
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
pub struct NBUSYBKR {
    bits: u8,
}
impl NBUSYBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CURRBKR {
    bits: u8,
}
impl CURRBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CTRLDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLDIRR {
    #[doc = "undocumented"]
    OUT,
    #[doc = "undocumented"]
    IN,
}
impl CTRLDIRR {
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
            CTRLDIRR::OUT => false,
            CTRLDIRR::IN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRLDIRR {
        match value {
            false => CTRLDIRR::OUT,
            true => CTRLDIRR::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == CTRLDIRR::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == CTRLDIRR::IN
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline]
    pub fn txini(&self) -> TXINIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXINIR { bits }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline]
    pub fn rxouti(&self) -> RXOUTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXOUTIR { bits }
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline]
    pub fn rxstpi(&self) -> RXSTPIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXSTPIR { bits }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline]
    pub fn nakouti(&self) -> NAKOUTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKOUTIR { bits }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline]
    pub fn nakini(&self) -> NAKINIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKINIR { bits }
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline]
    pub fn stalledi(&self) -> STALLEDIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STALLEDIR { bits }
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline]
    pub fn dtseq(&self) -> DTSEQR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTSEQR { bits }
    }
    #[doc = "Bit 11 - Ram Access Error Interrupt"]
    #[inline]
    pub fn ramaceri(&self) -> RAMACERIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAMACERIR { bits }
    }
    #[doc = "Bits 12:13 - Number Of Busy Banks"]
    #[inline]
    pub fn nbusybk(&self) -> NBUSYBKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBUSYBKR { bits }
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline]
    pub fn currbk(&self) -> CURRBKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CURRBKR { bits }
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline]
    pub fn ctrldir(&self) -> CTRLDIRR {
        CTRLDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
