#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BGCTRL {
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
#[doc = "Possible values of the field `ADCISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCISELR {
    #[doc = "undocumented"]
    DIS,
    #[doc = "undocumented"]
    VTEMP,
    #[doc = "undocumented"]
    VREF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCISELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCISELR::DIS => 0,
            ADCISELR::VTEMP => 1,
            ADCISELR::VREF => 2,
            ADCISELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCISELR {
        match value {
            0 => ADCISELR::DIS,
            1 => ADCISELR::VTEMP,
            2 => ADCISELR::VREF,
            i => ADCISELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ADCISELR::DIS
    }
    #[doc = "Checks if the value of the field is `VTEMP`"]
    #[inline]
    pub fn is_vtemp(&self) -> bool {
        *self == ADCISELR::VTEMP
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline]
    pub fn is_vref(&self) -> bool {
        *self == ADCISELR::VREF
    }
}
#[doc = r" Value of the field"]
pub struct TSENR {
    bits: bool,
}
impl TSENR {
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
#[doc = "Values that can be written to the field `ADCISEL`"]
pub enum ADCISELW {
    #[doc = "`0`"]
    DIS,
    #[doc = "`1`"]
    VTEMP,
    #[doc = "`10`"]
    VREF,
}
impl ADCISELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCISELW::DIS => 0,
            ADCISELW::VTEMP => 1,
            ADCISELW::VREF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCISELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCISELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCISELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCISELW::DIS)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn vtemp(self) -> &'a mut W {
        self.variant(ADCISELW::VTEMP)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn vref(self) -> &'a mut W {
        self.variant(ADCISELW::VREF)
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
#[doc = r" Proxy"]
pub struct _TSENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENW<'a> {
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
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline]
    pub fn adcisel(&self) -> ADCISELR {
        ADCISELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline]
    pub fn tsen(&self) -> TSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSENR { bits }
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
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline]
    pub fn adcisel(&mut self) -> _ADCISELW {
        _ADCISELW { w: self }
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline]
    pub fn tsen(&mut self) -> _TSENW {
        _TSENW { w: self }
    }
}
