#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR_SPI_MASTER_MODE {
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
#[doc = "Values that can be written to the field `RSTRX`"]
pub enum RSTRXW {
    #[doc = "No effect"]
    _0,
    #[doc = "Resets the receiver"]
    _1,
}
impl RSTRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTRXW::_0 => false,
            RSTRXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTRXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTRXW::_0)
    }
    #[doc = "Resets the receiver"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTRXW::_1)
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
#[doc = "Values that can be written to the field `RSTTX`"]
pub enum RSTTXW {
    #[doc = "No effect"]
    _0,
    #[doc = "Resets the transmitter"]
    _1,
}
impl RSTTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTTXW::_0 => false,
            RSTTXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTTXW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTTXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTTXW::_0)
    }
    #[doc = "Resets the transmitter"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTTXW::_1)
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
#[doc = "Values that can be written to the field `RXEN`"]
pub enum RXENW {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the receiver, if RXDIS is 0"]
    _1,
}
impl RXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXENW::_0 => false,
            RXENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXENW::_0)
    }
    #[doc = "Enables the receiver, if RXDIS is 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXENW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDIS`"]
pub enum RXDISW {
    #[doc = "No effect"]
    _0,
    #[doc = "Disables the receiver"]
    _1,
}
impl RXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDISW::_0 => false,
            RXDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDISW::_0)
    }
    #[doc = "Disables the receiver"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDISW::_1)
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
#[doc = "Values that can be written to the field `TXEN`"]
pub enum TXENW {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the transmitter if TXDIS is 0"]
    _1,
}
impl TXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXENW::_0 => false,
            TXENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXENW::_0)
    }
    #[doc = "Enables the transmitter if TXDIS is 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXENW::_1)
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
#[doc = "Values that can be written to the field `TXDIS`"]
pub enum TXDISW {
    #[doc = "No effect"]
    _0,
    #[doc = "Disables the transmitter"]
    _1,
}
impl TXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDISW::_0 => false,
            TXDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDISW::_0)
    }
    #[doc = "Disables the transmitter"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDISW::_1)
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
#[doc = "Values that can be written to the field `RSTSTA`"]
pub enum RSTSTAW {
    #[doc = "No effect"]
    _0,
    #[doc = "Resets the status bits PARE, FRAME, OVRE and RXBRK in the CSR"]
    _1,
}
impl RSTSTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTSTAW::_0 => false,
            RSTSTAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTSTAW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTSTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTSTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTSTAW::_0)
    }
    #[doc = "Resets the status bits PARE, FRAME, OVRE and RXBRK in the CSR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTSTAW::_1)
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
#[doc = "Values that can be written to the field `STTBRK`"]
pub enum STTBRKW {
    #[doc = "No effect"]
    _0,
    #[doc = "Starts transmission of a break after the characters present in THR and the Transmit Shift Register have been transmitted. No effect if a break is already being transmitted"]
    _1,
}
impl STTBRKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STTBRKW::_0 => false,
            STTBRKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STTBRKW<'a> {
    w: &'a mut W,
}
impl<'a> _STTBRKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STTBRKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTBRKW::_0)
    }
    #[doc = "Starts transmission of a break after the characters present in THR and the Transmit Shift Register have been transmitted. No effect if a break is already being transmitted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTBRKW::_1)
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
#[doc = "Values that can be written to the field `STPBRK`"]
pub enum STPBRKW {
    #[doc = "No effect"]
    _0,
    #[doc = "Stops transmission of the break after a minimum of one character length and transmits a high level during 12-bit periods.No effect if no break is being transmitted"]
    _1,
}
impl STPBRKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPBRKW::_0 => false,
            STPBRKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPBRKW<'a> {
    w: &'a mut W,
}
impl<'a> _STPBRKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPBRKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPBRKW::_0)
    }
    #[doc = "Stops transmission of the break after a minimum of one character length and transmits a high level during 12-bit periods.No effect if no break is being transmitted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPBRKW::_1)
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
#[doc = "Values that can be written to the field `STTTO`"]
pub enum STTTOW {
    #[doc = "No effect"]
    _0,
    #[doc = "Starts waiting for a character before clocking the time-out counter"]
    _1,
}
impl STTTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STTTOW::_0 => false,
            STTTOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STTTOW<'a> {
    w: &'a mut W,
}
impl<'a> _STTTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STTTOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STTTOW::_0)
    }
    #[doc = "Starts waiting for a character before clocking the time-out counter"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STTTOW::_1)
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
#[doc = "Values that can be written to the field `SENDA`"]
pub enum SENDAW {
    #[doc = "No effect"]
    _0,
    #[doc = "In Multi-drop Mode only, the next character written to the THR is sent with the address bit set"]
    _1,
}
impl SENDAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SENDAW::_0 => false,
            SENDAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENDAW<'a> {
    w: &'a mut W,
}
impl<'a> _SENDAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENDAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENDAW::_0)
    }
    #[doc = "In Multi-drop Mode only, the next character written to the THR is sent with the address bit set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENDAW::_1)
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
#[doc = "Values that can be written to the field `RSTIT`"]
pub enum RSTITW {
    #[doc = "No effect"]
    _0,
    #[doc = "Resets ITERATION in CSR. No effect if the ISO7816 is not enabled"]
    _1,
}
impl RSTITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTITW::_0 => false,
            RSTITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTITW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTITW::_0)
    }
    #[doc = "Resets ITERATION in CSR. No effect if the ISO7816 is not enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTITW::_1)
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
#[doc = "Values that can be written to the field `RSTNACK`"]
pub enum RSTNACKW {
    #[doc = "No effect"]
    _0,
    #[doc = "Resets NACK in CSR"]
    _1,
}
impl RSTNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTNACKW::_0 => false,
            RSTNACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTNACKW::_0)
    }
    #[doc = "Resets NACK in CSR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTNACKW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RETTO`"]
pub enum RETTOW {
    #[doc = "No effect"]
    _0,
    #[doc = "Restart Time-out"]
    _1,
}
impl RETTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RETTOW::_0 => false,
            RETTOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RETTOW<'a> {
    w: &'a mut W,
}
impl<'a> _RETTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RETTOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RETTOW::_0)
    }
    #[doc = "Restart Time-out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RETTOW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTREN`"]
pub enum DTRENW {
    #[doc = "No effect"]
    _0,
    #[doc = "Drives the pin DTR at 0"]
    _1,
}
impl DTRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTRENW::_0 => false,
            DTRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTRENW::_0)
    }
    #[doc = "Drives the pin DTR at 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTRENW::_1)
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
#[doc = "Values that can be written to the field `DTRDIS`"]
pub enum DTRDISW {
    #[doc = "No effect"]
    _0,
    #[doc = "Drives the pin DTR to 1"]
    _1,
}
impl DTRDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTRDISW::_0 => false,
            DTRDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTRDISW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTRDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTRDISW::_0)
    }
    #[doc = "Drives the pin DTR to 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTRDISW::_1)
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
#[doc = "Values that can be written to the field `FCS`"]
pub enum FCSW {
    #[doc = "No effect"]
    _0,
    #[doc = "Forces the Slave Select Line NSS (RTS pin) to 0, even if USART is no transmitting, in order to address SPI slave devices supporting the CSAAT Mode (Chip Select Active After Transfer)"]
    _1,
}
impl FCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCSW::_0 => false,
            FCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCSW<'a> {
    w: &'a mut W,
}
impl<'a> _FCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCSW::_0)
    }
    #[doc = "Forces the Slave Select Line NSS (RTS pin) to 0, even if USART is no transmitting, in order to address SPI slave devices supporting the CSAAT Mode (Chip Select Active After Transfer)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCSW::_1)
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
#[doc = "Values that can be written to the field `RCS`"]
pub enum RCSW {
    #[doc = "No effect"]
    _0,
    #[doc = "Releases the Slave Select Line NSS (RTS pin)"]
    _1,
}
impl RCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCSW::_0 => false,
            RCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCSW<'a> {
    w: &'a mut W,
}
impl<'a> _RCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCSW::_0)
    }
    #[doc = "Releases the Slave Select Line NSS (RTS pin)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCSW::_1)
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
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline]
    pub fn rstrx(&mut self) -> _RSTRXW {
        _RSTRXW { w: self }
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline]
    pub fn rsttx(&mut self) -> _RSTTXW {
        _RSTTXW { w: self }
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline]
    pub fn rxen(&mut self) -> _RXENW {
        _RXENW { w: self }
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline]
    pub fn rxdis(&mut self) -> _RXDISW {
        _RXDISW { w: self }
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline]
    pub fn txen(&mut self) -> _TXENW {
        _TXENW { w: self }
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline]
    pub fn txdis(&mut self) -> _TXDISW {
        _TXDISW { w: self }
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline]
    pub fn rststa(&mut self) -> _RSTSTAW {
        _RSTSTAW { w: self }
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline]
    pub fn sttbrk(&mut self) -> _STTBRKW {
        _STTBRKW { w: self }
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline]
    pub fn stpbrk(&mut self) -> _STPBRKW {
        _STPBRKW { w: self }
    }
    #[doc = "Bit 11 - Start Time-out"]
    #[inline]
    pub fn sttto(&mut self) -> _STTTOW {
        _STTTOW { w: self }
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline]
    pub fn senda(&mut self) -> _SENDAW {
        _SENDAW { w: self }
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline]
    pub fn rstit(&mut self) -> _RSTITW {
        _RSTITW { w: self }
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline]
    pub fn rstnack(&mut self) -> _RSTNACKW {
        _RSTNACKW { w: self }
    }
    #[doc = "Bit 15 - Rearm Time-out"]
    #[inline]
    pub fn retto(&mut self) -> _RETTOW {
        _RETTOW { w: self }
    }
    #[doc = "Bit 16 - Data Terminal Ready Enable"]
    #[inline]
    pub fn dtren(&mut self) -> _DTRENW {
        _DTRENW { w: self }
    }
    #[doc = "Bit 17 - Data Terminal Ready Disable"]
    #[inline]
    pub fn dtrdis(&mut self) -> _DTRDISW {
        _DTRDISW { w: self }
    }
    #[doc = "Bit 18 - Force SPI Chip Select"]
    #[inline]
    pub fn fcs(&mut self) -> _FCSW {
        _FCSW { w: self }
    }
    #[doc = "Bit 19 - Release SPI Chip Select"]
    #[inline]
    pub fn rcs(&mut self) -> _RCSW {
        _RCSW { w: self }
    }
}
