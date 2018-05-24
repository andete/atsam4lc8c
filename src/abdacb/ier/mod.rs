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
#[doc = "Values that can be written to the field `TXRDY`"]
pub enum TXRDYW {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the Audio DAC TX Ready interrupt"]
    _1,
}
impl TXRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRDYW::_0 => false,
            TXRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDYW::_0)
    }
    #[doc = "Enables the Audio DAC TX Ready interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRDYW::_1)
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
#[doc = "Values that can be written to the field `TXUR`"]
pub enum TXURW {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the Audio DAC Underrun interrupt"]
    _1,
}
impl TXURW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXURW::_0 => false,
            TXURW::_1 => true,
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
    pub fn _0(self) -> &'a mut W {
        self.variant(TXURW::_0)
    }
    #[doc = "Enables the Audio DAC Underrun interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXURW::_1)
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
    #[doc = "Bit 1 - Transmit Ready Interrupt Enable"]
    #[inline]
    pub fn txrdy(&mut self) -> _TXRDYW {
        _TXRDYW { w: self }
    }
    #[doc = "Bit 2 - Transmit Underrun Interrupt Enable"]
    #[inline]
    pub fn txur(&mut self) -> _TXURW {
        _TXURW { w: self }
    }
}
