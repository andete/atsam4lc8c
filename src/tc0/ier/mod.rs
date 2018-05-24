#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Values that can be written to the field `COVFS`"]
pub enum COVFSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the Counter Overflow Interrupt."]
    _1,
}
impl COVFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COVFSW::_0 => false,
            COVFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COVFSW<'a> {
    w: &'a mut W,
}
impl<'a> _COVFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COVFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COVFSW::_0)
    }
    #[doc = "Enables the Counter Overflow Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COVFSW::_1)
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
#[doc = "Values that can be written to the field `LOVRS`"]
pub enum LOVRSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the Load Overrun Interrupt."]
    _1,
}
impl LOVRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOVRSW::_0 => false,
            LOVRSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOVRSW<'a> {
    w: &'a mut W,
}
impl<'a> _LOVRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOVRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOVRSW::_0)
    }
    #[doc = "Enables the Load Overrun Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOVRSW::_1)
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
#[doc = "Values that can be written to the field `CPAS`"]
pub enum CPASW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the RA Compare Interrupt."]
    _1,
}
impl CPASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPASW::_0 => false,
            CPASW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPASW<'a> {
    w: &'a mut W,
}
impl<'a> _CPASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPASW::_0)
    }
    #[doc = "Enables the RA Compare Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPASW::_1)
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
#[doc = "Values that can be written to the field `CPBS`"]
pub enum CPBSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the RB Compare Interrupt."]
    _1,
}
impl CPBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPBSW::_0 => false,
            CPBSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPBSW<'a> {
    w: &'a mut W,
}
impl<'a> _CPBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPBSW::_0)
    }
    #[doc = "Enables the RB Compare Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPBSW::_1)
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
#[doc = "Values that can be written to the field `CPCS`"]
pub enum CPCSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the RC Compare Interrupt."]
    _1,
}
impl CPCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPCSW::_0 => false,
            CPCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPCSW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCSW::_0)
    }
    #[doc = "Enables the RC Compare Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCSW::_1)
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
#[doc = "Values that can be written to the field `LDRAS`"]
pub enum LDRASW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the RA Load Interrupt."]
    _1,
}
impl LDRASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDRASW::_0 => false,
            LDRASW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDRASW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDRASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDRASW::_0)
    }
    #[doc = "Enables the RA Load Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDRASW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LDRBS`"]
pub enum LDRBSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the RB Load Interrupt."]
    _1,
}
impl LDRBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDRBSW::_0 => false,
            LDRBSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDRBSW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDRBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDRBSW::_0)
    }
    #[doc = "Enables the RB Load Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDRBSW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETRGS`"]
pub enum ETRGSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the External Trigger Interrupt."]
    _1,
}
impl ETRGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETRGSW::_0 => false,
            ETRGSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETRGSW<'a> {
    w: &'a mut W,
}
impl<'a> _ETRGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETRGSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETRGSW::_0)
    }
    #[doc = "Enables the External Trigger Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETRGSW::_1)
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
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline]
    pub fn covfs(&mut self) -> _COVFSW {
        _COVFSW { w: self }
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline]
    pub fn lovrs(&mut self) -> _LOVRSW {
        _LOVRSW { w: self }
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline]
    pub fn cpas(&mut self) -> _CPASW {
        _CPASW { w: self }
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline]
    pub fn cpbs(&mut self) -> _CPBSW {
        _CPBSW { w: self }
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline]
    pub fn cpcs(&mut self) -> _CPCSW {
        _CPCSW { w: self }
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline]
    pub fn ldras(&mut self) -> _LDRASW {
        _LDRASW { w: self }
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline]
    pub fn ldrbs(&mut self) -> _LDRBSW {
        _LDRBSW { w: self }
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline]
    pub fn etrgs(&mut self) -> _ETRGSW {
        _ETRGSW { w: self }
    }
}
