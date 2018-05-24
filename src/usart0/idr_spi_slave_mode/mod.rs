#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDR_SPI_SLAVE_MODE {
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
}
#[doc = "Values that can be written to the field `RXRDY`"]
pub enum RXRDYW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl RXRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXRDYW::_0 => false,
            RXRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRDYW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRDYW::_1)
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
#[doc = "Values that can be written to the field `TXRDY`"]
pub enum TXRDYW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl TXRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRDYW::_0 => false,
            TXRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRDYW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRDYW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXBRK`"]
pub enum RXBRKW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl RXBRKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXBRKW::_0 => false,
            RXBRKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXBRKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBRKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXBRKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRKW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRKW::_1)
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
#[doc = "Values that can be written to the field `OVRE`"]
pub enum OVREW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl OVREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVREW::_0 => false,
            OVREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVREW<'a> {
    w: &'a mut W,
}
impl<'a> _OVREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVREW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVREW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRAME`"]
pub enum FRAMEW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl FRAMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAMEW::_0 => false,
            FRAMEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRAMEW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRAMEW::_1)
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
#[doc = "Values that can be written to the field `PARE`"]
pub enum PAREW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl PAREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAREW::_0 => false,
            PAREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAREW<'a> {
    w: &'a mut W,
}
impl<'a> _PAREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAREW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAREW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMEOUT`"]
pub enum TIMEOUTW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMEOUTW::_0 => false,
            TIMEOUTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMEOUTW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMEOUTW::_1)
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
#[doc = "Values that can be written to the field `TXEMPTY`"]
pub enum TXEMPTYW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl TXEMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEMPTYW::_0 => false,
            TXEMPTYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEMPTYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTYW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEMPTYW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UNRE`"]
pub enum UNREW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl UNREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNREW::_0 => false,
            UNREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNREW<'a> {
    w: &'a mut W,
}
impl<'a> _UNREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNREW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNREW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXBUFE`"]
pub enum TXBUFEW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl TXBUFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXBUFEW::_0 => false,
            TXBUFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXBUFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBUFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXBUFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFEW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXBUFEW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXBUFF`"]
pub enum RXBUFFW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl RXBUFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXBUFFW::_0 => false,
            RXBUFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXBUFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBUFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXBUFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFFW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBUFFW::_1)
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
#[doc = "Values that can be written to the field `NACK`"]
pub enum NACKW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl NACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKW::_0 => false,
            NACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIIC`"]
pub enum RIICW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl RIICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIICW::_0 => false,
            RIICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIICW<'a> {
    w: &'a mut W,
}
impl<'a> _RIICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIICW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIICW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSRIC`"]
pub enum DSRICW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl DSRICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSRICW::_0 => false,
            DSRICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSRICW<'a> {
    w: &'a mut W,
}
impl<'a> _DSRICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSRICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSRICW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSRICW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDIC`"]
pub enum DCDICW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl DCDICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCDICW::_0 => false,
            DCDICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDICW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDICW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCDICW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTSIC`"]
pub enum CTSICW {
    #[doc = "No Effect"]
    _0,
    #[doc = "Disables the interrupt"]
    _1,
}
impl CTSICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSICW::_0 => false,
            CTSICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSICW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSICW::_0)
    }
    #[doc = "Disables the interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSICW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - Receiver Ready Interrupt Disable"]
    #[inline]
    pub fn rxrdy(&mut self) -> _RXRDYW {
        _RXRDYW { w: self }
    }
    #[doc = "Bit 1 - Transmitter Ready Interrupt Disable"]
    #[inline]
    pub fn txrdy(&mut self) -> _TXRDYW {
        _TXRDYW { w: self }
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline]
    pub fn rxbrk(&mut self) -> _RXBRKW {
        _RXBRKW { w: self }
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline]
    pub fn ovre(&mut self) -> _OVREW {
        _OVREW { w: self }
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline]
    pub fn frame(&mut self) -> _FRAMEW {
        _FRAMEW { w: self }
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline]
    pub fn pare(&mut self) -> _PAREW {
        _PAREW { w: self }
    }
    #[doc = "Bit 8 - Time-out Interrupt Disable"]
    #[inline]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
    #[doc = "Bit 9 - Transmitter Empty Interrupt Disable"]
    #[inline]
    pub fn txempty(&mut self) -> _TXEMPTYW {
        _TXEMPTYW { w: self }
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Disable"]
    #[inline]
    pub fn unre(&mut self) -> _UNREW {
        _UNREW { w: self }
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Disable"]
    #[inline]
    pub fn txbufe(&mut self) -> _TXBUFEW {
        _TXBUFEW { w: self }
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Disable"]
    #[inline]
    pub fn rxbuff(&mut self) -> _RXBUFFW {
        _RXBUFFW { w: self }
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Disable"]
    #[inline]
    pub fn nack(&mut self) -> _NACKW {
        _NACKW { w: self }
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline]
    pub fn riic(&mut self) -> _RIICW {
        _RIICW { w: self }
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline]
    pub fn dsric(&mut self) -> _DSRICW {
        _DSRICW { w: self }
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline]
    pub fn dcdic(&mut self) -> _DCDICW {
        _DCDICW { w: self }
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline]
    pub fn ctsic(&mut self) -> _CTSICW {
        _CTSICW { w: self }
    }
}
