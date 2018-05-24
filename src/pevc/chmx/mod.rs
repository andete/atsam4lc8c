#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMX {
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
#[doc = "Possible values of the field `EVMX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVMXR {
    #[doc = "Event 0"]
    _0X00,
    #[doc = "Event 1"]
    _0X01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVMXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVMXR::_0X00 => 0,
            EVMXR::_0X01 => 1,
            EVMXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVMXR {
        match value {
            0 => EVMXR::_0X00,
            1 => EVMXR::_0X01,
            i => EVMXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == EVMXR::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline]
    pub fn is_0x01(&self) -> bool {
        *self == EVMXR::_0X01
    }
}
#[doc = "Possible values of the field `SMX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMXR {
    #[doc = "Hardware events"]
    _0,
    #[doc = "Software event"]
    _1,
}
impl SMXR {
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
            SMXR::_0 => false,
            SMXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMXR {
        match value {
            false => SMXR::_0,
            true => SMXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SMXR::_1
    }
}
#[doc = "Values that can be written to the field `EVMX`"]
pub enum EVMXW {
    #[doc = "Event 0"]
    _0X00,
    #[doc = "Event 1"]
    _0X01,
}
impl EVMXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVMXW::_0X00 => 0,
            EVMXW::_0X01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVMXW<'a> {
    w: &'a mut W,
}
impl<'a> _EVMXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVMXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Event 0"]
    #[inline]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(EVMXW::_0X00)
    }
    #[doc = "Event 1"]
    #[inline]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(EVMXW::_0X01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMX`"]
pub enum SMXW {
    #[doc = "Hardware events"]
    _0,
    #[doc = "Software event"]
    _1,
}
impl SMXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMXW::_0 => false,
            SMXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMXW<'a> {
    w: &'a mut W,
}
impl<'a> _SMXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware events"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMXW::_0)
    }
    #[doc = "Software event"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMXW::_1)
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
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline]
    pub fn evmx(&self) -> EVMXR {
        EVMXR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline]
    pub fn smx(&self) -> SMXR {
        SMXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:5 - Event Multiplexer"]
    #[inline]
    pub fn evmx(&mut self) -> _EVMXW {
        _EVMXW { w: self }
    }
    #[doc = "Bit 8 - Software Event Multiplexer"]
    #[inline]
    pub fn smx(&mut self) -> _SMXW {
        _SMXW { w: self }
    }
}
