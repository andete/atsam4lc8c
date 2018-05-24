#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIDI {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `FI_DI_RATIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FI_DI_RATIOR {
    #[doc = "Baud Rate = 0"]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FI_DI_RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FI_DI_RATIOR::DISABLE => 0,
            FI_DI_RATIOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FI_DI_RATIOR {
        match value {
            0 => FI_DI_RATIOR::DISABLE,
            i => FI_DI_RATIOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FI_DI_RATIOR::DISABLE
    }
}
#[doc = "Values that can be written to the field `FI_DI_RATIO`"]
pub enum FI_DI_RATIOW {
    #[doc = "Baud Rate = 0"]
    DISABLE,
}
impl FI_DI_RATIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            FI_DI_RATIOW::DISABLE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FI_DI_RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _FI_DI_RATIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FI_DI_RATIOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Baud Rate = 0"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FI_DI_RATIOW::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline]
    pub fn fi_di_ratio(&self) -> FI_DI_RATIOR {
        FI_DI_RATIOR::_from({
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 372 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline]
    pub fn fi_di_ratio(&mut self) -> _FI_DI_RATIOW {
        _FI_DI_RATIOW { w: self }
    }
}
