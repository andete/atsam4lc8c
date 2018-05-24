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
#[doc = "Possible values of the field `DT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR {
    #[doc = "Digital tuner off"]
    OFF,
    #[doc = "Digital tuner on"]
    ON,
}
impl DTR {
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
            DTR::OFF => false,
            DTR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTR {
        match value {
            false => DTR::OFF,
            true => DTR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == DTR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == DTR::ON
    }
}
#[doc = "Possible values of the field `DTEXPWA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEXPWAR {
    #[doc = "Digital tuner exponent is a constant value. Writes to EXP bitfield in DTR will be discarded."]
    _0,
    #[doc = "Digital tuner exponent is chosen by writing to EXP bitfield in DTR"]
    _1,
}
impl DTEXPWAR {
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
            DTEXPWAR::_0 => false,
            DTEXPWAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTEXPWAR {
        match value {
            false => DTEXPWAR::_0,
            true => DTEXPWAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTEXPWAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTEXPWAR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DTEXPVALUER {
    bits: u8,
}
impl DTEXPVALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `NUMAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUMARR {
    #[doc = "No alarm comparators"]
    ZERO,
    #[doc = "One alarm comparator"]
    ONE,
    #[doc = "Two alarm comparators"]
    TWO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NUMARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NUMARR::ZERO => 0,
            NUMARR::ONE => 1,
            NUMARR::TWO => 2,
            NUMARR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NUMARR {
        match value {
            0 => NUMARR::ZERO,
            1 => NUMARR::ONE,
            2 => NUMARR::TWO,
            i => NUMARR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == NUMARR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == NUMARR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == NUMARR::TWO
    }
}
#[doc = "Possible values of the field `NUMPIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUMPIRR {
    #[doc = "One periodic comparator"]
    ONE,
    #[doc = "Two periodic comparators"]
    TWO,
}
impl NUMPIRR {
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
            NUMPIRR::ONE => false,
            NUMPIRR::TWO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NUMPIRR {
        match value {
            false => NUMPIRR::ONE,
            true => NUMPIRR::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == NUMPIRR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == NUMPIRR::TWO
    }
}
#[doc = "Possible values of the field `PIR0WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR0WAR {
    #[doc = "Periodic alarm prescaler 0 tapping is a constant value. Writes to INSEL bitfield in PIR0 will be discarded."]
    _0,
    #[doc = "Periodic alarm prescaler 0 tapping is chosen by writing to INSEL bitfield in PIR0"]
    _1,
}
impl PIR0WAR {
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
            PIR0WAR::_0 => false,
            PIR0WAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIR0WAR {
        match value {
            false => PIR0WAR::_0,
            true => PIR0WAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PIR0WAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PIR0WAR::_1
    }
}
#[doc = "Possible values of the field `PIR1WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR1WAR {
    #[doc = "Writes to PIR1 will be discarded"]
    _0,
    #[doc = "PIR1 can be written"]
    _1,
}
impl PIR1WAR {
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
            PIR1WAR::_0 => false,
            PIR1WAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIR1WAR {
        match value {
            false => PIR1WAR::_0,
            true => PIR1WAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PIR1WAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PIR1WAR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PER0VALUER {
    bits: u8,
}
impl PER0VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER1VALUER {
    bits: u8,
}
impl PER1VALUER {
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
    #[doc = "Bit 0 - Digital Tuner"]
    #[inline]
    pub fn dt(&self) -> DTR {
        DTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Digital Tuner Exponent Writeable"]
    #[inline]
    pub fn dtexpwa(&self) -> DTEXPWAR {
        DTEXPWAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:6 - Digital Tuner Exponent Value"]
    #[inline]
    pub fn dtexpvalue(&self) -> DTEXPVALUER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTEXPVALUER { bits }
    }
    #[doc = "Bits 8:9 - Number of alarm comparators"]
    #[inline]
    pub fn numar(&self) -> NUMARR {
        NUMARR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Number of periodic comparators"]
    #[inline]
    pub fn numpir(&self) -> NUMPIRR {
        NUMPIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Periodic Interval 0 Writeable"]
    #[inline]
    pub fn pir0wa(&self) -> PIR0WAR {
        PIR0WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Periodic Interval 1 Writeable"]
    #[inline]
    pub fn pir1wa(&self) -> PIR1WAR {
        PIR1WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Periodic Interval 0 Value"]
    #[inline]
    pub fn per0value(&self) -> PER0VALUER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER0VALUER { bits }
    }
    #[doc = "Bits 24:28 - Periodic Interval 1 Value"]
    #[inline]
    pub fn per1value(&self) -> PER1VALUER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER1VALUER { bits }
    }
}
