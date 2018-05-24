#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMR_CAPTURE {
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
#[doc = "Possible values of the field `TCCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCLKSR {
    #[doc = "TIMER_CLOCK1"]
    TIMER_CLOCK1,
    #[doc = "TIMER_CLOCK2"]
    TIMER_CLOCK2,
    #[doc = "TIMER_CLOCK3"]
    TIMER_CLOCK3,
    #[doc = "TIMER_CLOCK4"]
    TIMER_CLOCK4,
    #[doc = "TIMER_CLOCK5"]
    TIMER_CLOCK5,
    #[doc = "XC0"]
    XC0,
    #[doc = "XC1"]
    XC1,
    #[doc = "XC2"]
    XC2,
}
impl TCCLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCCLKSR::TIMER_CLOCK1 => 0,
            TCCLKSR::TIMER_CLOCK2 => 1,
            TCCLKSR::TIMER_CLOCK3 => 2,
            TCCLKSR::TIMER_CLOCK4 => 3,
            TCCLKSR::TIMER_CLOCK5 => 4,
            TCCLKSR::XC0 => 5,
            TCCLKSR::XC1 => 6,
            TCCLKSR::XC2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCCLKSR {
        match value {
            0 => TCCLKSR::TIMER_CLOCK1,
            1 => TCCLKSR::TIMER_CLOCK2,
            2 => TCCLKSR::TIMER_CLOCK3,
            3 => TCCLKSR::TIMER_CLOCK4,
            4 => TCCLKSR::TIMER_CLOCK5,
            5 => TCCLKSR::XC0,
            6 => TCCLKSR::XC1,
            7 => TCCLKSR::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKSR::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKSR::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKSR::XC2
    }
}
#[doc = "Possible values of the field `CLKI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKIR {
    #[doc = "Counter is incremented on rising edge of the clock."]
    _0,
    #[doc = "Counter is incremented on falling edge of the clock."]
    _1,
}
impl CLKIR {
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
            CLKIR::_0 => false,
            CLKIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKIR {
        match value {
            false => CLKIR::_0,
            true => CLKIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKIR::_1
    }
}
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "The clock is not gated by an external signal."]
    NOT_GATED,
    #[doc = "XC0 is ANDed with the selected clock."]
    CLK_AND_XC0,
    #[doc = "XC1 is ANDed with the selected clock."]
    CLK_AND_XC1,
    #[doc = "XC2 is ANDed with the selected clock."]
    CLK_AND_XC2,
}
impl BURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURSTR::NOT_GATED => 0,
            BURSTR::CLK_AND_XC0 => 1,
            BURSTR::CLK_AND_XC1 => 2,
            BURSTR::CLK_AND_XC2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURSTR {
        match value {
            0 => BURSTR::NOT_GATED,
            1 => BURSTR::CLK_AND_XC0,
            2 => BURSTR::CLK_AND_XC1,
            3 => BURSTR::CLK_AND_XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline]
    pub fn is_not_gated(&self) -> bool {
        *self == BURSTR::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC0`"]
    #[inline]
    pub fn is_clk_and_xc0(&self) -> bool {
        *self == BURSTR::CLK_AND_XC0
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC1`"]
    #[inline]
    pub fn is_clk_and_xc1(&self) -> bool {
        *self == BURSTR::CLK_AND_XC1
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC2`"]
    #[inline]
    pub fn is_clk_and_xc2(&self) -> bool {
        *self == BURSTR::CLK_AND_XC2
    }
}
#[doc = "Possible values of the field `LDBSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDBSTOPR {
    #[doc = "Counter clock is not stopped when RB loading occurs."]
    _0,
    #[doc = "Counter clock is stopped when RB loading occurs."]
    _1,
}
impl LDBSTOPR {
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
            LDBSTOPR::_0 => false,
            LDBSTOPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDBSTOPR {
        match value {
            false => LDBSTOPR::_0,
            true => LDBSTOPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LDBSTOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LDBSTOPR::_1
    }
}
#[doc = "Possible values of the field `LDBDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDBDISR {
    #[doc = "Counter clock is not disabled when RB loading occurs."]
    _0,
    #[doc = "Counter clock is disabled when RB loading occurs."]
    _1,
}
impl LDBDISR {
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
            LDBDISR::_0 => false,
            LDBDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDBDISR {
        match value {
            false => LDBDISR::_0,
            true => LDBDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LDBDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LDBDISR::_1
    }
}
#[doc = "Possible values of the field `ETRGEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGEDGR {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge"]
    POS_EDGE,
    #[doc = "falling edge"]
    NEG_EDGE,
    #[doc = "each edge"]
    BOTH_EDGES,
}
impl ETRGEDGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ETRGEDGR::NO_EDGE => 0,
            ETRGEDGR::POS_EDGE => 1,
            ETRGEDGR::NEG_EDGE => 2,
            ETRGEDGR::BOTH_EDGES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ETRGEDGR {
        match value {
            0 => ETRGEDGR::NO_EDGE,
            1 => ETRGEDGR::POS_EDGE,
            2 => ETRGEDGR::NEG_EDGE,
            3 => ETRGEDGR::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline]
    pub fn is_no_edge(&self) -> bool {
        *self == ETRGEDGR::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE`"]
    #[inline]
    pub fn is_pos_edge(&self) -> bool {
        *self == ETRGEDGR::POS_EDGE
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE`"]
    #[inline]
    pub fn is_neg_edge(&self) -> bool {
        *self == ETRGEDGR::NEG_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline]
    pub fn is_both_edges(&self) -> bool {
        *self == ETRGEDGR::BOTH_EDGES
    }
}
#[doc = "Possible values of the field `ABETRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABETRGR {
    #[doc = "TIOB is used as an external trigger."]
    _0,
    #[doc = "TIOA is used as an external trigger."]
    _1,
}
impl ABETRGR {
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
            ABETRGR::_0 => false,
            ABETRGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABETRGR {
        match value {
            false => ABETRGR::_0,
            true => ABETRGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ABETRGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ABETRGR::_1
    }
}
#[doc = "Possible values of the field `CPCTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCTRGR {
    #[doc = "RC Compare has no effect on the counter and its clock."]
    _0,
    #[doc = "RC Compare resets the counter and starts the counter clock."]
    _1,
}
impl CPCTRGR {
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
            CPCTRGR::_0 => false,
            CPCTRGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPCTRGR {
        match value {
            false => CPCTRGR::_0,
            true => CPCTRGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPCTRGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPCTRGR::_1
    }
}
#[doc = "Possible values of the field `WAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVER {
    #[doc = "Capture Mode is enabled."]
    _0,
    #[doc = "Capture Mode is disabled (Waveform Mode is enabled)."]
    _1,
}
impl WAVER {
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
            WAVER::_0 => false,
            WAVER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAVER {
        match value {
            false => WAVER::_0,
            true => WAVER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAVER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAVER::_1
    }
}
#[doc = "Possible values of the field `LDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAR {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge of TIOA"]
    POS_EDGE_TIOA,
    #[doc = "falling edge of TIOA"]
    NEG_EDGE_TIOA,
    #[doc = "each edge of TIOA"]
    BOTH_EDGES_TIOA,
}
impl LDRAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDRAR::NO_EDGE => 0,
            LDRAR::POS_EDGE_TIOA => 1,
            LDRAR::NEG_EDGE_TIOA => 2,
            LDRAR::BOTH_EDGES_TIOA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDRAR {
        match value {
            0 => LDRAR::NO_EDGE,
            1 => LDRAR::POS_EDGE_TIOA,
            2 => LDRAR::NEG_EDGE_TIOA,
            3 => LDRAR::BOTH_EDGES_TIOA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline]
    pub fn is_no_edge(&self) -> bool {
        *self == LDRAR::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE_TIOA`"]
    #[inline]
    pub fn is_pos_edge_tioa(&self) -> bool {
        *self == LDRAR::POS_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE_TIOA`"]
    #[inline]
    pub fn is_neg_edge_tioa(&self) -> bool {
        *self == LDRAR::NEG_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_TIOA`"]
    #[inline]
    pub fn is_both_edges_tioa(&self) -> bool {
        *self == LDRAR::BOTH_EDGES_TIOA
    }
}
#[doc = "Possible values of the field `LDRB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBR {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge of TIOA"]
    POS_EDGE_TIOA,
    #[doc = "falling edge of TIOA"]
    NEG_EDGE_TIOA,
    #[doc = "each edge of TIOA"]
    BOTH_EDGES_TIOA,
}
impl LDRBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDRBR::NO_EDGE => 0,
            LDRBR::POS_EDGE_TIOA => 1,
            LDRBR::NEG_EDGE_TIOA => 2,
            LDRBR::BOTH_EDGES_TIOA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDRBR {
        match value {
            0 => LDRBR::NO_EDGE,
            1 => LDRBR::POS_EDGE_TIOA,
            2 => LDRBR::NEG_EDGE_TIOA,
            3 => LDRBR::BOTH_EDGES_TIOA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline]
    pub fn is_no_edge(&self) -> bool {
        *self == LDRBR::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE_TIOA`"]
    #[inline]
    pub fn is_pos_edge_tioa(&self) -> bool {
        *self == LDRBR::POS_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE_TIOA`"]
    #[inline]
    pub fn is_neg_edge_tioa(&self) -> bool {
        *self == LDRBR::NEG_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_TIOA`"]
    #[inline]
    pub fn is_both_edges_tioa(&self) -> bool {
        *self == LDRBR::BOTH_EDGES_TIOA
    }
}
#[doc = "Values that can be written to the field `TCCLKS`"]
pub enum TCCLKSW {
    #[doc = "TIMER_CLOCK1"]
    TIMER_CLOCK1,
    #[doc = "TIMER_CLOCK2"]
    TIMER_CLOCK2,
    #[doc = "TIMER_CLOCK3"]
    TIMER_CLOCK3,
    #[doc = "TIMER_CLOCK4"]
    TIMER_CLOCK4,
    #[doc = "TIMER_CLOCK5"]
    TIMER_CLOCK5,
    #[doc = "XC0"]
    XC0,
    #[doc = "XC1"]
    XC1,
    #[doc = "XC2"]
    XC2,
}
impl TCCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCCLKSW::TIMER_CLOCK1 => 0,
            TCCLKSW::TIMER_CLOCK2 => 1,
            TCCLKSW::TIMER_CLOCK3 => 2,
            TCCLKSW::TIMER_CLOCK4 => 3,
            TCCLKSW::TIMER_CLOCK5 => 4,
            TCCLKSW::XC0 => 5,
            TCCLKSW::XC1 => 6,
            TCCLKSW::XC2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "TIMER_CLOCK1"]
    #[inline]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK1)
    }
    #[doc = "TIMER_CLOCK2"]
    #[inline]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK2)
    }
    #[doc = "TIMER_CLOCK3"]
    #[inline]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK3)
    }
    #[doc = "TIMER_CLOCK4"]
    #[inline]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK4)
    }
    #[doc = "TIMER_CLOCK5"]
    #[inline]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK5)
    }
    #[doc = "XC0"]
    #[inline]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKSW::XC0)
    }
    #[doc = "XC1"]
    #[inline]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKSW::XC1)
    }
    #[doc = "XC2"]
    #[inline]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKSW::XC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKI`"]
pub enum CLKIW {
    #[doc = "Counter is incremented on rising edge of the clock."]
    _0,
    #[doc = "Counter is incremented on falling edge of the clock."]
    _1,
}
impl CLKIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKIW::_0 => false,
            CLKIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKIW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter is incremented on rising edge of the clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKIW::_0)
    }
    #[doc = "Counter is incremented on falling edge of the clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKIW::_1)
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
#[doc = "Values that can be written to the field `BURST`"]
pub enum BURSTW {
    #[doc = "The clock is not gated by an external signal."]
    NOT_GATED,
    #[doc = "XC0 is ANDed with the selected clock."]
    CLK_AND_XC0,
    #[doc = "XC1 is ANDed with the selected clock."]
    CLK_AND_XC1,
    #[doc = "XC2 is ANDed with the selected clock."]
    CLK_AND_XC2,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURSTW::NOT_GATED => 0,
            BURSTW::CLK_AND_XC0 => 1,
            BURSTW::CLK_AND_XC1 => 2,
            BURSTW::CLK_AND_XC2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(BURSTW::NOT_GATED)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline]
    pub fn clk_and_xc0(self) -> &'a mut W {
        self.variant(BURSTW::CLK_AND_XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline]
    pub fn clk_and_xc1(self) -> &'a mut W {
        self.variant(BURSTW::CLK_AND_XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline]
    pub fn clk_and_xc2(self) -> &'a mut W {
        self.variant(BURSTW::CLK_AND_XC2)
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
#[doc = "Values that can be written to the field `LDBSTOP`"]
pub enum LDBSTOPW {
    #[doc = "Counter clock is not stopped when RB loading occurs."]
    _0,
    #[doc = "Counter clock is stopped when RB loading occurs."]
    _1,
}
impl LDBSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDBSTOPW::_0 => false,
            LDBSTOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDBSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LDBSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDBSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter clock is not stopped when RB loading occurs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDBSTOPW::_0)
    }
    #[doc = "Counter clock is stopped when RB loading occurs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDBSTOPW::_1)
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
#[doc = "Values that can be written to the field `LDBDIS`"]
pub enum LDBDISW {
    #[doc = "Counter clock is not disabled when RB loading occurs."]
    _0,
    #[doc = "Counter clock is disabled when RB loading occurs."]
    _1,
}
impl LDBDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDBDISW::_0 => false,
            LDBDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDBDISW<'a> {
    w: &'a mut W,
}
impl<'a> _LDBDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDBDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter clock is not disabled when RB loading occurs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDBDISW::_0)
    }
    #[doc = "Counter clock is disabled when RB loading occurs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDBDISW::_1)
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
#[doc = "Values that can be written to the field `ETRGEDG`"]
pub enum ETRGEDGW {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge"]
    POS_EDGE,
    #[doc = "falling edge"]
    NEG_EDGE,
    #[doc = "each edge"]
    BOTH_EDGES,
}
impl ETRGEDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETRGEDGW::NO_EDGE => 0,
            ETRGEDGW::POS_EDGE => 1,
            ETRGEDGW::NEG_EDGE => 2,
            ETRGEDGW::BOTH_EDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETRGEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _ETRGEDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETRGEDGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(ETRGEDGW::NO_EDGE)
    }
    #[doc = "rising edge"]
    #[inline]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(ETRGEDGW::POS_EDGE)
    }
    #[doc = "falling edge"]
    #[inline]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(ETRGEDGW::NEG_EDGE)
    }
    #[doc = "each edge"]
    #[inline]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(ETRGEDGW::BOTH_EDGES)
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
#[doc = "Values that can be written to the field `ABETRG`"]
pub enum ABETRGW {
    #[doc = "TIOB is used as an external trigger."]
    _0,
    #[doc = "TIOA is used as an external trigger."]
    _1,
}
impl ABETRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABETRGW::_0 => false,
            ABETRGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ABETRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABETRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TIOB is used as an external trigger."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABETRGW::_0)
    }
    #[doc = "TIOA is used as an external trigger."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABETRGW::_1)
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
#[doc = "Values that can be written to the field `CPCTRG`"]
pub enum CPCTRGW {
    #[doc = "RC Compare has no effect on the counter and its clock."]
    _0,
    #[doc = "RC Compare resets the counter and starts the counter clock."]
    _1,
}
impl CPCTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPCTRGW::_0 => false,
            CPCTRGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPCTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPCTRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RC Compare has no effect on the counter and its clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCTRGW::_0)
    }
    #[doc = "RC Compare resets the counter and starts the counter clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCTRGW::_1)
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
#[doc = "Values that can be written to the field `WAVE`"]
pub enum WAVEW {
    #[doc = "Capture Mode is enabled."]
    _0,
    #[doc = "Capture Mode is disabled (Waveform Mode is enabled)."]
    _1,
}
impl WAVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAVEW::_0 => false,
            WAVEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture Mode is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAVEW::_0)
    }
    #[doc = "Capture Mode is disabled (Waveform Mode is enabled)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAVEW::_1)
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
#[doc = "Values that can be written to the field `LDRA`"]
pub enum LDRAW {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge of TIOA"]
    POS_EDGE_TIOA,
    #[doc = "falling edge of TIOA"]
    NEG_EDGE_TIOA,
    #[doc = "each edge of TIOA"]
    BOTH_EDGES_TIOA,
}
impl LDRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDRAW::NO_EDGE => 0,
            LDRAW::POS_EDGE_TIOA => 1,
            LDRAW::NEG_EDGE_TIOA => 2,
            LDRAW::BOTH_EDGES_TIOA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDRAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(LDRAW::NO_EDGE)
    }
    #[doc = "rising edge of TIOA"]
    #[inline]
    pub fn pos_edge_tioa(self) -> &'a mut W {
        self.variant(LDRAW::POS_EDGE_TIOA)
    }
    #[doc = "falling edge of TIOA"]
    #[inline]
    pub fn neg_edge_tioa(self) -> &'a mut W {
        self.variant(LDRAW::NEG_EDGE_TIOA)
    }
    #[doc = "each edge of TIOA"]
    #[inline]
    pub fn both_edges_tioa(self) -> &'a mut W {
        self.variant(LDRAW::BOTH_EDGES_TIOA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LDRB`"]
pub enum LDRBW {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge of TIOA"]
    POS_EDGE_TIOA,
    #[doc = "falling edge of TIOA"]
    NEG_EDGE_TIOA,
    #[doc = "each edge of TIOA"]
    BOTH_EDGES_TIOA,
}
impl LDRBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDRBW::NO_EDGE => 0,
            LDRBW::POS_EDGE_TIOA => 1,
            LDRBW::NEG_EDGE_TIOA => 2,
            LDRBW::BOTH_EDGES_TIOA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDRBW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDRBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(LDRBW::NO_EDGE)
    }
    #[doc = "rising edge of TIOA"]
    #[inline]
    pub fn pos_edge_tioa(self) -> &'a mut W {
        self.variant(LDRBW::POS_EDGE_TIOA)
    }
    #[doc = "falling edge of TIOA"]
    #[inline]
    pub fn neg_edge_tioa(self) -> &'a mut W {
        self.variant(LDRBW::NEG_EDGE_TIOA)
    }
    #[doc = "each edge of TIOA"]
    #[inline]
    pub fn both_edges_tioa(self) -> &'a mut W {
        self.variant(LDRBW::BOTH_EDGES_TIOA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline]
    pub fn tcclks(&self) -> TCCLKSR {
        TCCLKSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline]
    pub fn clki(&self) -> CLKIR {
        CLKIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        BURSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline]
    pub fn ldbstop(&self) -> LDBSTOPR {
        LDBSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline]
    pub fn ldbdis(&self) -> LDBDISR {
        LDBDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline]
    pub fn etrgedg(&self) -> ETRGEDGR {
        ETRGEDGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline]
    pub fn abetrg(&self) -> ABETRGR {
        ABETRGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline]
    pub fn cpctrg(&self) -> CPCTRGR {
        CPCTRGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Wave"]
    #[inline]
    pub fn wave(&self) -> WAVER {
        WAVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - RA Loading Selection"]
    #[inline]
    pub fn ldra(&self) -> LDRAR {
        LDRAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - RB Loading Selection"]
    #[inline]
    pub fn ldrb(&self) -> LDRBR {
        LDRBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline]
    pub fn tcclks(&mut self) -> _TCCLKSW {
        _TCCLKSW { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline]
    pub fn clki(&mut self) -> _CLKIW {
        _CLKIW { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline]
    pub fn ldbstop(&mut self) -> _LDBSTOPW {
        _LDBSTOPW { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline]
    pub fn ldbdis(&mut self) -> _LDBDISW {
        _LDBDISW { w: self }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline]
    pub fn etrgedg(&mut self) -> _ETRGEDGW {
        _ETRGEDGW { w: self }
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline]
    pub fn abetrg(&mut self) -> _ABETRGW {
        _ABETRGW { w: self }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline]
    pub fn cpctrg(&mut self) -> _CPCTRGW {
        _CPCTRGW { w: self }
    }
    #[doc = "Bit 15 - Wave"]
    #[inline]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 16:17 - RA Loading Selection"]
    #[inline]
    pub fn ldra(&mut self) -> _LDRAW {
        _LDRAW { w: self }
    }
    #[doc = "Bits 18:19 - RB Loading Selection"]
    #[inline]
    pub fn ldrb(&mut self) -> _LDRBW {
        _LDRBW { w: self }
    }
}
