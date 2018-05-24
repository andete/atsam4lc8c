#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BRGR {
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
#[doc = "Possible values of the field `CD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDR {
    #[doc = "Disables the clock"]
    DISABLE,
    #[doc = "Clock Divisor Bypass"]
    BYPASS,
    #[doc = "Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    _2,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CDR::DISABLE => 0,
            CDR::BYPASS => 1,
            CDR::_2 => 2,
            CDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CDR {
        match value {
            0 => CDR::DISABLE,
            1 => CDR::BYPASS,
            2 => CDR::_2,
            i => CDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CDR::DISABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == CDR::BYPASS
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == CDR::_2
    }
}
#[doc = "Possible values of the field `FP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPR {
    #[doc = "Fractional divider is disabled"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FPR::_0 => 0,
            FPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FPR {
        match value {
            0 => FPR::_0,
            i => FPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FPR::_0
    }
}
#[doc = "Values that can be written to the field `CD`"]
pub enum CDW {
    #[doc = "Disables the clock"]
    DISABLE,
    #[doc = "Clock Divisor Bypass"]
    BYPASS,
    #[doc = "Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    _2,
}
impl CDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            CDW::DISABLE => 0,
            CDW::BYPASS => 1,
            CDW::_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDW<'a> {
    w: &'a mut W,
}
impl<'a> _CDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables the clock"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CDW::DISABLE)
    }
    #[doc = "Clock Divisor Bypass"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(CDW::BYPASS)
    }
    #[doc = "Baud Rate (Asynchronous Mode) = Selected Clock/(16 x CD) or (8 x CD); Baud Rate (Synchronous Mode) = Selected Clock/CD;"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(CDW::_2)
    }
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
#[doc = "Values that can be written to the field `FP`"]
pub enum FPW {
    #[doc = "Fractional divider is disabled"]
    _0,
}
impl FPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FPW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPW<'a> {
    w: &'a mut W,
}
impl<'a> _FPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Fractional divider is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline]
    pub fn cd(&self) -> CDR {
        CDR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline]
    pub fn fp(&self) -> FPR {
        FPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline]
    pub fn cd(&mut self) -> _CDW {
        _CDW { w: self }
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline]
    pub fn fp(&mut self) -> _FPW {
        _FPW { w: self }
    }
}
