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
#[doc = "Possible values of the field `ULBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULBTR {
    #[doc = "Infinite Length"]
    INFINITE,
    #[doc = "Single Access"]
    SINGLE,
    #[doc = "Four Beat Burst"]
    FOUR_BEAT,
    #[doc = "Eight Beat Burst"]
    EIGHT_BEAT,
    #[doc = "Sixteen Beat Burst"]
    SIXTEEN_BEAT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ULBTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ULBTR::INFINITE => 0,
            ULBTR::SINGLE => 1,
            ULBTR::FOUR_BEAT => 2,
            ULBTR::EIGHT_BEAT => 3,
            ULBTR::SIXTEEN_BEAT => 4,
            ULBTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ULBTR {
        match value {
            0 => ULBTR::INFINITE,
            1 => ULBTR::SINGLE,
            2 => ULBTR::FOUR_BEAT,
            3 => ULBTR::EIGHT_BEAT,
            4 => ULBTR::SIXTEEN_BEAT,
            i => ULBTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INFINITE`"]
    #[inline]
    pub fn is_infinite(&self) -> bool {
        *self == ULBTR::INFINITE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == ULBTR::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR_BEAT`"]
    #[inline]
    pub fn is_four_beat(&self) -> bool {
        *self == ULBTR::FOUR_BEAT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BEAT`"]
    #[inline]
    pub fn is_eight_beat(&self) -> bool {
        *self == ULBTR::EIGHT_BEAT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BEAT`"]
    #[inline]
    pub fn is_sixteen_beat(&self) -> bool {
        *self == ULBTR::SIXTEEN_BEAT
    }
}
#[doc = "Values that can be written to the field `ULBT`"]
pub enum ULBTW {
    #[doc = "Infinite Length"]
    INFINITE,
    #[doc = "Single Access"]
    SINGLE,
    #[doc = "Four Beat Burst"]
    FOUR_BEAT,
    #[doc = "Eight Beat Burst"]
    EIGHT_BEAT,
    #[doc = "Sixteen Beat Burst"]
    SIXTEEN_BEAT,
}
impl ULBTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ULBTW::INFINITE => 0,
            ULBTW::SINGLE => 1,
            ULBTW::FOUR_BEAT => 2,
            ULBTW::EIGHT_BEAT => 3,
            ULBTW::SIXTEEN_BEAT => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULBTW<'a> {
    w: &'a mut W,
}
impl<'a> _ULBTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULBTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Infinite Length"]
    #[inline]
    pub fn infinite(self) -> &'a mut W {
        self.variant(ULBTW::INFINITE)
    }
    #[doc = "Single Access"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(ULBTW::SINGLE)
    }
    #[doc = "Four Beat Burst"]
    #[inline]
    pub fn four_beat(self) -> &'a mut W {
        self.variant(ULBTW::FOUR_BEAT)
    }
    #[doc = "Eight Beat Burst"]
    #[inline]
    pub fn eight_beat(self) -> &'a mut W {
        self.variant(ULBTW::EIGHT_BEAT)
    }
    #[doc = "Sixteen Beat Burst"]
    #[inline]
    pub fn sixteen_beat(self) -> &'a mut W {
        self.variant(ULBTW::SIXTEEN_BEAT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline]
    pub fn ulbt(&self) -> ULBTR {
        ULBTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline]
    pub fn ulbt(&mut self) -> _ULBTW {
        _ULBTW { w: self }
    }
}
