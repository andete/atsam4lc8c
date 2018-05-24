#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IMR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RDRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRFR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl RDRFR {
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
            RDRFR::_0 => false,
            RDRFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDRFR {
        match value {
            false => RDRFR::_0,
            true => RDRFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDRFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDRFR::_1
    }
}
#[doc = "Possible values of the field `TDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRER {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl TDRER {
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
            TDRER::_0 => false,
            TDRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRER {
        match value {
            false => TDRER::_0,
            true => TDRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDRER::_1
    }
}
#[doc = "Possible values of the field `MODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl MODFR {
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
            MODFR::_0 => false,
            MODFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODFR {
        match value {
            false => MODFR::_0,
            true => MODFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MODFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MODFR::_1
    }
}
#[doc = "Possible values of the field `OVRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRESR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl OVRESR {
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
            OVRESR::_0 => false,
            OVRESR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRESR {
        match value {
            false => OVRESR::_0,
            true => OVRESR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVRESR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVRESR::_1
    }
}
#[doc = "Possible values of the field `ENDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRXR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl ENDRXR {
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
            ENDRXR::_0 => false,
            ENDRXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDRXR {
        match value {
            false => ENDRXR::_0,
            true => ENDRXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENDRXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENDRXR::_1
    }
}
#[doc = "Possible values of the field `ENDTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTXR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl ENDTXR {
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
            ENDTXR::_0 => false,
            ENDTXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDTXR {
        match value {
            false => ENDTXR::_0,
            true => ENDTXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENDTXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENDTXR::_1
    }
}
#[doc = "Possible values of the field `RXBUFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFFR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl RXBUFFR {
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
            RXBUFFR::_0 => false,
            RXBUFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXBUFFR {
        match value {
            false => RXBUFFR::_0,
            true => RXBUFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXBUFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXBUFFR::_1
    }
}
#[doc = "Possible values of the field `TXBUFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFER {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl TXBUFER {
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
            TXBUFER::_0 => false,
            TXBUFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXBUFER {
        match value {
            false => TXBUFER::_0,
            true => TXBUFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXBUFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXBUFER::_1
    }
}
#[doc = "Possible values of the field `NSSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSRR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl NSSRR {
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
            NSSRR::_0 => false,
            NSSRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSSRR {
        match value {
            false => NSSRR::_0,
            true => NSSRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NSSRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NSSRR::_1
    }
}
#[doc = "Possible values of the field `TXEMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTYR {
    #[doc = "The corresponding interrupt is not enabled."]
    _0,
    #[doc = "The corresponding interrupt is enabled."]
    _1,
}
impl TXEMPTYR {
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
            TXEMPTYR::_0 => false,
            TXEMPTYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEMPTYR {
        match value {
            false => TXEMPTYR::_0,
            true => TXEMPTYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTYR::_1
    }
}
#[doc = r" Value of the field"]
pub struct UNDESR {
    bits: bool,
}
impl UNDESR {
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
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline]
    pub fn rdrf(&self) -> RDRFR {
        RDRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Mask"]
    #[inline]
    pub fn tdre(&self) -> TDRER {
        TDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline]
    pub fn modf(&self) -> MODFR {
        MODFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline]
    pub fn ovres(&self) -> OVRESR {
        OVRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Mask"]
    #[inline]
    pub fn endrx(&self) -> ENDRXR {
        ENDRXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Mask"]
    #[inline]
    pub fn endtx(&self) -> ENDTXR {
        ENDTXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Mask"]
    #[inline]
    pub fn rxbuff(&self) -> RXBUFFR {
        RXBUFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Mask"]
    #[inline]
    pub fn txbufe(&self) -> TXBUFER {
        TXBUFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline]
    pub fn nssr(&self) -> NSSRR {
        NSSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline]
    pub fn txempty(&self) -> TXEMPTYR {
        TXEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline]
    pub fn undes(&self) -> UNDESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNDESR { bits }
    }
}
