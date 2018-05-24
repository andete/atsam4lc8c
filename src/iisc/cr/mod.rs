#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Values that can be written to the field `RXEN`"]
pub enum RXENW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables Data Receive if RXDIS is not set"]
    ON,
}
impl RXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXENW::OFF => false,
            RXENW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(RXENW::OFF)
    }
    #[doc = "Enables Data Receive if RXDIS is not set"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(RXENW::ON)
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
#[doc = "Values that can be written to the field `RXDIS`"]
pub enum RXDISW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Disables Data Receive"]
    ON,
}
impl RXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDISW::OFF => false,
            RXDISW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(RXDISW::OFF)
    }
    #[doc = "Disables Data Receive"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(RXDISW::ON)
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
#[doc = "Values that can be written to the field `CKEN`"]
pub enum CKENW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables clocks if CKDIS is not set"]
    ON,
}
impl CKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CKENW::OFF => false,
            CKENW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(CKENW::OFF)
    }
    #[doc = "Enables clocks if CKDIS is not set"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(CKENW::ON)
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
#[doc = "Values that can be written to the field `CKDIS`"]
pub enum CKDISW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Disables clocks"]
    ON,
}
impl CKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CKDISW::OFF => false,
            CKDISW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CKDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(CKDISW::OFF)
    }
    #[doc = "Disables clocks"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(CKDISW::ON)
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
#[doc = "Values that can be written to the field `TXEN`"]
pub enum TXENW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Enables Data Transmit if TXDIS is not set"]
    ON,
}
impl TXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXENW::OFF => false,
            TXENW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(TXENW::OFF)
    }
    #[doc = "Enables Data Transmit if TXDIS is not set"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(TXENW::ON)
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
#[doc = "Values that can be written to the field `TXDIS`"]
pub enum TXDISW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Disables Data Transmit"]
    ON,
}
impl TXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDISW::OFF => false,
            TXDISW::ON => true,
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
    pub fn off(self) -> &'a mut W {
        self.variant(TXDISW::OFF)
    }
    #[doc = "Disables Data Transmit"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(TXDISW::ON)
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
#[doc = "Values that can be written to the field `SWRST`"]
pub enum SWRSTW {
    #[doc = "No effect"]
    OFF,
    #[doc = "Performs a software reset. Has priority on any other bit in CR"]
    ON,
}
impl SWRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTW::OFF => false,
            SWRSTW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(SWRSTW::OFF)
    }
    #[doc = "Performs a software reset. Has priority on any other bit in CR"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(SWRSTW::ON)
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
    #[doc = "Bit 0 - Receive Enable"]
    #[inline]
    pub fn rxen(&mut self) -> _RXENW {
        _RXENW { w: self }
    }
    #[doc = "Bit 1 - Receive Disable"]
    #[inline]
    pub fn rxdis(&mut self) -> _RXDISW {
        _RXDISW { w: self }
    }
    #[doc = "Bit 2 - Clocks Enable"]
    #[inline]
    pub fn cken(&mut self) -> _CKENW {
        _CKENW { w: self }
    }
    #[doc = "Bit 3 - Clocks Disable"]
    #[inline]
    pub fn ckdis(&mut self) -> _CKDISW {
        _CKDISW { w: self }
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline]
    pub fn txen(&mut self) -> _TXENW {
        _TXENW { w: self }
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline]
    pub fn txdis(&mut self) -> _TXDISW {
        _TXDISW { w: self }
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
}
