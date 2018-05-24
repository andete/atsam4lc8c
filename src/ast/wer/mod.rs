#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WER {
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
#[doc = "Possible values of the field `OVF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVFR {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl OVFR {
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
            OVFR::_0 => false,
            OVFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVFR {
        match value {
            false => OVFR::_0,
            true => OVFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVFR::_1
    }
}
#[doc = "Possible values of the field `ALARM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM0R {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl ALARM0R {
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
            ALARM0R::_0 => false,
            ALARM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARM0R {
        match value {
            false => ALARM0R::_0,
            true => ALARM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALARM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALARM0R::_1
    }
}
#[doc = "Possible values of the field `ALARM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1R {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl ALARM1R {
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
            ALARM1R::_0 => false,
            ALARM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARM1R {
        match value {
            false => ALARM1R::_0,
            true => ALARM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALARM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALARM1R::_1
    }
}
#[doc = "Possible values of the field `PER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER0R {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl PER0R {
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
            PER0R::_0 => false,
            PER0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER0R {
        match value {
            false => PER0R::_0,
            true => PER0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PER0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PER0R::_1
    }
}
#[doc = "Possible values of the field `PER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER1R {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl PER1R {
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
            PER1R::_0 => false,
            PER1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER1R {
        match value {
            false => PER1R::_0,
            true => PER1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PER1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PER1R::_1
    }
}
#[doc = "Values that can be written to the field `OVF`"]
pub enum OVFW {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl OVFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVFW::_0 => false,
            OVFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVFW<'a> {
    w: &'a mut W,
}
impl<'a> _OVFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFW::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFW::_1)
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
#[doc = "Values that can be written to the field `ALARM0`"]
pub enum ALARM0W {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl ALARM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARM0W::_0 => false,
            ALARM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARM0W<'a> {
    w: &'a mut W,
}
impl<'a> _ALARM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM0W::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM0W::_1)
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
#[doc = "Values that can be written to the field `ALARM1`"]
pub enum ALARM1W {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl ALARM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARM1W::_0 => false,
            ALARM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARM1W<'a> {
    w: &'a mut W,
}
impl<'a> _ALARM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM1W::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM1W::_1)
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
#[doc = "Values that can be written to the field `PER0`"]
pub enum PER0W {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl PER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PER0W::_0 => false,
            PER0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PER0W<'a> {
    w: &'a mut W,
}
impl<'a> _PER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PER0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER0W::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER0W::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PER1`"]
pub enum PER1W {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    _0,
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    _1,
}
impl PER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PER1W::_0 => false,
            PER1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PER1W<'a> {
    w: &'a mut W,
}
impl<'a> _PER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PER1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER1W::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER1W::_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Overflow"]
    #[inline]
    pub fn ovf(&self) -> OVFR {
        OVFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline]
    pub fn alarm0(&self) -> ALARM0R {
        ALARM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline]
    pub fn alarm1(&self) -> ALARM1R {
        ALARM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline]
    pub fn per0(&self) -> PER0R {
        PER0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline]
    pub fn per1(&self) -> PER1R {
        PER1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Overflow"]
    #[inline]
    pub fn ovf(&mut self) -> _OVFW {
        _OVFW { w: self }
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline]
    pub fn alarm0(&mut self) -> _ALARM0W {
        _ALARM0W { w: self }
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline]
    pub fn alarm1(&mut self) -> _ALARM1W {
        _ALARM1W { w: self }
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline]
    pub fn per0(&mut self) -> _PER0W {
        _PER0W { w: self }
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline]
    pub fn per1(&mut self) -> _PER1W {
        _PER1W { w: self }
    }
}
