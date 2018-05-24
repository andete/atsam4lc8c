#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCFG {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Cycle Counter"]
    CYCLE,
    #[doc = "Instruction Hit Counter"]
    IHIT,
    #[doc = "Data Hit Counter"]
    DHIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::CYCLE => 0,
            MODER::IHIT => 1,
            MODER::DHIT => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::CYCLE,
            1 => MODER::IHIT,
            2 => MODER::DHIT,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline]
    pub fn is_cycle(&self) -> bool {
        *self == MODER::CYCLE
    }
    #[doc = "Checks if the value of the field is `IHIT`"]
    #[inline]
    pub fn is_ihit(&self) -> bool {
        *self == MODER::IHIT
    }
    #[doc = "Checks if the value of the field is `DHIT`"]
    #[inline]
    pub fn is_dhit(&self) -> bool {
        *self == MODER::DHIT
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Cycle Counter"]
    CYCLE,
    #[doc = "Instruction Hit Counter"]
    IHIT,
    #[doc = "Data Hit Counter"]
    DHIT,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::CYCLE => 0,
            MODEW::IHIT => 1,
            MODEW::DHIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Cycle Counter"]
    #[inline]
    pub fn cycle(self) -> &'a mut W {
        self.variant(MODEW::CYCLE)
    }
    #[doc = "Instruction Hit Counter"]
    #[inline]
    pub fn ihit(self) -> &'a mut W {
        self.variant(MODEW::IHIT)
    }
    #[doc = "Data Hit Counter"]
    #[inline]
    pub fn dhit(self) -> &'a mut W {
        self.variant(MODEW::DHIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
