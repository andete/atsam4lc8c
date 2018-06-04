#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCCTRL {
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
#[doc = "Possible values of the field `MCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCSELR {
    #[doc = "System RC oscillator"]
    RCSYS,
    #[doc = "Oscillator0"]
    OSC0,
    #[doc = "PLL"]
    PLL,
    #[doc = "DFLL"]
    DFLL,
    #[doc = "80 MHz RC oscillator"]
    RC80M,
    #[doc = "4/8/12 MHz RC oscillator"]
    RCFAST,
    #[doc = "1 MHz RC oscillator"]
    RC1M,
}
impl MCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCSELR::RCSYS => 0,
            MCSELR::OSC0 => 1,
            MCSELR::PLL => 2,
            MCSELR::DFLL => 3,
            MCSELR::RC80M => 4,
            MCSELR::RCFAST => 5,
            MCSELR::RC1M => 6,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCSELR {
        match value {
            0 => MCSELR::RCSYS,
            1 => MCSELR::OSC0,
            2 => MCSELR::PLL,
            3 => MCSELR::DFLL,
            4 => MCSELR::RC80M,
            5 => MCSELR::RCFAST,
            6 => MCSELR::RC1M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCSYS`"]
    #[inline]
    pub fn is_rcsys(&self) -> bool {
        *self == MCSELR::RCSYS
    }
    #[doc = "Checks if the value of the field is `OSC0`"]
    #[inline]
    pub fn is_osc0(&self) -> bool {
        *self == MCSELR::OSC0
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == MCSELR::PLL
    }
    #[doc = "Checks if the value of the field is `DFLL`"]
    #[inline]
    pub fn is_dfll(&self) -> bool {
        *self == MCSELR::DFLL
    }
    #[doc = "Checks if the value of the field is `RC80M`"]
    #[inline]
    pub fn is_rc80m(&self) -> bool {
        *self == MCSELR::RC80M
    }
    #[doc = "Checks if the value of the field is `RCFAST`"]
    #[inline]
    pub fn is_rcfast(&self) -> bool {
        *self == MCSELR::RCFAST
    }
    #[doc = "Checks if the value of the field is `RC1M`"]
    #[inline]
    pub fn is_rc1m(&self) -> bool {
        *self == MCSELR::RC1M
    }
}
#[doc = "Values that can be written to the field `MCSEL`"]
pub enum MCSELW {
    #[doc = "System RC oscillator"]
    RCSYS,
    #[doc = "Oscillator0"]
    OSC0,
    #[doc = "PLL"]
    PLL,
    #[doc = "DFLL"]
    DFLL,
    #[doc = "80 MHz RC oscillator"]
    RC80M,
    #[doc = "4/8/12 MHz RC oscillator"]
    RCFAST,
    #[doc = "1 MHz RC oscillator"]
    RC1M,
}
impl MCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCSELW::RCSYS => 0,
            MCSELW::OSC0 => 1,
            MCSELW::PLL => 2,
            MCSELW::DFLL => 3,
            MCSELW::RC80M => 4,
            MCSELW::RCFAST => 5,
            MCSELW::RC1M => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System RC oscillator"]
    #[inline]
    pub fn rcsys(self) -> &'a mut W {
        self.variant(MCSELW::RCSYS)
    }
    #[doc = "Oscillator0"]
    #[inline]
    pub fn osc0(self) -> &'a mut W {
        self.variant(MCSELW::OSC0)
    }
    #[doc = "PLL"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCSELW::PLL)
    }
    #[doc = "DFLL"]
    #[inline]
    pub fn dfll(self) -> &'a mut W {
        self.variant(MCSELW::DFLL)
    }
    #[doc = "80 MHz RC oscillator"]
    #[inline]
    pub fn rc80m(self) -> &'a mut W {
        self.variant(MCSELW::RC80M)
    }
    #[doc = "4/8/12 MHz RC oscillator"]
    #[inline]
    pub fn rcfast(self) -> &'a mut W {
        self.variant(MCSELW::RCFAST)
    }
    #[doc = "1 MHz RC oscillator"]
    #[inline]
    pub fn rc1m(self) -> &'a mut W {
        self.variant(MCSELW::RC1M)
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
    #[doc = "Bits 0:2 - Main Clock Select"]
    #[inline]
    pub fn mcsel(&self) -> MCSELR {
        MCSELR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Main Clock Select"]
    #[inline]
    pub fn mcsel(&mut self) -> _MCSELW {
        _MCSELW { w: self }
    }
}
