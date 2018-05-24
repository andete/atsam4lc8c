#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {
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
#[doc = "Possible values of the field `FRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDYR {
    #[doc = "Flash Ready does not generate an interrupt"]
    _0,
    #[doc = "Flash Ready generates an interrupt"]
    _1,
}
impl FRDYR {
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
            FRDYR::_0 => false,
            FRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRDYR {
        match value {
            false => FRDYR::_0,
            true => FRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRDYR::_1
    }
}
#[doc = "Possible values of the field `LOCKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKER {
    #[doc = "Lock Error does not generate an interrupt"]
    _0,
    #[doc = "Lock Error generates an interrupt"]
    _1,
}
impl LOCKER {
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
            LOCKER::_0 => false,
            LOCKER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKER {
        match value {
            false => LOCKER::_0,
            true => LOCKER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCKER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCKER::_1
    }
}
#[doc = "Possible values of the field `PROGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGER {
    #[doc = "Programming Error does not generate an interrupt"]
    _0,
    #[doc = "Programming Error generates an interrupt"]
    _1,
}
impl PROGER {
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
            PROGER::_0 => false,
            PROGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROGER {
        match value {
            false => PROGER::_0,
            true => PROGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PROGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PROGER::_1
    }
}
#[doc = "Possible values of the field `FWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWSR {
    #[doc = "The flash is read with 0 wait states"]
    _0,
    #[doc = "The flash is read with 1 wait states"]
    _1,
}
impl FWSR {
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
            FWSR::_0 => false,
            FWSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWSR {
        match value {
            false => FWSR::_0,
            true => FWSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FWSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FWSR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WS1OPTR {
    bits: bool,
}
impl WS1OPTR {
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
#[doc = "Values that can be written to the field `FRDY`"]
pub enum FRDYW {
    #[doc = "Flash Ready does not generate an interrupt"]
    _0,
    #[doc = "Flash Ready generates an interrupt"]
    _1,
}
impl FRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRDYW::_0 => false,
            FRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _FRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash Ready does not generate an interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDYW::_0)
    }
    #[doc = "Flash Ready generates an interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDYW::_1)
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
#[doc = "Values that can be written to the field `LOCKE`"]
pub enum LOCKEW {
    #[doc = "Lock Error does not generate an interrupt"]
    _0,
    #[doc = "Lock Error generates an interrupt"]
    _1,
}
impl LOCKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKEW::_0 => false,
            LOCKEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lock Error does not generate an interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKEW::_0)
    }
    #[doc = "Lock Error generates an interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKEW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROGE`"]
pub enum PROGEW {
    #[doc = "Programming Error does not generate an interrupt"]
    _0,
    #[doc = "Programming Error generates an interrupt"]
    _1,
}
impl PROGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROGEW::_0 => false,
            PROGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROGEW<'a> {
    w: &'a mut W,
}
impl<'a> _PROGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Programming Error does not generate an interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROGEW::_0)
    }
    #[doc = "Programming Error generates an interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROGEW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FWS`"]
pub enum FWSW {
    #[doc = "The flash is read with 0 wait states"]
    _0,
    #[doc = "The flash is read with 1 wait states"]
    _1,
}
impl FWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWSW::_0 => false,
            FWSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWSW<'a> {
    w: &'a mut W,
}
impl<'a> _FWSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The flash is read with 0 wait states"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWSW::_0)
    }
    #[doc = "The flash is read with 1 wait states"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWSW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WS1OPTW<'a> {
    w: &'a mut W,
}
impl<'a> _WS1OPTW<'a> {
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
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline]
    pub fn frdy(&self) -> FRDYR {
        FRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline]
    pub fn locke(&self) -> LOCKER {
        LOCKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline]
    pub fn proge(&self) -> PROGER {
        PROGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline]
    pub fn fws(&self) -> FWSR {
        FWSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline]
    pub fn ws1opt(&self) -> WS1OPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WS1OPTR { bits }
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
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline]
    pub fn frdy(&mut self) -> _FRDYW {
        _FRDYW { w: self }
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline]
    pub fn locke(&mut self) -> _LOCKEW {
        _LOCKEW { w: self }
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline]
    pub fn proge(&mut self) -> _PROGEW {
        _PROGEW { w: self }
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline]
    pub fn fws(&mut self) -> _FWSW {
        _FWSW { w: self }
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline]
    pub fn ws1opt(&mut self) -> _WS1OPTW {
        _WS1OPTW { w: self }
    }
}
