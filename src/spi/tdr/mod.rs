#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TDR {
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
pub struct _TDW<'a> {
    w: &'a mut W,
}
impl<'a> _TDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
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
#[doc = "Values that can be written to the field `LASTXFER`"]
pub enum LASTXFERW {
    #[doc = "No effect."]
    _0,
    #[doc = "The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    _1,
}
impl LASTXFERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LASTXFERW::_0 => false,
            LASTXFERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LASTXFERW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTXFERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LASTXFERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LASTXFERW::_0)
    }
    #[doc = "The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LASTXFERW::_1)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline]
    pub fn td(&mut self) -> _TDW {
        _TDW { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline]
    pub fn lastxfer(&mut self) -> _LASTXFERW {
        _LASTXFERW { w: self }
    }
}
