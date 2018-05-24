#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDR {
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
#[doc = "Values that can be written to the field `RDRF`"]
pub enum RDRFW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl RDRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDRFW::_0 => false,
            RDRFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDRFW<'a> {
    w: &'a mut W,
}
impl<'a> _RDRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDRFW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDRFW::_1)
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
#[doc = "Values that can be written to the field `TDRE`"]
pub enum TDREW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl TDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDREW::_0 => false,
            TDREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDREW<'a> {
    w: &'a mut W,
}
impl<'a> _TDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDREW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDREW::_1)
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
#[doc = "Values that can be written to the field `MODF`"]
pub enum MODFW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl MODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODFW::_0 => false,
            MODFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODFW<'a> {
    w: &'a mut W,
}
impl<'a> _MODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFW::_1)
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
#[doc = "Values that can be written to the field `OVRES`"]
pub enum OVRESW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl OVRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRESW::_0 => false,
            OVRESW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRESW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRESW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRESW::_1)
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
#[doc = "Values that can be written to the field `ENDRX`"]
pub enum ENDRXW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl ENDRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDRXW::_0 => false,
            ENDRXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDRXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDRXW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDRXW::_1)
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
#[doc = "Values that can be written to the field `ENDTX`"]
pub enum ENDTXW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl ENDTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDTXW::_0 => false,
            ENDTXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDTXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDTXW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDTXW::_1)
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
#[doc = "Values that can be written to the field `RXBUFF`"]
pub enum RXBUFFW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
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
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBUFFW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXBUFE`"]
pub enum TXBUFEW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
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
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXBUFEW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NSSR`"]
pub enum NSSRW {
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
    _1,
}
impl NSSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSSRW::_0 => false,
            NSSRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSSRW<'a> {
    w: &'a mut W,
}
impl<'a> _NSSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSSRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NSSRW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NSSRW::_1)
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
    #[doc = "No effect."]
    _0,
    #[doc = "Disables the corresponding interrupt."]
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
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEMPTYW::_0)
    }
    #[doc = "Disables the corresponding interrupt."]
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
#[doc = r" Proxy"]
pub struct _UNDESW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDESW<'a> {
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
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Disable"]
    #[inline]
    pub fn rdrf(&mut self) -> _RDRFW {
        _RDRFW { w: self }
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Disable"]
    #[inline]
    pub fn tdre(&mut self) -> _TDREW {
        _TDREW { w: self }
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Disable"]
    #[inline]
    pub fn modf(&mut self) -> _MODFW {
        _MODFW { w: self }
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Disable"]
    #[inline]
    pub fn ovres(&mut self) -> _OVRESW {
        _OVRESW { w: self }
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Disable"]
    #[inline]
    pub fn endrx(&mut self) -> _ENDRXW {
        _ENDRXW { w: self }
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Disable"]
    #[inline]
    pub fn endtx(&mut self) -> _ENDTXW {
        _ENDTXW { w: self }
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Disable"]
    #[inline]
    pub fn rxbuff(&mut self) -> _RXBUFFW {
        _RXBUFFW { w: self }
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Disable"]
    #[inline]
    pub fn txbufe(&mut self) -> _TXBUFEW {
        _TXBUFEW { w: self }
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Disable"]
    #[inline]
    pub fn nssr(&mut self) -> _NSSRW {
        _NSSRW { w: self }
    }
    #[doc = "Bit 9 - Transmission Registers Empty Disable"]
    #[inline]
    pub fn txempty(&mut self) -> _TXEMPTYW {
        _TXEMPTYW { w: self }
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Disable"]
    #[inline]
    pub fn undes(&mut self) -> _UNDESW {
        _UNDESW { w: self }
    }
}
