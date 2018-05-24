#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCK {
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
#[doc = "Possible values of the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENR {
    #[doc = "The clock is disabled"]
    _0,
    #[doc = "The clock is enabled"]
    _1,
}
impl CENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CENR::_0 => false,
            CENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENR {
        match value {
            false => CENR::_0,
            true => CENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CENR::_1
    }
}
#[doc = "Possible values of the field `CSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSELR {
    #[doc = "Slow clock"]
    SLOWCLOCK,
    #[doc = "32 kHz clock"]
    _32KHZCLK,
    #[doc = "PB clock"]
    PBCLOCK,
    #[doc = "Generic clock"]
    GCLK,
    #[doc = "1kHz clock from 32 kHz oscillator"]
    _1KHZCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSSELR::SLOWCLOCK => 0,
            CSSELR::_32KHZCLK => 1,
            CSSELR::PBCLOCK => 2,
            CSSELR::GCLK => 3,
            CSSELR::_1KHZCLK => 4,
            CSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSSELR {
        match value {
            0 => CSSELR::SLOWCLOCK,
            1 => CSSELR::_32KHZCLK,
            2 => CSSELR::PBCLOCK,
            3 => CSSELR::GCLK,
            4 => CSSELR::_1KHZCLK,
            i => CSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOWCLOCK`"]
    #[inline]
    pub fn is_slowclock(&self) -> bool {
        *self == CSSELR::SLOWCLOCK
    }
    #[doc = "Checks if the value of the field is `_32KHZCLK`"]
    #[inline]
    pub fn is_32khzclk(&self) -> bool {
        *self == CSSELR::_32KHZCLK
    }
    #[doc = "Checks if the value of the field is `PBCLOCK`"]
    #[inline]
    pub fn is_pbclock(&self) -> bool {
        *self == CSSELR::PBCLOCK
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == CSSELR::GCLK
    }
    #[doc = "Checks if the value of the field is `_1KHZCLK`"]
    #[inline]
    pub fn is_1khzclk(&self) -> bool {
        *self == CSSELR::_1KHZCLK
    }
}
#[doc = "Values that can be written to the field `CEN`"]
pub enum CENW {
    #[doc = "The clock is disabled"]
    _0,
    #[doc = "The clock is enabled"]
    _1,
}
impl CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CENW::_0 => false,
            CENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CENW::_0)
    }
    #[doc = "The clock is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CENW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSSEL`"]
pub enum CSSELW {
    #[doc = "Slow clock"]
    SLOWCLOCK,
    #[doc = "32 kHz clock"]
    _32KHZCLK,
    #[doc = "PB clock"]
    PBCLOCK,
    #[doc = "Generic clock"]
    GCLK,
    #[doc = "1kHz clock from 32 kHz oscillator"]
    _1KHZCLK,
}
impl CSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSSELW::SLOWCLOCK => 0,
            CSSELW::_32KHZCLK => 1,
            CSSELW::PBCLOCK => 2,
            CSSELW::GCLK => 3,
            CSSELW::_1KHZCLK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slow clock"]
    #[inline]
    pub fn slowclock(self) -> &'a mut W {
        self.variant(CSSELW::SLOWCLOCK)
    }
    #[doc = "32 kHz clock"]
    #[inline]
    pub fn _32khzclk(self) -> &'a mut W {
        self.variant(CSSELW::_32KHZCLK)
    }
    #[doc = "PB clock"]
    #[inline]
    pub fn pbclock(self) -> &'a mut W {
        self.variant(CSSELW::PBCLOCK)
    }
    #[doc = "Generic clock"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(CSSELW::GCLK)
    }
    #[doc = "1kHz clock from 32 kHz oscillator"]
    #[inline]
    pub fn _1khzclk(self) -> &'a mut W {
        self.variant(CSSELW::_1KHZCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Clock Enable"]
    #[inline]
    pub fn cen(&self) -> CENR {
        CENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline]
    pub fn cssel(&self) -> CSSELR {
        CSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Clock Enable"]
    #[inline]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
    #[doc = "Bits 8:10 - Clock Source Selection"]
    #[inline]
    pub fn cssel(&mut self) -> _CSSELW {
        _CSSELW { w: self }
    }
}
