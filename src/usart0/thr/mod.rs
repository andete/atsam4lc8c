#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::THR {
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
#[doc = r" Proxy"]
pub struct _TXCHRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCHRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSYNH`"]
pub enum TXSYNHW {
    #[doc = "The next character sent is encoded as a data. Start Frame Delimiter is DATA SYNC"]
    _0,
    #[doc = "The next character sent is encoded as a command. Start Frame Delimiter is COMMAND SYNC"]
    _1,
}
impl TXSYNHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSYNHW::_0 => false,
            TXSYNHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSYNHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSYNHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSYNHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The next character sent is encoded as a data. Start Frame Delimiter is DATA SYNC"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSYNHW::_0)
    }
    #[doc = "The next character sent is encoded as a command. Start Frame Delimiter is COMMAND SYNC"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXSYNHW::_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline]
    pub fn txchr(&mut self) -> _TXCHRW {
        _TXCHRW { w: self }
    }
    #[doc = "Bit 15 - Sync Field to be transmitted"]
    #[inline]
    pub fn txsynh(&mut self) -> _TXSYNHW {
        _TXSYNHW { w: self }
    }
}
