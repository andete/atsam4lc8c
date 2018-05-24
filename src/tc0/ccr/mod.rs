#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Values that can be written to the field `CLKEN`"]
pub enum CLKENW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the clock if CLKDIS is not 1."]
    _1,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::_0 => false,
            CLKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKENW::_0)
    }
    #[doc = "Enables the clock if CLKDIS is not 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKENW::_1)
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
#[doc = "Values that can be written to the field `CLKDIS`"]
pub enum CLKDISW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the clock."]
    _1,
}
impl CLKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKDISW::_0 => false,
            CLKDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKDISW::_0)
    }
    #[doc = "Disables the clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKDISW::_1)
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
#[doc = "Values that can be written to the field `SWTRG`"]
pub enum SWTRGW {
    #[doc = "No effect."]
    _0,
    #[doc = "A software trigger is performed:the counter is reset and clock is started."]
    _1,
}
impl SWTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWTRGW::_0 => false,
            SWTRGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWTRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTRGW::_0)
    }
    #[doc = "A software trigger is performed:the counter is reset and clock is started."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTRGW::_1)
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
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline]
    pub fn clkdis(&mut self) -> _CLKDISW {
        _CLKDISW { w: self }
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline]
    pub fn swtrg(&mut self) -> _SWTRGW {
        _SWTRGW { w: self }
    }
}
