#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Values that can be written to the field `RXOR`"]
pub enum RXORW {
    #[doc = "No effect"]
    NO,
    #[doc = "Clears the corresponding SR bit"]
    CLEAR,
}
impl RXORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXORW::NO => false,
            RXORW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXORW<'a> {
    w: &'a mut W,
}
impl<'a> _RXORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(RXORW::NO)
    }
    #[doc = "Clears the corresponding SR bit"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXORW::CLEAR)
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
#[doc = "Values that can be written to the field `TXUR`"]
pub enum TXURW {
    #[doc = "No effect"]
    NO,
    #[doc = "Clears the corresponding SR bit"]
    CLEAR,
}
impl TXURW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXURW::NO => false,
            TXURW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXURW<'a> {
    w: &'a mut W,
}
impl<'a> _TXURW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXURW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(TXURW::NO)
    }
    #[doc = "Clears the corresponding SR bit"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXURW::CLEAR)
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
#[doc = r" Proxy"]
pub struct _RXORCHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXORCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXURCHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXURCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline]
    pub fn rxor(&mut self) -> _RXORW {
        _RXORW { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline]
    pub fn txur(&mut self) -> _TXURW {
        _TXURW { w: self }
    }
    #[doc = "Bits 8:9 - Receive Overrun Channels"]
    #[inline]
    pub fn rxorch(&mut self) -> _RXORCHW {
        _RXORCHW { w: self }
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channels"]
    #[inline]
    pub fn txurch(&mut self) -> _TXURCHW {
        _TXURCHW { w: self }
    }
}
