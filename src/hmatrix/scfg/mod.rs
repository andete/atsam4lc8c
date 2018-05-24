#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCFG {
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
#[doc = r" Value of the field"]
pub struct SLOT_CYCLER {
    bits: u8,
}
impl SLOT_CYCLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DEFMSTR_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEFMSTR_TYPER {
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    NO_DEFAULT,
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    LAST_DEFAULT,
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    FIXED_DEFAULT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEFMSTR_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEFMSTR_TYPER::NO_DEFAULT => 0,
            DEFMSTR_TYPER::LAST_DEFAULT => 1,
            DEFMSTR_TYPER::FIXED_DEFAULT => 2,
            DEFMSTR_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEFMSTR_TYPER {
        match value {
            0 => DEFMSTR_TYPER::NO_DEFAULT,
            1 => DEFMSTR_TYPER::LAST_DEFAULT,
            2 => DEFMSTR_TYPER::FIXED_DEFAULT,
            i => DEFMSTR_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEFAULT`"]
    #[inline]
    pub fn is_no_default(&self) -> bool {
        *self == DEFMSTR_TYPER::NO_DEFAULT
    }
    #[doc = "Checks if the value of the field is `LAST_DEFAULT`"]
    #[inline]
    pub fn is_last_default(&self) -> bool {
        *self == DEFMSTR_TYPER::LAST_DEFAULT
    }
    #[doc = "Checks if the value of the field is `FIXED_DEFAULT`"]
    #[inline]
    pub fn is_fixed_default(&self) -> bool {
        *self == DEFMSTR_TYPER::FIXED_DEFAULT
    }
}
#[doc = r" Value of the field"]
pub struct FIXED_DEFMSTRR {
    bits: u8,
}
impl FIXED_DEFMSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ARBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBTR {
    #[doc = "Round-Robin Arbitration"]
    ROUND_ROBIN,
    #[doc = "Fixed Priority Arbitration"]
    FIXED_PRIORITY,
}
impl ARBTR {
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
            ARBTR::ROUND_ROBIN => false,
            ARBTR::FIXED_PRIORITY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBTR {
        match value {
            false => ARBTR::ROUND_ROBIN,
            true => ARBTR::FIXED_PRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline]
    pub fn is_round_robin(&self) -> bool {
        *self == ARBTR::ROUND_ROBIN
    }
    #[doc = "Checks if the value of the field is `FIXED_PRIORITY`"]
    #[inline]
    pub fn is_fixed_priority(&self) -> bool {
        *self == ARBTR::FIXED_PRIORITY
    }
}
#[doc = r" Proxy"]
pub struct _SLOT_CYCLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOT_CYCLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEFMSTR_TYPE`"]
pub enum DEFMSTR_TYPEW {
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    NO_DEFAULT,
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    LAST_DEFAULT,
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    FIXED_DEFAULT,
}
impl DEFMSTR_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEFMSTR_TYPEW::NO_DEFAULT => 0,
            DEFMSTR_TYPEW::LAST_DEFAULT => 1,
            DEFMSTR_TYPEW::FIXED_DEFAULT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEFMSTR_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEFMSTR_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEFMSTR_TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    #[inline]
    pub fn no_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPEW::NO_DEFAULT)
    }
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    #[inline]
    pub fn last_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPEW::LAST_DEFAULT)
    }
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    #[inline]
    pub fn fixed_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPEW::FIXED_DEFAULT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIXED_DEFMSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXED_DEFMSTRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARBT`"]
pub enum ARBTW {
    #[doc = "Round-Robin Arbitration"]
    ROUND_ROBIN,
    #[doc = "Fixed Priority Arbitration"]
    FIXED_PRIORITY,
}
impl ARBTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBTW::ROUND_ROBIN => false,
            ARBTW::FIXED_PRIORITY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBTW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Round-Robin Arbitration"]
    #[inline]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARBTW::ROUND_ROBIN)
    }
    #[doc = "Fixed Priority Arbitration"]
    #[inline]
    pub fn fixed_priority(self) -> &'a mut W {
        self.variant(ARBTW::FIXED_PRIORITY)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline]
    pub fn slot_cycle(&self) -> SLOT_CYCLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLOT_CYCLER { bits }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPER {
        DEFMSTR_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIXED_DEFMSTRR { bits }
    }
    #[doc = "Bit 24 - Arbitration Type"]
    #[inline]
    pub fn arbt(&self) -> ARBTR {
        ARBTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline]
    pub fn slot_cycle(&mut self) -> _SLOT_CYCLEW {
        _SLOT_CYCLEW { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline]
    pub fn defmstr_type(&mut self) -> _DEFMSTR_TYPEW {
        _DEFMSTR_TYPEW { w: self }
    }
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline]
    pub fn fixed_defmstr(&mut self) -> _FIXED_DEFMSTRW {
        _FIXED_DEFMSTRW { w: self }
    }
    #[doc = "Bit 24 - Arbitration Type"]
    #[inline]
    pub fn arbt(&mut self) -> _ARBTW {
        _ARBTW { w: self }
    }
}
