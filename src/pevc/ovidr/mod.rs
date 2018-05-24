#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVIDR {
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
#[doc = "Values that can be written to the field `OVID`"]
pub enum OVIDW {
    #[doc = "No Action"]
    _0,
    #[doc = "Enable Overrun Interrupt for Channel j"]
    _1,
}
impl OVIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            OVIDW::_0 => 0,
            OVIDW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVIDW<'a> {
    w: &'a mut W,
}
impl<'a> _OVIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVIDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVIDW::_0)
    }
    #[doc = "Enable Overrun Interrupt for Channel j"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVIDW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:31 - Overrun Interrupt Disable"]
    #[inline]
    pub fn ovid(&mut self) -> _OVIDW {
        _OVIDW { w: self }
    }
}
