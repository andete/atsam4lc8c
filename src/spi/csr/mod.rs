#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "The inactive state value of SPCK is logic level zero."]
    _0,
    #[doc = "The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    _1,
}
impl CPOLR {
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
            CPOLR::_0 => false,
            CPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::_0,
            true => CPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPOLR::_1
    }
}
#[doc = "Possible values of the field `NCPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCPHAR {
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    _0,
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    _1,
}
impl NCPHAR {
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
            NCPHAR::_0 => false,
            NCPHAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NCPHAR {
        match value {
            false => NCPHAR::_0,
            true => NCPHAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NCPHAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NCPHAR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CSNAATR {
    bits: bool,
}
impl CSNAATR {
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
#[doc = "Possible values of the field `CSAAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSAATR {
    #[doc = "The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    _0,
    #[doc = "The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    _1,
}
impl CSAATR {
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
            CSAATR::_0 => false,
            CSAATR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSAATR {
        match value {
            false => CSAATR::_0,
            true => CSAATR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSAATR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSAATR::_1
    }
}
#[doc = "Possible values of the field `BITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITSR {
    #[doc = "8 bits per transfer"]
    _8_BPT,
    #[doc = "9 bits per transfer"]
    _9_BPT,
    #[doc = "10 bits per transfer"]
    _10_BPT,
    #[doc = "11 bits per transfer"]
    _11_BPT,
    #[doc = "12 bits per transfer"]
    _12_BPT,
    #[doc = "13 bits per transfer"]
    _13_BPT,
    #[doc = "14 bits per transfer"]
    _14_BPT,
    #[doc = "15 bits per transfer"]
    _15_BPT,
    #[doc = "16 bits per transfer"]
    _16_BPT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITSR::_8_BPT => 0,
            BITSR::_9_BPT => 1,
            BITSR::_10_BPT => 2,
            BITSR::_11_BPT => 3,
            BITSR::_12_BPT => 4,
            BITSR::_13_BPT => 5,
            BITSR::_14_BPT => 6,
            BITSR::_15_BPT => 7,
            BITSR::_16_BPT => 8,
            BITSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITSR {
        match value {
            0 => BITSR::_8_BPT,
            1 => BITSR::_9_BPT,
            2 => BITSR::_10_BPT,
            3 => BITSR::_11_BPT,
            4 => BITSR::_12_BPT,
            5 => BITSR::_13_BPT,
            6 => BITSR::_14_BPT,
            7 => BITSR::_15_BPT,
            8 => BITSR::_16_BPT,
            i => BITSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BPT`"]
    #[inline]
    pub fn is_8_bpt(&self) -> bool {
        *self == BITSR::_8_BPT
    }
    #[doc = "Checks if the value of the field is `_9_BPT`"]
    #[inline]
    pub fn is_9_bpt(&self) -> bool {
        *self == BITSR::_9_BPT
    }
    #[doc = "Checks if the value of the field is `_10_BPT`"]
    #[inline]
    pub fn is_10_bpt(&self) -> bool {
        *self == BITSR::_10_BPT
    }
    #[doc = "Checks if the value of the field is `_11_BPT`"]
    #[inline]
    pub fn is_11_bpt(&self) -> bool {
        *self == BITSR::_11_BPT
    }
    #[doc = "Checks if the value of the field is `_12_BPT`"]
    #[inline]
    pub fn is_12_bpt(&self) -> bool {
        *self == BITSR::_12_BPT
    }
    #[doc = "Checks if the value of the field is `_13_BPT`"]
    #[inline]
    pub fn is_13_bpt(&self) -> bool {
        *self == BITSR::_13_BPT
    }
    #[doc = "Checks if the value of the field is `_14_BPT`"]
    #[inline]
    pub fn is_14_bpt(&self) -> bool {
        *self == BITSR::_14_BPT
    }
    #[doc = "Checks if the value of the field is `_15_BPT`"]
    #[inline]
    pub fn is_15_bpt(&self) -> bool {
        *self == BITSR::_15_BPT
    }
    #[doc = "Checks if the value of the field is `_16_BPT`"]
    #[inline]
    pub fn is_16_bpt(&self) -> bool {
        *self == BITSR::_16_BPT
    }
}
#[doc = r" Value of the field"]
pub struct SCBRR {
    bits: u8,
}
impl SCBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYBSR {
    bits: u8,
}
impl DLYBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYBCTR {
    bits: u8,
}
impl DLYBCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "The inactive state value of SPCK is logic level zero."]
    _0,
    #[doc = "The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    _1,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::_0 => false,
            CPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The inactive state value of SPCK is logic level zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOLW::_0)
    }
    #[doc = "The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NCPHA`"]
pub enum NCPHAW {
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    _0,
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    _1,
}
impl NCPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NCPHAW::_0 => false,
            NCPHAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _NCPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCPHAW::_0)
    }
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCPHAW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSNAATW<'a> {
    w: &'a mut W,
}
impl<'a> _CSNAATW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSAAT`"]
pub enum CSAATW {
    #[doc = "The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    _0,
    #[doc = "The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    _1,
}
impl CSAATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSAATW::_0 => false,
            CSAATW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSAATW<'a> {
    w: &'a mut W,
}
impl<'a> _CSAATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSAATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSAATW::_0)
    }
    #[doc = "The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSAATW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BITS`"]
pub enum BITSW {
    #[doc = "8 bits per transfer"]
    _8_BPT,
    #[doc = "9 bits per transfer"]
    _9_BPT,
    #[doc = "10 bits per transfer"]
    _10_BPT,
    #[doc = "11 bits per transfer"]
    _11_BPT,
    #[doc = "12 bits per transfer"]
    _12_BPT,
    #[doc = "13 bits per transfer"]
    _13_BPT,
    #[doc = "14 bits per transfer"]
    _14_BPT,
    #[doc = "15 bits per transfer"]
    _15_BPT,
    #[doc = "16 bits per transfer"]
    _16_BPT,
}
impl BITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITSW::_8_BPT => 0,
            BITSW::_9_BPT => 1,
            BITSW::_10_BPT => 2,
            BITSW::_11_BPT => 3,
            BITSW::_12_BPT => 4,
            BITSW::_13_BPT => 5,
            BITSW::_14_BPT => 6,
            BITSW::_15_BPT => 7,
            BITSW::_16_BPT => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITSW<'a> {
    w: &'a mut W,
}
impl<'a> _BITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits per transfer"]
    #[inline]
    pub fn _8_bpt(self) -> &'a mut W {
        self.variant(BITSW::_8_BPT)
    }
    #[doc = "9 bits per transfer"]
    #[inline]
    pub fn _9_bpt(self) -> &'a mut W {
        self.variant(BITSW::_9_BPT)
    }
    #[doc = "10 bits per transfer"]
    #[inline]
    pub fn _10_bpt(self) -> &'a mut W {
        self.variant(BITSW::_10_BPT)
    }
    #[doc = "11 bits per transfer"]
    #[inline]
    pub fn _11_bpt(self) -> &'a mut W {
        self.variant(BITSW::_11_BPT)
    }
    #[doc = "12 bits per transfer"]
    #[inline]
    pub fn _12_bpt(self) -> &'a mut W {
        self.variant(BITSW::_12_BPT)
    }
    #[doc = "13 bits per transfer"]
    #[inline]
    pub fn _13_bpt(self) -> &'a mut W {
        self.variant(BITSW::_13_BPT)
    }
    #[doc = "14 bits per transfer"]
    #[inline]
    pub fn _14_bpt(self) -> &'a mut W {
        self.variant(BITSW::_14_BPT)
    }
    #[doc = "15 bits per transfer"]
    #[inline]
    pub fn _15_bpt(self) -> &'a mut W {
        self.variant(BITSW::_15_BPT)
    }
    #[doc = "16 bits per transfer"]
    #[inline]
    pub fn _16_bpt(self) -> &'a mut W {
        self.variant(BITSW::_16_BPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCBRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCBRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBCTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline]
    pub fn ncpha(&self) -> NCPHAR {
        NCPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer"]
    #[inline]
    pub fn csnaat(&self) -> CSNAATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSNAATR { bits }
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline]
    pub fn csaat(&self) -> CSAATR {
        CSAATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline]
    pub fn bits_(&self) -> BITSR {
        BITSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline]
    pub fn scbr(&self) -> SCBRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCBRR { bits }
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline]
    pub fn dlybs(&self) -> DLYBSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBSR { bits }
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&self) -> DLYBCTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBCTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline]
    pub fn ncpha(&mut self) -> _NCPHAW {
        _NCPHAW { w: self }
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer"]
    #[inline]
    pub fn csnaat(&mut self) -> _CSNAATW {
        _CSNAATW { w: self }
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline]
    pub fn csaat(&mut self) -> _CSAATW {
        _CSAATW { w: self }
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline]
    pub fn bits_(&mut self) -> _BITSW {
        _BITSW { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline]
    pub fn scbr(&mut self) -> _SCBRW {
        _SCBRW { w: self }
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline]
    pub fn dlybs(&mut self) -> _DLYBSW {
        _DLYBSW { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&mut self) -> _DLYBCTW {
        _DLYBCTW { w: self }
    }
}
