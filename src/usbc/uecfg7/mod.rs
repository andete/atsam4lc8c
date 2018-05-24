#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UECFG7 {
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
#[doc = "Possible values of the field `EPBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPBKR {
    #[doc = "undocumented"]
    SINGLE,
    #[doc = "undocumented"]
    DOUBLE,
}
impl EPBKR {
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
            EPBKR::SINGLE => false,
            EPBKR::DOUBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPBKR {
        match value {
            false => EPBKR::SINGLE,
            true => EPBKR::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == EPBKR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline]
    pub fn is_double(&self) -> bool {
        *self == EPBKR::DOUBLE
    }
}
#[doc = "Possible values of the field `EPSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPSIZER {
    #[doc = "undocumented"]
    _8,
    #[doc = "undocumented"]
    _16,
    #[doc = "undocumented"]
    _32,
    #[doc = "undocumented"]
    _64,
    #[doc = "undocumented"]
    _128,
    #[doc = "undocumented"]
    _256,
    #[doc = "undocumented"]
    _512,
    #[doc = "undocumented"]
    _1024,
}
impl EPSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPSIZER::_8 => 0,
            EPSIZER::_16 => 1,
            EPSIZER::_32 => 2,
            EPSIZER::_64 => 3,
            EPSIZER::_128 => 4,
            EPSIZER::_256 => 5,
            EPSIZER::_512 => 6,
            EPSIZER::_1024 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPSIZER {
        match value {
            0 => EPSIZER::_8,
            1 => EPSIZER::_16,
            2 => EPSIZER::_32,
            3 => EPSIZER::_64,
            4 => EPSIZER::_128,
            5 => EPSIZER::_256,
            6 => EPSIZER::_512,
            7 => EPSIZER::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == EPSIZER::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == EPSIZER::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == EPSIZER::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == EPSIZER::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == EPSIZER::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == EPSIZER::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == EPSIZER::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == EPSIZER::_1024
    }
}
#[doc = "Possible values of the field `EPDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIRR {
    #[doc = "undocumented"]
    OUT,
    #[doc = "undocumented"]
    IN,
}
impl EPDIRR {
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
            EPDIRR::OUT => false,
            EPDIRR::IN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPDIRR {
        match value {
            false => EPDIRR::OUT,
            true => EPDIRR::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == EPDIRR::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == EPDIRR::IN
    }
}
#[doc = "Possible values of the field `EPTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPTYPER {
    #[doc = "undocumented"]
    CONTROL,
    #[doc = "undocumented"]
    ISOCHRONOUS,
    #[doc = "undocumented"]
    BULK,
    #[doc = "undocumented"]
    INTERRUPT,
}
impl EPTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPTYPER::CONTROL => 0,
            EPTYPER::ISOCHRONOUS => 1,
            EPTYPER::BULK => 2,
            EPTYPER::INTERRUPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPTYPER {
        match value {
            0 => EPTYPER::CONTROL,
            1 => EPTYPER::ISOCHRONOUS,
            2 => EPTYPER::BULK,
            3 => EPTYPER::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == EPTYPER::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline]
    pub fn is_isochronous(&self) -> bool {
        *self == EPTYPER::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPER::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == EPTYPER::INTERRUPT
    }
}
#[doc = r" Value of the field"]
pub struct REPNBR {
    bits: u8,
}
impl REPNBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `EPBK`"]
pub enum EPBKW {
    #[doc = "`0`"]
    SINGLE,
    #[doc = "`1`"]
    DOUBLE,
}
impl EPBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPBKW::SINGLE => false,
            EPBKW::DOUBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPBKW<'a> {
    w: &'a mut W,
}
impl<'a> _EPBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPBKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(EPBKW::SINGLE)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn double(self) -> &'a mut W {
        self.variant(EPBKW::DOUBLE)
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
#[doc = "Values that can be written to the field `EPSIZE`"]
pub enum EPSIZEW {
    #[doc = "`0`"]
    _8,
    #[doc = "`1`"]
    _16,
    #[doc = "`10`"]
    _32,
    #[doc = "`11`"]
    _64,
    #[doc = "`100`"]
    _128,
    #[doc = "`101`"]
    _256,
    #[doc = "`110`"]
    _512,
    #[doc = "`111`"]
    _1024,
}
impl EPSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPSIZEW::_8 => 0,
            EPSIZEW::_16 => 1,
            EPSIZEW::_32 => 2,
            EPSIZEW::_64 => 3,
            EPSIZEW::_128 => 4,
            EPSIZEW::_256 => 5,
            EPSIZEW::_512 => 6,
            EPSIZEW::_1024 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(EPSIZEW::_8)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(EPSIZEW::_16)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(EPSIZEW::_32)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(EPSIZEW::_64)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(EPSIZEW::_128)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(EPSIZEW::_256)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(EPSIZEW::_512)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(EPSIZEW::_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPDIR`"]
pub enum EPDIRW {
    #[doc = "`0`"]
    OUT,
    #[doc = "`1`"]
    IN,
}
impl EPDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPDIRW::OUT => false,
            EPDIRW::IN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(EPDIRW::OUT)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(EPDIRW::IN)
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
#[doc = "Values that can be written to the field `EPTYPE`"]
pub enum EPTYPEW {
    #[doc = "`0`"]
    CONTROL,
    #[doc = "`1`"]
    ISOCHRONOUS,
    #[doc = "`10`"]
    BULK,
    #[doc = "`11`"]
    INTERRUPT,
}
impl EPTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPTYPEW::CONTROL => 0,
            EPTYPEW::ISOCHRONOUS => 1,
            EPTYPEW::BULK => 2,
            EPTYPEW::INTERRUPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(EPTYPEW::CONTROL)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(EPTYPEW::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPTYPEW::BULK)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EPTYPEW::INTERRUPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REPNBW<'a> {
    w: &'a mut W,
}
impl<'a> _REPNBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 2 - Endpoint Bank"]
    #[inline]
    pub fn epbk(&self) -> EPBKR {
        EPBKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline]
    pub fn epsize(&self) -> EPSIZER {
        EPSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&self) -> EPDIRR {
        EPDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline]
    pub fn eptype(&self) -> EPTYPER {
        EPTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Redirected Endpoint Number"]
    #[inline]
    pub fn repnb(&self) -> REPNBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REPNBR { bits }
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
    #[doc = "Bit 2 - Endpoint Bank"]
    #[inline]
    pub fn epbk(&mut self) -> _EPBKW {
        _EPBKW { w: self }
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline]
    pub fn epsize(&mut self) -> _EPSIZEW {
        _EPSIZEW { w: self }
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&mut self) -> _EPDIRW {
        _EPDIRW { w: self }
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline]
    pub fn eptype(&mut self) -> _EPTYPEW {
        _EPTYPEW { w: self }
    }
    #[doc = "Bits 16:19 - Redirected Endpoint Number"]
    #[inline]
    pub fn repnb(&mut self) -> _REPNBW {
        _REPNBW { w: self }
    }
}
