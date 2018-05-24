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
#[doc = "Values that can be written to the field `RXRDY`"]
pub enum RXRDYW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables the corresponding interrupt"]
    ON,
}
impl RXRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXRDYW::OFF => false,
            RXRDYW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(RXRDYW::OFF)
    }
    #[doc = "Enables the corresponding interrupt"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(RXRDYW::ON)
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
#[doc = "Values that can be written to the field `RXOR`"]
pub enum RXORW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables the corresponding interrupt"]
    ON,
}
impl RXORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXORW::OFF => false,
            RXORW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(RXORW::OFF)
    }
    #[doc = "Enables the corresponding interrupt"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(RXORW::ON)
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
#[doc = "Values that can be written to the field `TXRDY`"]
pub enum TXRDYW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables the corresponding interrupt"]
    ON,
}
impl TXRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRDYW::OFF => false,
            TXRDYW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(TXRDYW::OFF)
    }
    #[doc = "Enables the corresponding interrupt"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(TXRDYW::ON)
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
#[doc = "Values that can be written to the field `TXUR`"]
pub enum TXURW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables the corresponding interrupt"]
    ON,
}
impl TXURW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXURW::OFF => false,
            TXURW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(TXURW::OFF)
    }
    #[doc = "Enables the corresponding interrupt"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(TXURW::ON)
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
    #[doc = "Bit 1 - Receiver Ready Interrupt Enable"]
    #[inline]
    pub fn rxrdy(&mut self) -> _RXRDYW {
        _RXRDYW { w: self }
    }
    #[doc = "Bit 2 - Receive Overrun Interrupt Enable"]
    #[inline]
    pub fn rxor(&mut self) -> _RXORW {
        _RXORW { w: self }
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Enable"]
    #[inline]
    pub fn txrdy(&mut self) -> _TXRDYW {
        _TXRDYW { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun Interrupt Enable"]
    #[inline]
    pub fn txur(&mut self) -> _TXURW {
        _TXURW { w: self }
    }
}
