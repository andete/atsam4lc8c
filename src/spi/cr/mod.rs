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
#[doc = "Values that can be written to the field `SPIEN`"]
pub enum SPIENW {
    #[doc = "No effect."]
    _0,
    #[doc = "Enables the SPI to transfer and receive data."]
    _1,
}
impl SPIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPIENW::_0 => false,
            SPIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIENW::_0)
    }
    #[doc = "Enables the SPI to transfer and receive data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIENW::_1)
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
#[doc = "Values that can be written to the field `SPIDIS`"]
pub enum SPIDISW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the SPI.All pins are set in input mode and no data is received or transmitted.If a transfer is in progress, the transfer is finished before the SPI is disabled.If both SPIEN and SPIDIS are equal to one when the control register is written, the SPI is disabled."]
    _1,
}
impl SPIDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPIDISW::_0 => false,
            SPIDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIDISW::_0)
    }
    #[doc = "Disables the SPI.All pins are set in input mode and no data is received or transmitted.If a transfer is in progress, the transfer is finished before the SPI is disabled.If both SPIEN and SPIDIS are equal to one when the control register is written, the SPI is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIDISW::_1)
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
#[doc = "Values that can be written to the field `SWRST`"]
pub enum SWRSTW {
    #[doc = "No effect."]
    _0,
    #[doc = "Reset the SPI. A software-triggered hardware reset of the SPI interface is performed."]
    _1,
}
impl SWRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTW::_0 => false,
            SWRSTW::_1 => true,
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
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTW::_0)
    }
    #[doc = "Reset the SPI. A software-triggered hardware reset of the SPI interface is performed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTW::_1)
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
#[doc = r" Proxy"]
pub struct _FLUSHFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _FLUSHFIFOW<'a> {
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
#[doc = "Values that can be written to the field `LASTXFER`"]
pub enum LASTXFERW {
    #[doc = "No effect."]
    _0,
    #[doc = "The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    _1,
}
impl LASTXFERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LASTXFERW::_0 => false,
            LASTXFERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LASTXFERW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTXFERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LASTXFERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LASTXFERW::_0)
    }
    #[doc = "The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LASTXFERW::_1)
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
    #[doc = "Bit 0 - SPI Enable"]
    #[inline]
    pub fn spien(&mut self) -> _SPIENW {
        _SPIENW { w: self }
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline]
    pub fn spidis(&mut self) -> _SPIDISW {
        _SPIDISW { w: self }
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 8 - Flush FIFO command"]
    #[inline]
    pub fn flushfifo(&mut self) -> _FLUSHFIFOW {
        _FLUSHFIFOW { w: self }
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline]
    pub fn lastxfer(&mut self) -> _LASTXFERW {
        _LASTXFERW { w: self }
    }
}
