#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BMR {
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
#[doc = "Possible values of the field `TC0XC0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0XC0SR {
    #[doc = "Select TCLK0 as clock signal 0."]
    TCLK0,
    #[doc = "Select no clock as clock signal 0."]
    NO_CLK,
    #[doc = "Select TIOA1 as clock signal 0."]
    TIOA1,
    #[doc = "Select TIOA2 as clock signal 0."]
    TIOA2,
}
impl TC0XC0SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TC0XC0SR::TCLK0 => 0,
            TC0XC0SR::NO_CLK => 1,
            TC0XC0SR::TIOA1 => 2,
            TC0XC0SR::TIOA2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TC0XC0SR {
        match value {
            0 => TC0XC0SR::TCLK0,
            1 => TC0XC0SR::NO_CLK,
            2 => TC0XC0SR::TIOA1,
            3 => TC0XC0SR::TIOA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0SR::TCLK0
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline]
    pub fn is_no_clk(&self) -> bool {
        *self == TC0XC0SR::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0SR::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0SR::TIOA2
    }
}
#[doc = "Possible values of the field `TC1XC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1XC1SR {
    #[doc = "Select TCLK1 as clock signal 1."]
    TCLK1,
    #[doc = "Select no clock as clock signal 1."]
    NO_CLK,
    #[doc = "Select TIOA0 as clock signal 1."]
    TIOA0,
    #[doc = "Select TIOA2 as clock signal 1."]
    TIOA2,
}
impl TC1XC1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TC1XC1SR::TCLK1 => 0,
            TC1XC1SR::NO_CLK => 1,
            TC1XC1SR::TIOA0 => 2,
            TC1XC1SR::TIOA2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TC1XC1SR {
        match value {
            0 => TC1XC1SR::TCLK1,
            1 => TC1XC1SR::NO_CLK,
            2 => TC1XC1SR::TIOA0,
            3 => TC1XC1SR::TIOA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1SR::TCLK1
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline]
    pub fn is_no_clk(&self) -> bool {
        *self == TC1XC1SR::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1SR::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1SR::TIOA2
    }
}
#[doc = "Possible values of the field `TC2XC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2XC2SR {
    #[doc = "Select TCLK2 as clock signal 2."]
    TCLK2,
    #[doc = "Select no clock as clock signal 2."]
    NO_CLK,
    #[doc = "Select TIOA0 as clock signal 2."]
    TIOA0,
    #[doc = "Select TIOA1 as clock signal 2."]
    TIOA1,
}
impl TC2XC2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TC2XC2SR::TCLK2 => 0,
            TC2XC2SR::NO_CLK => 1,
            TC2XC2SR::TIOA0 => 2,
            TC2XC2SR::TIOA1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TC2XC2SR {
        match value {
            0 => TC2XC2SR::TCLK2,
            1 => TC2XC2SR::NO_CLK,
            2 => TC2XC2SR::TIOA0,
            3 => TC2XC2SR::TIOA1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2SR::TCLK2
    }
    #[doc = "Checks if the value of the field is `NO_CLK`"]
    #[inline]
    pub fn is_no_clk(&self) -> bool {
        *self == TC2XC2SR::NO_CLK
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline]
    pub fn is_tioa0(&self) -> bool {
        *self == TC2XC2SR::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2SR::TIOA1
    }
}
#[doc = "Values that can be written to the field `TC0XC0S`"]
pub enum TC0XC0SW {
    #[doc = "Select TCLK0 as clock signal 0."]
    TCLK0,
    #[doc = "Select no clock as clock signal 0."]
    NO_CLK,
    #[doc = "Select TIOA1 as clock signal 0."]
    TIOA1,
    #[doc = "Select TIOA2 as clock signal 0."]
    TIOA2,
}
impl TC0XC0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC0XC0SW::TCLK0 => 0,
            TC0XC0SW::NO_CLK => 1,
            TC0XC0SW::TIOA1 => 2,
            TC0XC0SW::TIOA2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC0XC0SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0XC0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC0XC0SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select TCLK0 as clock signal 0."]
    #[inline]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0SW::TCLK0)
    }
    #[doc = "Select no clock as clock signal 0."]
    #[inline]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC0XC0SW::NO_CLK)
    }
    #[doc = "Select TIOA1 as clock signal 0."]
    #[inline]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0SW::TIOA1)
    }
    #[doc = "Select TIOA2 as clock signal 0."]
    #[inline]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0SW::TIOA2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TC1XC1S`"]
pub enum TC1XC1SW {
    #[doc = "Select TCLK1 as clock signal 1."]
    TCLK1,
    #[doc = "Select no clock as clock signal 1."]
    NO_CLK,
    #[doc = "Select TIOA0 as clock signal 1."]
    TIOA0,
    #[doc = "Select TIOA2 as clock signal 1."]
    TIOA2,
}
impl TC1XC1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC1XC1SW::TCLK1 => 0,
            TC1XC1SW::NO_CLK => 1,
            TC1XC1SW::TIOA0 => 2,
            TC1XC1SW::TIOA2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC1XC1SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1XC1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC1XC1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select TCLK1 as clock signal 1."]
    #[inline]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1SW::TCLK1)
    }
    #[doc = "Select no clock as clock signal 1."]
    #[inline]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC1XC1SW::NO_CLK)
    }
    #[doc = "Select TIOA0 as clock signal 1."]
    #[inline]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1SW::TIOA0)
    }
    #[doc = "Select TIOA2 as clock signal 1."]
    #[inline]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1SW::TIOA2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TC2XC2S`"]
pub enum TC2XC2SW {
    #[doc = "Select TCLK2 as clock signal 2."]
    TCLK2,
    #[doc = "Select no clock as clock signal 2."]
    NO_CLK,
    #[doc = "Select TIOA0 as clock signal 2."]
    TIOA0,
    #[doc = "Select TIOA1 as clock signal 2."]
    TIOA1,
}
impl TC2XC2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC2XC2SW::TCLK2 => 0,
            TC2XC2SW::NO_CLK => 1,
            TC2XC2SW::TIOA0 => 2,
            TC2XC2SW::TIOA1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC2XC2SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2XC2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC2XC2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select TCLK2 as clock signal 2."]
    #[inline]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2SW::TCLK2)
    }
    #[doc = "Select no clock as clock signal 2."]
    #[inline]
    pub fn no_clk(self) -> &'a mut W {
        self.variant(TC2XC2SW::NO_CLK)
    }
    #[doc = "Select TIOA0 as clock signal 2."]
    #[inline]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2SW::TIOA0)
    }
    #[doc = "Select TIOA1 as clock signal 2."]
    #[inline]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2SW::TIOA1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline]
    pub fn tc0xc0s(&self) -> TC0XC0SR {
        TC0XC0SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline]
    pub fn tc1xc1s(&self) -> TC1XC1SR {
        TC1XC1SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline]
    pub fn tc2xc2s(&self) -> TC2XC2SR {
        TC2XC2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline]
    pub fn tc0xc0s(&mut self) -> _TC0XC0SW {
        _TC0XC0SW { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline]
    pub fn tc1xc1s(&mut self) -> _TC1XC1SW {
        _TC1XC1SW { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline]
    pub fn tc2xc2s(&mut self) -> _TC2XC2SW {
        _TC2XC2SW { w: self }
    }
}
