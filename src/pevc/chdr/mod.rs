#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHDR {
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
#[doc = "Values that can be written to the field `CHD`"]
pub enum CHDW {
    #[doc = "No Action"]
    _0,
    #[doc = "Disable Channel j"]
    _1,
}
impl CHDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CHDW::_0 => 0,
            CHDW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHDW<'a> {
    w: &'a mut W,
}
impl<'a> _CHDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHDW::_0)
    }
    #[doc = "Disable Channel j"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHDW::_1)
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
    #[doc = "Bits 0:31 - Channel Disable"]
    #[inline]
    pub fn chd(&mut self) -> _CHDW {
        _CHDW { w: self }
    }
}
