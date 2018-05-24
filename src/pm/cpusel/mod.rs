#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPUSEL {
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
#[doc = "Possible values of the field `CPUSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUSELR {
    #[doc = "fCPU:fmain. CPUDIV:"]
    _0,
    #[doc = "fCPU:fmain / 2^(CPUSEL+1)"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CPUSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CPUSELR::_0 => 0,
            CPUSELR::_1 => 1,
            CPUSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CPUSELR {
        match value {
            0 => CPUSELR::_0,
            1 => CPUSELR::_1,
            i => CPUSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPUSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPUSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CPUDIVR {
    bits: bool,
}
impl CPUDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `CPUSEL`"]
pub enum CPUSELW {
    #[doc = "fCPU:fmain. CPUDIV:"]
    _0,
    #[doc = "fCPU:fmain / 2^(CPUSEL+1)"]
    _1,
}
impl CPUSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CPUSELW::_0 => 0,
            CPUSELW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPUSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CPUSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPUSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fCPU:fmain. CPUDIV:"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPUSELW::_0)
    }
    #[doc = "fCPU:fmain / 2^(CPUSEL+1)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPUSELW::_1)
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
#[doc = r" Proxy"]
pub struct _CPUDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CPUDIVW<'a> {
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline]
    pub fn cpusel(&self) -> CPUSELR {
        CPUSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline]
    pub fn cpudiv(&self) -> CPUDIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPUDIVR { bits }
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
    #[doc = "Bits 0:2 - CPU Clock Select"]
    #[inline]
    pub fn cpusel(&mut self) -> _CPUSELW {
        _CPUSELW { w: self }
    }
    #[doc = "Bit 7 - CPU Division"]
    #[inline]
    pub fn cpudiv(&mut self) -> _CPUDIVW {
        _CPUDIVW { w: self }
    }
}
