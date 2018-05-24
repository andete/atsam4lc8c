#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `COVFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVFSR {
    #[doc = "No counter overflow has occurred since the last read of the Status Register."]
    _0,
    #[doc = "A counter overflow has occurred since the last read of the Status Register."]
    _1,
}
impl COVFSR {
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
            COVFSR::_0 => false,
            COVFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COVFSR {
        match value {
            false => COVFSR::_0,
            true => COVFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COVFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COVFSR::_1
    }
}
#[doc = "Possible values of the field `LOVRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOVRSR {
    #[doc = "Load overrun has not occurred since the last read of the Status Register or WAVE:1."]
    _0,
    #[doc = "RA or RB have been loaded at least twice without any read of the corresponding register since the last read of the StatusRegister, if WAVE:0."]
    _1,
}
impl LOVRSR {
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
            LOVRSR::_0 => false,
            LOVRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOVRSR {
        match value {
            false => LOVRSR::_0,
            true => LOVRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOVRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOVRSR::_1
    }
}
#[doc = "Possible values of the field `CPAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPASR {
    #[doc = "RA Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0,
    #[doc = "RA Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1,
}
impl CPASR {
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
            CPASR::_0 => false,
            CPASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPASR {
        match value {
            false => CPASR::_0,
            true => CPASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPASR::_1
    }
}
#[doc = "Possible values of the field `CPBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPBSR {
    #[doc = "RB Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0,
    #[doc = "RB Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1,
}
impl CPBSR {
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
            CPBSR::_0 => false,
            CPBSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPBSR {
        match value {
            false => CPBSR::_0,
            true => CPBSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPBSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPBSR::_1
    }
}
#[doc = "Possible values of the field `CPCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCSR {
    #[doc = "RC Compare has not occurred since the last read of the Status Register."]
    _0,
    #[doc = "RC Compare has occurred since the last read of the Status Register."]
    _1,
}
impl CPCSR {
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
            CPCSR::_0 => false,
            CPCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPCSR {
        match value {
            false => CPCSR::_0,
            true => CPCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPCSR::_1
    }
}
#[doc = "Possible values of the field `LDRAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRASR {
    #[doc = "RA Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0,
    #[doc = "RA Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1,
}
impl LDRASR {
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
            LDRASR::_0 => false,
            LDRASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDRASR {
        match value {
            false => LDRASR::_0,
            true => LDRASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LDRASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LDRASR::_1
    }
}
#[doc = "Possible values of the field `LDRBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBSR {
    #[doc = "RB Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0,
    #[doc = "RB Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1,
}
impl LDRBSR {
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
            LDRBSR::_0 => false,
            LDRBSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDRBSR {
        match value {
            false => LDRBSR::_0,
            true => LDRBSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LDRBSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LDRBSR::_1
    }
}
#[doc = "Possible values of the field `ETRGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGSR {
    #[doc = "External trigger has not occurred since the last read of the Status Register."]
    _0,
    #[doc = "External trigger has occurred since the last read of the Status Register."]
    _1,
}
impl ETRGSR {
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
            ETRGSR::_0 => false,
            ETRGSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETRGSR {
        match value {
            false => ETRGSR::_0,
            true => ETRGSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ETRGSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ETRGSR::_1
    }
}
#[doc = "Possible values of the field `CLKSTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSTAR {
    #[doc = "Clock is disabled."]
    _0,
    #[doc = "Clock is enabled."]
    _1,
}
impl CLKSTAR {
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
            CLKSTAR::_0 => false,
            CLKSTAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSTAR {
        match value {
            false => CLKSTAR::_0,
            true => CLKSTAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKSTAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKSTAR::_1
    }
}
#[doc = "Possible values of the field `MTIOA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTIOAR {
    #[doc = "TIOA is low. If WAVE:0, this means that TIOA pin is low. If WAVE:1, this means that TIOA is driven low."]
    _0,
    #[doc = "TIOA is high. If WAVE:0, this means that TIOA pin is high. If WAVE:1, this means that TIOA is driven high."]
    _1,
}
impl MTIOAR {
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
            MTIOAR::_0 => false,
            MTIOAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTIOAR {
        match value {
            false => MTIOAR::_0,
            true => MTIOAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTIOAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTIOAR::_1
    }
}
#[doc = "Possible values of the field `MTIOB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTIOBR {
    #[doc = "TIOB is low. If WAVE:0, this means that TIOB pin is low. If WAVE:1, this means that TIOB is driven low."]
    _0,
    #[doc = "TIOB is high. If WAVE:0, this means that TIOB pin is high. If WAVE:1, this means that TIOB is driven high."]
    _1,
}
impl MTIOBR {
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
            MTIOBR::_0 => false,
            MTIOBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTIOBR {
        match value {
            false => MTIOBR::_0,
            true => MTIOBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTIOBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTIOBR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter Overflow Status"]
    #[inline]
    pub fn covfs(&self) -> COVFSR {
        COVFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Load Overrun Status"]
    #[inline]
    pub fn lovrs(&self) -> LOVRSR {
        LOVRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RA Compare Status"]
    #[inline]
    pub fn cpas(&self) -> CPASR {
        CPASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RB Compare Status"]
    #[inline]
    pub fn cpbs(&self) -> CPBSR {
        CPBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RC Compare Status"]
    #[inline]
    pub fn cpcs(&self) -> CPCSR {
        CPCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RA Loading Status"]
    #[inline]
    pub fn ldras(&self) -> LDRASR {
        LDRASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RB Loading Status"]
    #[inline]
    pub fn ldrbs(&self) -> LDRBSR {
        LDRBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - External Trigger Status"]
    #[inline]
    pub fn etrgs(&self) -> ETRGSR {
        ETRGSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline]
    pub fn clksta(&self) -> CLKSTAR {
        CLKSTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIOA Mirror"]
    #[inline]
    pub fn mtioa(&self) -> MTIOAR {
        MTIOAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIOB Mirror"]
    #[inline]
    pub fn mtiob(&self) -> MTIOBR {
        MTIOBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
