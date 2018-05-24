#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDR {
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
}
#[doc = "Values that can be written to the field `OVF`"]
pub enum OVFW {
    #[doc = "No effect"]
    _0,
    #[doc = "Disable Interrupt."]
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
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFW::_0)
    }
    #[doc = "Disable Interrupt."]
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
    #[doc = "No effect"]
    _0,
    #[doc = "Disable interrupt"]
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
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM0W::_0)
    }
    #[doc = "Disable interrupt"]
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
    #[doc = "No effect"]
    _0,
    #[doc = "Disable interrupt"]
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
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM1W::_0)
    }
    #[doc = "Disable interrupt"]
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
    #[doc = "No effet"]
    _0,
    #[doc = "Disalbe interrupt"]
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
    #[doc = "No effet"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER0W::_0)
    }
    #[doc = "Disalbe interrupt"]
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
    #[doc = "No effect"]
    _0,
    #[doc = "Disable interrupt"]
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
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER1W::_0)
    }
    #[doc = "Disable interrupt"]
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
#[doc = "Values that can be written to the field `READY`"]
pub enum READYW {
    #[doc = "No effect"]
    _0,
    #[doc = "Disable interrupt"]
    _1,
}
impl READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READYW::_0 => false,
            READYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READYW<'a> {
    w: &'a mut W,
}
impl<'a> _READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(READYW::_0)
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(READYW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKRDY`"]
pub enum CLKRDYW {
    #[doc = "No effect"]
    _0,
    #[doc = "Disable interrupt"]
    _1,
}
impl CLKRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKRDYW::_0 => false,
            CLKRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKRDYW::_0)
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKRDYW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 25 - AST Ready"]
    #[inline]
    pub fn ready(&mut self) -> _READYW {
        _READYW { w: self }
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline]
    pub fn clkrdy(&mut self) -> _CLKRDYW {
        _CLKRDYW { w: self }
    }
}
