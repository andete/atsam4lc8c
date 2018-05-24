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
#[doc = "Possible values of the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRR {
    #[doc = "SPI is in Slave mode."]
    _0,
    #[doc = "SPI is in Master mode."]
    _1,
}
impl MSTRR {
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
            MSTRR::_0 => false,
            MSTRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTRR {
        match value {
            false => MSTRR::_0,
            true => MSTRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSTRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSTRR::_1
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Fixed Peripheral Select."]
    _0,
    #[doc = "Variable Peripheral Select."]
    _1,
}
impl PSR {
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
            PSR::_0 => false,
            PSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::_0,
            true => PSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PSR::_1
    }
}
#[doc = "Possible values of the field `PCSDEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSDECR {
    #[doc = "The chip selects are directly connected to a peripheral device."]
    _0,
    #[doc = "The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    _1,
}
impl PCSDECR {
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
            PCSDECR::_0 => false,
            PCSDECR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCSDECR {
        match value {
            false => PCSDECR::_0,
            true => PCSDECR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCSDECR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCSDECR::_1
    }
}
#[doc = "Possible values of the field `MODFDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFDISR {
    #[doc = "Mode fault detection is enabled."]
    _0,
    #[doc = "Mode fault detection is disabled."]
    _1,
}
impl MODFDISR {
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
            MODFDISR::_0 => false,
            MODFDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODFDISR {
        match value {
            false => MODFDISR::_0,
            true => MODFDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MODFDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MODFDISR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WDRBTR {
    bits: bool,
}
impl WDRBTR {
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
pub struct RXFIFOENR {
    bits: bool,
}
impl RXFIFOENR {
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
#[doc = "Possible values of the field `LLB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLBR {
    #[doc = "Local loopback path disabled."]
    _0,
    #[doc = "Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    _1,
}
impl LLBR {
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
            LLBR::_0 => false,
            LLBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LLBR {
        match value {
            false => LLBR::_0,
            true => LLBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LLBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LLBR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PCSR {
    bits: u8,
}
impl PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYBCSR {
    bits: u8,
}
impl DLYBCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MSTR`"]
pub enum MSTRW {
    #[doc = "SPI is in Slave mode."]
    _0,
    #[doc = "SPI is in Master mode."]
    _1,
}
impl MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTRW::_0 => false,
            MSTRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI is in Slave mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTRW::_0)
    }
    #[doc = "SPI is in Master mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTRW::_1)
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
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Fixed Peripheral Select."]
    _0,
    #[doc = "Variable Peripheral Select."]
    _1,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSW::_0 => false,
            PSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed Peripheral Select."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSW::_0)
    }
    #[doc = "Variable Peripheral Select."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSW::_1)
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
#[doc = "Values that can be written to the field `PCSDEC`"]
pub enum PCSDECW {
    #[doc = "The chip selects are directly connected to a peripheral device."]
    _0,
    #[doc = "The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    _1,
}
impl PCSDECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCSDECW::_0 => false,
            PCSDECW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSDECW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSDECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSDECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The chip selects are directly connected to a peripheral device."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSDECW::_0)
    }
    #[doc = "The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSDECW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODFDIS`"]
pub enum MODFDISW {
    #[doc = "Mode fault detection is enabled."]
    _0,
    #[doc = "Mode fault detection is disabled."]
    _1,
}
impl MODFDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODFDISW::_0 => false,
            MODFDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODFDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MODFDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODFDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode fault detection is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFDISW::_0)
    }
    #[doc = "Mode fault detection is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFDISW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDRBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRBTW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFOENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFOENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LLB`"]
pub enum LLBW {
    #[doc = "Local loopback path disabled."]
    _0,
    #[doc = "Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    _1,
}
impl LLBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LLBW::_0 => false,
            LLBW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LLBW<'a> {
    w: &'a mut W,
}
impl<'a> _LLBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LLBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Local loopback path disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLBW::_0)
    }
    #[doc = "Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLBW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCSW<'a> {
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
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        MSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline]
    pub fn pcsdec(&self) -> PCSDECR {
        PCSDECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline]
    pub fn modfdis(&self) -> MODFDISR {
        MODFDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - wait data read before transfer"]
    #[inline]
    pub fn wdrbt(&self) -> WDRBTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDRBTR { bits }
    }
    #[doc = "Bit 6 - FIFO in Reception Enable"]
    #[inline]
    pub fn rxfifoen(&self) -> RXFIFOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFIFOENR { bits }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline]
    pub fn llb(&self) -> LLBR {
        LLBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCSR { bits }
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline]
    pub fn dlybcs(&self) -> DLYBCSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBCSR { bits }
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
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline]
    pub fn pcsdec(&mut self) -> _PCSDECW {
        _PCSDECW { w: self }
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline]
    pub fn modfdis(&mut self) -> _MODFDISW {
        _MODFDISW { w: self }
    }
    #[doc = "Bit 5 - wait data read before transfer"]
    #[inline]
    pub fn wdrbt(&mut self) -> _WDRBTW {
        _WDRBTW { w: self }
    }
    #[doc = "Bit 6 - FIFO in Reception Enable"]
    #[inline]
    pub fn rxfifoen(&mut self) -> _RXFIFOENW {
        _RXFIFOENW { w: self }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline]
    pub fn llb(&mut self) -> _LLBW {
        _LLBW { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline]
    pub fn dlybcs(&mut self) -> _DLYBCSW {
        _DLYBCSW { w: self }
    }
}
