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
#[doc = "Possible values of the field `COVFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVFSR {
    #[doc = "The Counter Overflow Interrupt is disabled."]
    _0,
    #[doc = "The Counter Overflow Interrupt is enabled."]
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
    #[doc = "The Load Overrun Interrupt is disabled."]
    _0,
    #[doc = "The Load Overrun Interrupt is enabled."]
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
    #[doc = "The RA Compare Interrupt is disabled."]
    _0,
    #[doc = "The RA Compare Interrupt is enabled."]
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
    #[doc = "The RB Compare Interrupt is disabled."]
    _0,
    #[doc = "The RB Compare Interrupt is enabled."]
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
    #[doc = "The RC Compare Interrupt is disabled."]
    _0,
    #[doc = "The RC Compare Interrupt is enabled."]
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
    #[doc = "The Load RA Interrupt is disabled."]
    _0,
    #[doc = "The Load RA Interrupt is enabled."]
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
    #[doc = "The Load RB Interrupt is disabled."]
    _0,
    #[doc = "The Load RB Interrupt is enabled."]
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
    #[doc = "The External Trigger Interrupt is disabled."]
    _0,
    #[doc = "The External Trigger Interrupt is enabled."]
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline]
    pub fn covfs(&self) -> COVFSR {
        COVFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline]
    pub fn lovrs(&self) -> LOVRSR {
        LOVRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline]
    pub fn cpas(&self) -> CPASR {
        CPASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline]
    pub fn cpbs(&self) -> CPBSR {
        CPBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline]
    pub fn cpcs(&self) -> CPCSR {
        CPCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline]
    pub fn ldras(&self) -> LDRASR {
        LDRASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline]
    pub fn ldrbs(&self) -> LDRBSR {
        LDRBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline]
    pub fn etrgs(&self) -> ETRGSR {
        ETRGSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
