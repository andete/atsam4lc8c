#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Slave mode (only serial data handled, clocks received from external master or controller)"]
    SLAVE,
    #[doc = "Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    MASTER,
}
impl MODER {
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
            MODER::SLAVE => false,
            MODER::MASTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::SLAVE,
            true => MODER::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MODER::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MODER::MASTER
    }
}
#[doc = "Possible values of the field `DATALENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATALENGTHR {
    #[doc = "32 bits"]
    _32,
    #[doc = "24 bits"]
    _24,
    #[doc = "20 bits"]
    _20,
    #[doc = "18 bits"]
    _18,
    #[doc = "16 bits"]
    _16,
    #[doc = "16 bits compact stereo"]
    _16C,
    #[doc = "8 bits"]
    _8,
    #[doc = "8 bits compact stereo"]
    _8C,
}
impl DATALENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATALENGTHR::_32 => 0,
            DATALENGTHR::_24 => 1,
            DATALENGTHR::_20 => 2,
            DATALENGTHR::_18 => 3,
            DATALENGTHR::_16 => 4,
            DATALENGTHR::_16C => 5,
            DATALENGTHR::_8 => 6,
            DATALENGTHR::_8C => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATALENGTHR {
        match value {
            0 => DATALENGTHR::_32,
            1 => DATALENGTHR::_24,
            2 => DATALENGTHR::_20,
            3 => DATALENGTHR::_18,
            4 => DATALENGTHR::_16,
            5 => DATALENGTHR::_16C,
            6 => DATALENGTHR::_8,
            7 => DATALENGTHR::_8C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == DATALENGTHR::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == DATALENGTHR::_24
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == DATALENGTHR::_20
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == DATALENGTHR::_18
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == DATALENGTHR::_16
    }
    #[doc = "Checks if the value of the field is `_16C`"]
    #[inline]
    pub fn is_16c(&self) -> bool {
        *self == DATALENGTHR::_16C
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == DATALENGTHR::_8
    }
    #[doc = "Checks if the value of the field is `_8C`"]
    #[inline]
    pub fn is_8c(&self) -> bool {
        *self == DATALENGTHR::_8C
    }
}
#[doc = "Possible values of the field `RXMONO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMONOR {
    #[doc = "Normal mode"]
    STEREO,
    #[doc = "Left channel data is duplicated to right channel"]
    MONO,
}
impl RXMONOR {
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
            RXMONOR::STEREO => false,
            RXMONOR::MONO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXMONOR {
        match value {
            false => RXMONOR::STEREO,
            true => RXMONOR::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline]
    pub fn is_stereo(&self) -> bool {
        *self == RXMONOR::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline]
    pub fn is_mono(&self) -> bool {
        *self == RXMONOR::MONO
    }
}
#[doc = "Possible values of the field `RXDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAR {
    #[doc = "Single DMA channel"]
    SINGLE,
    #[doc = "One DMA channel per data channel"]
    MULTIPLE,
}
impl RXDMAR {
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
            RXDMAR::SINGLE => false,
            RXDMAR::MULTIPLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAR {
        match value {
            false => RXDMAR::SINGLE,
            true => RXDMAR::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == RXDMAR::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        *self == RXDMAR::MULTIPLE
    }
}
#[doc = "Possible values of the field `RXLOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLOOPR {
    #[doc = "Normal mode"]
    OFF,
    #[doc = "ISDO internally connected to ISDI"]
    ON,
}
impl RXLOOPR {
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
            RXLOOPR::OFF => false,
            RXLOOPR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXLOOPR {
        match value {
            false => RXLOOPR::OFF,
            true => RXLOOPR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RXLOOPR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == RXLOOPR::ON
    }
}
#[doc = "Possible values of the field `TXMONO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMONOR {
    #[doc = "Normal mode"]
    STEREO,
    #[doc = "Left channel data is duplicated to right channel"]
    MONO,
}
impl TXMONOR {
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
            TXMONOR::STEREO => false,
            TXMONOR::MONO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXMONOR {
        match value {
            false => TXMONOR::STEREO,
            true => TXMONOR::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline]
    pub fn is_stereo(&self) -> bool {
        *self == TXMONOR::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline]
    pub fn is_mono(&self) -> bool {
        *self == TXMONOR::MONO
    }
}
#[doc = "Possible values of the field `TXDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAR {
    #[doc = "Single DMA channel"]
    SINGLE,
    #[doc = "One DMA channel per data channel"]
    MULTIPLE,
}
impl TXDMAR {
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
            TXDMAR::SINGLE => false,
            TXDMAR::MULTIPLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAR {
        match value {
            false => TXDMAR::SINGLE,
            true => TXDMAR::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == TXDMAR::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        *self == TXDMAR::MULTIPLE
    }
}
#[doc = "Possible values of the field `TXSAME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSAMER {
    #[doc = "Zero data transmitted in case of underrun"]
    ZERO,
    #[doc = "Last data transmitted in case of underrun"]
    SAME,
}
impl TXSAMER {
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
            TXSAMER::ZERO => false,
            TXSAMER::SAME => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSAMER {
        match value {
            false => TXSAMER::ZERO,
            true => TXSAMER::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == TXSAMER::ZERO
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline]
    pub fn is_same(&self) -> bool {
        *self == TXSAMER::SAME
    }
}
#[doc = "Possible values of the field `IMCKFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMCKFSR {
    #[doc = "16 fs"]
    _16,
    #[doc = "32 fs"]
    _32,
    #[doc = "64 fs"]
    _64,
    #[doc = "128 fs"]
    _128,
    #[doc = "256 fs"]
    _256,
    #[doc = "384 fs"]
    _384,
    #[doc = "512 fs"]
    _512,
    #[doc = "768 fs"]
    _768,
    #[doc = "1024 fs"]
    _1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IMCKFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IMCKFSR::_16 => 0,
            IMCKFSR::_32 => 1,
            IMCKFSR::_64 => 3,
            IMCKFSR::_128 => 7,
            IMCKFSR::_256 => 15,
            IMCKFSR::_384 => 23,
            IMCKFSR::_512 => 31,
            IMCKFSR::_768 => 47,
            IMCKFSR::_1024 => 63,
            IMCKFSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IMCKFSR {
        match value {
            0 => IMCKFSR::_16,
            1 => IMCKFSR::_32,
            3 => IMCKFSR::_64,
            7 => IMCKFSR::_128,
            15 => IMCKFSR::_256,
            23 => IMCKFSR::_384,
            31 => IMCKFSR::_512,
            47 => IMCKFSR::_768,
            63 => IMCKFSR::_1024,
            i => IMCKFSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == IMCKFSR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == IMCKFSR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == IMCKFSR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == IMCKFSR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == IMCKFSR::_256
    }
    #[doc = "Checks if the value of the field is `_384`"]
    #[inline]
    pub fn is_384(&self) -> bool {
        *self == IMCKFSR::_384
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == IMCKFSR::_512
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline]
    pub fn is_768(&self) -> bool {
        *self == IMCKFSR::_768
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == IMCKFSR::_1024
    }
}
#[doc = "Possible values of the field `IMCKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMCKMODER {
    #[doc = "No IMCK generated"]
    NO_IMCK,
    #[doc = "IMCK generated"]
    IMCK,
}
impl IMCKMODER {
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
            IMCKMODER::NO_IMCK => false,
            IMCKMODER::IMCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMCKMODER {
        match value {
            false => IMCKMODER::NO_IMCK,
            true => IMCKMODER::IMCK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMCK`"]
    #[inline]
    pub fn is_no_imck(&self) -> bool {
        *self == IMCKMODER::NO_IMCK
    }
    #[doc = "Checks if the value of the field is `IMCK`"]
    #[inline]
    pub fn is_imck(&self) -> bool {
        *self == IMCKMODER::IMCK
    }
}
#[doc = "Possible values of the field `IWS24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWS24R {
    #[doc = "IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    _32,
    #[doc = "IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    _24,
}
impl IWS24R {
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
            IWS24R::_32 => false,
            IWS24R::_24 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IWS24R {
        match value {
            false => IWS24R::_32,
            true => IWS24R::_24,
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == IWS24R::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == IWS24R::_24
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Slave mode (only serial data handled, clocks received from external master or controller)"]
    SLAVE,
    #[doc = "Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    MASTER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::SLAVE => false,
            MODEW::MASTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode (only serial data handled, clocks received from external master or controller)"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MODEW::SLAVE)
    }
    #[doc = "Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MODEW::MASTER)
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
#[doc = "Values that can be written to the field `DATALENGTH`"]
pub enum DATALENGTHW {
    #[doc = "32 bits"]
    _32,
    #[doc = "24 bits"]
    _24,
    #[doc = "20 bits"]
    _20,
    #[doc = "18 bits"]
    _18,
    #[doc = "16 bits"]
    _16,
    #[doc = "16 bits compact stereo"]
    _16C,
    #[doc = "8 bits"]
    _8,
    #[doc = "8 bits compact stereo"]
    _8C,
}
impl DATALENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATALENGTHW::_32 => 0,
            DATALENGTHW::_24 => 1,
            DATALENGTHW::_20 => 2,
            DATALENGTHW::_18 => 3,
            DATALENGTHW::_16 => 4,
            DATALENGTHW::_16C => 5,
            DATALENGTHW::_8 => 6,
            DATALENGTHW::_8C => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATALENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATALENGTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32 bits"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(DATALENGTHW::_32)
    }
    #[doc = "24 bits"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(DATALENGTHW::_24)
    }
    #[doc = "20 bits"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(DATALENGTHW::_20)
    }
    #[doc = "18 bits"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(DATALENGTHW::_18)
    }
    #[doc = "16 bits"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(DATALENGTHW::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline]
    pub fn _16c(self) -> &'a mut W {
        self.variant(DATALENGTHW::_16C)
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(DATALENGTHW::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline]
    pub fn _8c(self) -> &'a mut W {
        self.variant(DATALENGTHW::_8C)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXMONO`"]
pub enum RXMONOW {
    #[doc = "Normal mode"]
    STEREO,
    #[doc = "Left channel data is duplicated to right channel"]
    MONO,
}
impl RXMONOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXMONOW::STEREO => false,
            RXMONOW::MONO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXMONOW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMONOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXMONOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn stereo(self) -> &'a mut W {
        self.variant(RXMONOW::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline]
    pub fn mono(self) -> &'a mut W {
        self.variant(RXMONOW::MONO)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDMA`"]
pub enum RXDMAW {
    #[doc = "Single DMA channel"]
    SINGLE,
    #[doc = "One DMA channel per data channel"]
    MULTIPLE,
}
impl RXDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAW::SINGLE => false,
            RXDMAW::MULTIPLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single DMA channel"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(RXDMAW::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline]
    pub fn multiple(self) -> &'a mut W {
        self.variant(RXDMAW::MULTIPLE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXLOOP`"]
pub enum RXLOOPW {
    #[doc = "Normal mode"]
    OFF,
    #[doc = "ISDO internally connected to ISDI"]
    ON,
}
impl RXLOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXLOOPW::OFF => false,
            RXLOOPW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXLOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXLOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(RXLOOPW::OFF)
    }
    #[doc = "ISDO internally connected to ISDI"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(RXLOOPW::ON)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXMONO`"]
pub enum TXMONOW {
    #[doc = "Normal mode"]
    STEREO,
    #[doc = "Left channel data is duplicated to right channel"]
    MONO,
}
impl TXMONOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXMONOW::STEREO => false,
            TXMONOW::MONO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXMONOW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMONOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXMONOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn stereo(self) -> &'a mut W {
        self.variant(TXMONOW::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline]
    pub fn mono(self) -> &'a mut W {
        self.variant(TXMONOW::MONO)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXDMA`"]
pub enum TXDMAW {
    #[doc = "Single DMA channel"]
    SINGLE,
    #[doc = "One DMA channel per data channel"]
    MULTIPLE,
}
impl TXDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAW::SINGLE => false,
            TXDMAW::MULTIPLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single DMA channel"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDMAW::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline]
    pub fn multiple(self) -> &'a mut W {
        self.variant(TXDMAW::MULTIPLE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSAME`"]
pub enum TXSAMEW {
    #[doc = "Zero data transmitted in case of underrun"]
    ZERO,
    #[doc = "Last data transmitted in case of underrun"]
    SAME,
}
impl TXSAMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSAMEW::ZERO => false,
            TXSAMEW::SAME => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSAMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSAMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXSAMEW::ZERO)
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline]
    pub fn same(self) -> &'a mut W {
        self.variant(TXSAMEW::SAME)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IMCKFS`"]
pub enum IMCKFSW {
    #[doc = "16 fs"]
    _16,
    #[doc = "32 fs"]
    _32,
    #[doc = "64 fs"]
    _64,
    #[doc = "128 fs"]
    _128,
    #[doc = "256 fs"]
    _256,
    #[doc = "384 fs"]
    _384,
    #[doc = "512 fs"]
    _512,
    #[doc = "768 fs"]
    _768,
    #[doc = "1024 fs"]
    _1024,
}
impl IMCKFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IMCKFSW::_16 => 0,
            IMCKFSW::_32 => 1,
            IMCKFSW::_64 => 3,
            IMCKFSW::_128 => 7,
            IMCKFSW::_256 => 15,
            IMCKFSW::_384 => 23,
            IMCKFSW::_512 => 31,
            IMCKFSW::_768 => 47,
            IMCKFSW::_1024 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMCKFSW<'a> {
    w: &'a mut W,
}
impl<'a> _IMCKFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMCKFSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16 fs"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(IMCKFSW::_16)
    }
    #[doc = "32 fs"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(IMCKFSW::_32)
    }
    #[doc = "64 fs"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(IMCKFSW::_64)
    }
    #[doc = "128 fs"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(IMCKFSW::_128)
    }
    #[doc = "256 fs"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(IMCKFSW::_256)
    }
    #[doc = "384 fs"]
    #[inline]
    pub fn _384(self) -> &'a mut W {
        self.variant(IMCKFSW::_384)
    }
    #[doc = "512 fs"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(IMCKFSW::_512)
    }
    #[doc = "768 fs"]
    #[inline]
    pub fn _768(self) -> &'a mut W {
        self.variant(IMCKFSW::_768)
    }
    #[doc = "1024 fs"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(IMCKFSW::_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IMCKMODE`"]
pub enum IMCKMODEW {
    #[doc = "No IMCK generated"]
    NO_IMCK,
    #[doc = "IMCK generated"]
    IMCK,
}
impl IMCKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IMCKMODEW::NO_IMCK => false,
            IMCKMODEW::IMCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMCKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _IMCKMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMCKMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No IMCK generated"]
    #[inline]
    pub fn no_imck(self) -> &'a mut W {
        self.variant(IMCKMODEW::NO_IMCK)
    }
    #[doc = "IMCK generated"]
    #[inline]
    pub fn imck(self) -> &'a mut W {
        self.variant(IMCKMODEW::IMCK)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IWS24`"]
pub enum IWS24W {
    #[doc = "IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    _32,
    #[doc = "IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    _24,
}
impl IWS24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IWS24W::_32 => false,
            IWS24W::_24 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IWS24W<'a> {
    w: &'a mut W,
}
impl<'a> _IWS24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IWS24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(IWS24W::_32)
    }
    #[doc = "IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(IWS24W::_24)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Master/Slave/Controller Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline]
    pub fn datalength(&self) -> DATALENGTHR {
        DATALENGTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Receiver Mono"]
    #[inline]
    pub fn rxmono(&self) -> RXMONOR {
        RXMONOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Single or Multiple DMA Channels for Receiver"]
    #[inline]
    pub fn rxdma(&self) -> RXDMAR {
        RXDMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Loop-back Test Mode"]
    #[inline]
    pub fn rxloop(&self) -> RXLOOPR {
        RXLOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmitter Mono"]
    #[inline]
    pub fn txmono(&self) -> TXMONOR {
        TXMONOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Single or Multiple DMA Channels for Transmitter"]
    #[inline]
    pub fn txdma(&self) -> TXDMAR {
        TXDMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline]
    pub fn txsame(&self) -> TXSAMER {
        TXSAMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline]
    pub fn imckfs(&self) -> IMCKFSR {
        IMCKFSR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline]
    pub fn imckmode(&self) -> IMCKMODER {
        IMCKMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - IWS Data Slot Width"]
    #[inline]
    pub fn iws24(&self) -> IWS24R {
        IWS24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Master/Slave/Controller Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline]
    pub fn datalength(&mut self) -> _DATALENGTHW {
        _DATALENGTHW { w: self }
    }
    #[doc = "Bit 8 - Receiver Mono"]
    #[inline]
    pub fn rxmono(&mut self) -> _RXMONOW {
        _RXMONOW { w: self }
    }
    #[doc = "Bit 9 - Single or Multiple DMA Channels for Receiver"]
    #[inline]
    pub fn rxdma(&mut self) -> _RXDMAW {
        _RXDMAW { w: self }
    }
    #[doc = "Bit 10 - Loop-back Test Mode"]
    #[inline]
    pub fn rxloop(&mut self) -> _RXLOOPW {
        _RXLOOPW { w: self }
    }
    #[doc = "Bit 12 - Transmitter Mono"]
    #[inline]
    pub fn txmono(&mut self) -> _TXMONOW {
        _TXMONOW { w: self }
    }
    #[doc = "Bit 13 - Single or Multiple DMA Channels for Transmitter"]
    #[inline]
    pub fn txdma(&mut self) -> _TXDMAW {
        _TXDMAW { w: self }
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline]
    pub fn txsame(&mut self) -> _TXSAMEW {
        _TXSAMEW { w: self }
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline]
    pub fn imckfs(&mut self) -> _IMCKFSW {
        _IMCKFSW { w: self }
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline]
    pub fn imckmode(&mut self) -> _IMCKMODEW {
        _IMCKMODEW { w: self }
    }
    #[doc = "Bit 31 - IWS Data Slot Width"]
    #[inline]
    pub fn iws24(&mut self) -> _IWS24W {
        _IWS24W { w: self }
    }
}
