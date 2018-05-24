#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAN {
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
#[doc = "Possible values of the field `TX_PL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_PLR {
    #[doc = "The Transmitter Preamble pattern generation is disabled"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TX_PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_PLR::_0 => 0,
            TX_PLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_PLR {
        match value {
            0 => TX_PLR::_0,
            i => TX_PLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_PLR::_0
    }
}
#[doc = "Possible values of the field `TX_PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_PPR {
    #[doc = "ALL_ONE"]
    _0,
    #[doc = "ALL_ZERO"]
    _1,
    #[doc = "ZERO_ONE"]
    _2,
    #[doc = "ONE_ZERO"]
    _3,
}
impl TX_PPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_PPR::_0 => 0,
            TX_PPR::_1 => 1,
            TX_PPR::_2 => 2,
            TX_PPR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_PPR {
        match value {
            0 => TX_PPR::_0,
            1 => TX_PPR::_1,
            2 => TX_PPR::_2,
            3 => TX_PPR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_PPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_PPR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == TX_PPR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == TX_PPR::_3
    }
}
#[doc = "Possible values of the field `TX_MPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_MPOLR {
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0,
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1,
}
impl TX_MPOLR {
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
            TX_MPOLR::_0 => false,
            TX_MPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_MPOLR {
        match value {
            false => TX_MPOLR::_0,
            true => TX_MPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_MPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_MPOLR::_1
    }
}
#[doc = "Possible values of the field `RX_PL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PLR {
    #[doc = "The receiver preamble pattern detection is disabled"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_PLR::_0 => 0,
            RX_PLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_PLR {
        match value {
            0 => RX_PLR::_0,
            i => RX_PLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_PLR::_0
    }
}
#[doc = "Possible values of the field `RX_PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PPR {
    #[doc = "ALL_ONE"]
    _0,
    #[doc = "ALL_ZERO"]
    _1,
    #[doc = "ZERO_ONE"]
    _2,
    #[doc = "ONE_ZERO"]
    _3,
}
impl RX_PPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_PPR::_0 => 0,
            RX_PPR::_1 => 1,
            RX_PPR::_2 => 2,
            RX_PPR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_PPR {
        match value {
            0 => RX_PPR::_0,
            1 => RX_PPR::_1,
            2 => RX_PPR::_2,
            3 => RX_PPR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_PPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_PPR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RX_PPR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RX_PPR::_3
    }
}
#[doc = "Possible values of the field `RX_MPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MPOLR {
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0,
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1,
}
impl RX_MPOLR {
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
            RX_MPOLR::_0 => false,
            RX_MPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_MPOLR {
        match value {
            false => RX_MPOLR::_0,
            true => RX_MPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_MPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_MPOLR::_1
    }
}
#[doc = "Possible values of the field `DRIFT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIFTR {
    #[doc = "The USART can not recover from an important clock drift"]
    _0,
    #[doc = "The USART can recover from clock drift. The 16X clock mode must be enabled"]
    _1,
}
impl DRIFTR {
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
            DRIFTR::_0 => false,
            DRIFTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRIFTR {
        match value {
            false => DRIFTR::_0,
            true => DRIFTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DRIFTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DRIFTR::_1
    }
}
#[doc = "Values that can be written to the field `TX_PL`"]
pub enum TX_PLW {
    #[doc = "The Transmitter Preamble pattern generation is disabled"]
    _0,
}
impl TX_PLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_PLW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_PLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_PLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The Transmitter Preamble pattern generation is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PLW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_PP`"]
pub enum TX_PPW {
    #[doc = "ALL_ONE"]
    _0,
    #[doc = "ALL_ZERO"]
    _1,
    #[doc = "ZERO_ONE"]
    _2,
    #[doc = "ONE_ZERO"]
    _3,
}
impl TX_PPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_PPW::_0 => 0,
            TX_PPW::_1 => 1,
            TX_PPW::_2 => 2,
            TX_PPW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_PPW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_PPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ALL_ONE"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PPW::_0)
    }
    #[doc = "ALL_ZERO"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_PPW::_1)
    }
    #[doc = "ZERO_ONE"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_PPW::_2)
    }
    #[doc = "ONE_ZERO"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_PPW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_MPOL`"]
pub enum TX_MPOLW {
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0,
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1,
}
impl TX_MPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_MPOLW::_0 => false,
            TX_MPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_MPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_MPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_MPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_MPOLW::_0)
    }
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_MPOLW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_PL`"]
pub enum RX_PLW {
    #[doc = "The receiver preamble pattern detection is disabled"]
    _0,
}
impl RX_PLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_PLW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_PLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_PLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The receiver preamble pattern detection is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PLW::_0)
    }
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
#[doc = "Values that can be written to the field `RX_PP`"]
pub enum RX_PPW {
    #[doc = "ALL_ONE"]
    _0,
    #[doc = "ALL_ZERO"]
    _1,
    #[doc = "ZERO_ONE"]
    _2,
    #[doc = "ONE_ZERO"]
    _3,
}
impl RX_PPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_PPW::_0 => 0,
            RX_PPW::_1 => 1,
            RX_PPW::_2 => 2,
            RX_PPW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_PPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_PPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ALL_ONE"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PPW::_0)
    }
    #[doc = "ALL_ZERO"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_PPW::_1)
    }
    #[doc = "ZERO_ONE"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_PPW::_2)
    }
    #[doc = "ONE_ZERO"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_PPW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_MPOL`"]
pub enum RX_MPOLW {
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0,
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1,
}
impl RX_MPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_MPOLW::_0 => false,
            RX_MPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_MPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_MPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_MPOLW::_0)
    }
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_MPOLW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRIFT`"]
pub enum DRIFTW {
    #[doc = "The USART can not recover from an important clock drift"]
    _0,
    #[doc = "The USART can recover from clock drift. The 16X clock mode must be enabled"]
    _1,
}
impl DRIFTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRIFTW::_0 => false,
            DRIFTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRIFTW<'a> {
    w: &'a mut W,
}
impl<'a> _DRIFTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRIFTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The USART can not recover from an important clock drift"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRIFTW::_0)
    }
    #[doc = "The USART can recover from clock drift. The 16X clock mode must be enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRIFTW::_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline]
    pub fn tx_pl(&self) -> TX_PLR {
        TX_PLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline]
    pub fn tx_pp(&self) -> TX_PPR {
        TX_PPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline]
    pub fn tx_mpol(&self) -> TX_MPOLR {
        TX_MPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline]
    pub fn rx_pl(&self) -> RX_PLR {
        RX_PLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline]
    pub fn rx_pp(&self) -> RX_PPR {
        RX_PPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline]
    pub fn rx_mpol(&self) -> RX_MPOLR {
        RX_MPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Drift compensation"]
    #[inline]
    pub fn drift(&self) -> DRIFTR {
        DRIFTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 805376004 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline]
    pub fn tx_pl(&mut self) -> _TX_PLW {
        _TX_PLW { w: self }
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline]
    pub fn tx_pp(&mut self) -> _TX_PPW {
        _TX_PPW { w: self }
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline]
    pub fn tx_mpol(&mut self) -> _TX_MPOLW {
        _TX_MPOLW { w: self }
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline]
    pub fn rx_pl(&mut self) -> _RX_PLW {
        _RX_PLW { w: self }
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline]
    pub fn rx_pp(&mut self) -> _RX_PPW {
        _RX_PPW { w: self }
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline]
    pub fn rx_mpol(&mut self) -> _RX_MPOLW {
        _RX_MPOLW { w: self }
    }
    #[doc = "Bit 30 - Drift compensation"]
    #[inline]
    pub fn drift(&mut self) -> _DRIFTW {
        _DRIFTW { w: self }
    }
}
