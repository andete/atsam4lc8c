#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UPCFG1 {
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
#[doc = "Possible values of the field `PBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBKR {
    #[doc = "undocumented"]
    SINGLE,
    #[doc = "undocumented"]
    DOUBLE,
}
impl PBKR {
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
            PBKR::SINGLE => false,
            PBKR::DOUBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBKR {
        match value {
            false => PBKR::SINGLE,
            true => PBKR::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == PBKR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline]
    pub fn is_double(&self) -> bool {
        *self == PBKR::DOUBLE
    }
}
#[doc = "Possible values of the field `PSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSIZER {
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
impl PSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSIZER::_8 => 0,
            PSIZER::_16 => 1,
            PSIZER::_32 => 2,
            PSIZER::_64 => 3,
            PSIZER::_128 => 4,
            PSIZER::_256 => 5,
            PSIZER::_512 => 6,
            PSIZER::_1024 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSIZER {
        match value {
            0 => PSIZER::_8,
            1 => PSIZER::_16,
            2 => PSIZER::_32,
            3 => PSIZER::_64,
            4 => PSIZER::_128,
            5 => PSIZER::_256,
            6 => PSIZER::_512,
            7 => PSIZER::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PSIZER::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PSIZER::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PSIZER::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PSIZER::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == PSIZER::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == PSIZER::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == PSIZER::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == PSIZER::_1024
    }
}
#[doc = "Possible values of the field `PTOKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTOKENR {
    #[doc = "undocumented"]
    SETUP,
    #[doc = "undocumented"]
    IN,
    #[doc = "undocumented"]
    OUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTOKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTOKENR::SETUP => 0,
            PTOKENR::IN => 1,
            PTOKENR::OUT => 2,
            PTOKENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTOKENR {
        match value {
            0 => PTOKENR::SETUP,
            1 => PTOKENR::IN,
            2 => PTOKENR::OUT,
            i => PTOKENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline]
    pub fn is_setup(&self) -> bool {
        *self == PTOKENR::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == PTOKENR::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == PTOKENR::OUT
    }
}
#[doc = "Possible values of the field `PTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYPER {
    #[doc = "undocumented"]
    CONTROL,
    #[doc = "undocumented"]
    ISOCHRONOUS,
    #[doc = "undocumented"]
    BULK,
    #[doc = "undocumented"]
    INTERRUPT,
}
impl PTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTYPER::CONTROL => 0,
            PTYPER::ISOCHRONOUS => 1,
            PTYPER::BULK => 2,
            PTYPER::INTERRUPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTYPER {
        match value {
            0 => PTYPER::CONTROL,
            1 => PTYPER::ISOCHRONOUS,
            2 => PTYPER::BULK,
            3 => PTYPER::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == PTYPER::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline]
    pub fn is_isochronous(&self) -> bool {
        *self == PTYPER::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == PTYPER::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PTYPER::INTERRUPT
    }
}
#[doc = r" Value of the field"]
pub struct PINGENR {
    bits: bool,
}
impl PINGENR {
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
#[doc = r" Value of the field"]
pub struct BINTERVALR {
    bits: u8,
}
impl BINTERVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PBK`"]
pub enum PBKW {
    #[doc = "`0`"]
    SINGLE,
    #[doc = "`1`"]
    DOUBLE,
}
impl PBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBKW::SINGLE => false,
            PBKW::DOUBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBKW<'a> {
    w: &'a mut W,
}
impl<'a> _PBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(PBKW::SINGLE)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn double(self) -> &'a mut W {
        self.variant(PBKW::DOUBLE)
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
#[doc = "Values that can be written to the field `PSIZE`"]
pub enum PSIZEW {
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
impl PSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSIZEW::_8 => 0,
            PSIZEW::_16 => 1,
            PSIZEW::_32 => 2,
            PSIZEW::_64 => 3,
            PSIZEW::_128 => 4,
            PSIZEW::_256 => 5,
            PSIZEW::_512 => 6,
            PSIZEW::_1024 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSIZEW::_8)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSIZEW::_16)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSIZEW::_32)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSIZEW::_64)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSIZEW::_128)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSIZEW::_256)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSIZEW::_512)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSIZEW::_1024)
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
#[doc = "Values that can be written to the field `PTOKEN`"]
pub enum PTOKENW {
    #[doc = "`0`"]
    SETUP,
    #[doc = "`1`"]
    IN,
    #[doc = "`10`"]
    OUT,
}
impl PTOKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTOKENW::SETUP => 0,
            PTOKENW::IN => 1,
            PTOKENW::OUT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTOKENW<'a> {
    w: &'a mut W,
}
impl<'a> _PTOKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTOKENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKENW::SETUP)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKENW::IN)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKENW::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTYPE`"]
pub enum PTYPEW {
    #[doc = "`0`"]
    CONTROL,
    #[doc = "`1`"]
    ISOCHRONOUS,
    #[doc = "`10`"]
    BULK,
    #[doc = "`11`"]
    INTERRUPT,
}
impl PTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTYPEW::CONTROL => 0,
            PTYPEW::ISOCHRONOUS => 1,
            PTYPEW::BULK => 2,
            PTYPEW::INTERRUPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _PTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(PTYPEW::CONTROL)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(PTYPEW::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(PTYPEW::BULK)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PTYPEW::INTERRUPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _PINGENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BINTERVALW<'a> {
    w: &'a mut W,
}
impl<'a> _BINTERVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 2 - Pipe Banks"]
    #[inline]
    pub fn pbk(&self) -> PBKR {
        PBKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline]
    pub fn psize(&self) -> PSIZER {
        PSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline]
    pub fn ptoken(&self) -> PTOKENR {
        PTOKENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline]
    pub fn ptype(&self) -> PTYPER {
        PTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline]
    pub fn pingen(&self) -> PINGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINGENR { bits }
    }
    #[doc = "Bits 24:31 - binterval parameter"]
    #[inline]
    pub fn binterval(&self) -> BINTERVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BINTERVALR { bits }
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
    #[doc = "Bit 2 - Pipe Banks"]
    #[inline]
    pub fn pbk(&mut self) -> _PBKW {
        _PBKW { w: self }
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline]
    pub fn psize(&mut self) -> _PSIZEW {
        _PSIZEW { w: self }
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline]
    pub fn ptoken(&mut self) -> _PTOKENW {
        _PTOKENW { w: self }
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline]
    pub fn ptype(&mut self) -> _PTYPEW {
        _PTYPEW { w: self }
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline]
    pub fn pingen(&mut self) -> _PINGENW {
        _PINGENW { w: self }
    }
    #[doc = "Bits 24:31 - binterval parameter"]
    #[inline]
    pub fn binterval(&mut self) -> _BINTERVALW {
        _BINTERVALW { w: self }
    }
}
