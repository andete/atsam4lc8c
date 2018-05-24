#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMR_WAVEFORM {
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
    #[doc = "TIMER_DIV1_CLOCK"]
    TIMER_DIV1_CLOCK,
    #[doc = "TIMER_DIV2_CLOCK"]
    TIMER_DIV2_CLOCK,
    #[doc = "TIMER_DIV3_CLOCK"]
    TIMER_DIV3_CLOCK,
    #[doc = "TIMER_DIV4_CLOCK"]
    TIMER_DIV4_CLOCK,
    #[doc = "TIMER_DIV5_CLOCK"]
    TIMER_DIV5_CLOCK,
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
            TCCLKSR::TIMER_DIV1_CLOCK => 0,
            TCCLKSR::TIMER_DIV2_CLOCK => 1,
            TCCLKSR::TIMER_DIV3_CLOCK => 2,
            TCCLKSR::TIMER_DIV4_CLOCK => 3,
            TCCLKSR::TIMER_DIV5_CLOCK => 4,
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
            0 => TCCLKSR::TIMER_DIV1_CLOCK,
            1 => TCCLKSR::TIMER_DIV2_CLOCK,
            2 => TCCLKSR::TIMER_DIV3_CLOCK,
            3 => TCCLKSR::TIMER_DIV4_CLOCK,
            4 => TCCLKSR::TIMER_DIV5_CLOCK,
            5 => TCCLKSR::XC0,
            6 => TCCLKSR::XC1,
            7 => TCCLKSR::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV1_CLOCK`"]
    #[inline]
    pub fn is_timer_div1_clock(&self) -> bool {
        *self == TCCLKSR::TIMER_DIV1_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV2_CLOCK`"]
    #[inline]
    pub fn is_timer_div2_clock(&self) -> bool {
        *self == TCCLKSR::TIMER_DIV2_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV3_CLOCK`"]
    #[inline]
    pub fn is_timer_div3_clock(&self) -> bool {
        *self == TCCLKSR::TIMER_DIV3_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV4_CLOCK`"]
    #[inline]
    pub fn is_timer_div4_clock(&self) -> bool {
        *self == TCCLKSR::TIMER_DIV4_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV5_CLOCK`"]
    #[inline]
    pub fn is_timer_div5_clock(&self) -> bool {
        *self == TCCLKSR::TIMER_DIV5_CLOCK
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
#[doc = "Possible values of the field `CPCSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCSTOPR {
    #[doc = "Counter clock is not stopped when counter reaches RC."]
    _0,
    #[doc = "Counter clock is stopped when counter reaches RC."]
    _1,
}
impl CPCSTOPR {
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
            CPCSTOPR::_0 => false,
            CPCSTOPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPCSTOPR {
        match value {
            false => CPCSTOPR::_0,
            true => CPCSTOPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPCSTOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPCSTOPR::_1
    }
}
#[doc = "Possible values of the field `CPCDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCDISR {
    #[doc = "Counter clock is not disabled when counter reaches RC."]
    _0,
    #[doc = "Counter clock is disabled when counter reaches RC."]
    _1,
}
impl CPCDISR {
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
            CPCDISR::_0 => false,
            CPCDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPCDISR {
        match value {
            false => CPCDISR::_0,
            true => CPCDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPCDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPCDISR::_1
    }
}
#[doc = "Possible values of the field `EEVTEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEVTEDGR {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge"]
    POS_EDGE,
    #[doc = "falling edge"]
    NEG_EDGE,
    #[doc = "each edge"]
    BOTH_EDGES,
}
impl EEVTEDGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EEVTEDGR::NO_EDGE => 0,
            EEVTEDGR::POS_EDGE => 1,
            EEVTEDGR::NEG_EDGE => 2,
            EEVTEDGR::BOTH_EDGES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EEVTEDGR {
        match value {
            0 => EEVTEDGR::NO_EDGE,
            1 => EEVTEDGR::POS_EDGE,
            2 => EEVTEDGR::NEG_EDGE,
            3 => EEVTEDGR::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline]
    pub fn is_no_edge(&self) -> bool {
        *self == EEVTEDGR::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE`"]
    #[inline]
    pub fn is_pos_edge(&self) -> bool {
        *self == EEVTEDGR::POS_EDGE
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE`"]
    #[inline]
    pub fn is_neg_edge(&self) -> bool {
        *self == EEVTEDGR::NEG_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline]
    pub fn is_both_edges(&self) -> bool {
        *self == EEVTEDGR::BOTH_EDGES
    }
}
#[doc = "Possible values of the field `EEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEVTR {
    #[doc = "TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    TIOB_INPUT,
    #[doc = "XC0 output"]
    XC0_OUTPUT,
    #[doc = "XC1 output"]
    XC1_OUTPUT,
    #[doc = "XC2 output"]
    XC2_OUTPUT,
}
impl EEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EEVTR::TIOB_INPUT => 0,
            EEVTR::XC0_OUTPUT => 1,
            EEVTR::XC1_OUTPUT => 2,
            EEVTR::XC2_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EEVTR {
        match value {
            0 => EEVTR::TIOB_INPUT,
            1 => EEVTR::XC0_OUTPUT,
            2 => EEVTR::XC1_OUTPUT,
            3 => EEVTR::XC2_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIOB_INPUT`"]
    #[inline]
    pub fn is_tiob_input(&self) -> bool {
        *self == EEVTR::TIOB_INPUT
    }
    #[doc = "Checks if the value of the field is `XC0_OUTPUT`"]
    #[inline]
    pub fn is_xc0_output(&self) -> bool {
        *self == EEVTR::XC0_OUTPUT
    }
    #[doc = "Checks if the value of the field is `XC1_OUTPUT`"]
    #[inline]
    pub fn is_xc1_output(&self) -> bool {
        *self == EEVTR::XC1_OUTPUT
    }
    #[doc = "Checks if the value of the field is `XC2_OUTPUT`"]
    #[inline]
    pub fn is_xc2_output(&self) -> bool {
        *self == EEVTR::XC2_OUTPUT
    }
}
#[doc = "Possible values of the field `ENETRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENETRGR {
    #[doc = "The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    _0,
    #[doc = "The external event resets the counter and starts the counter clock."]
    _1,
}
impl ENETRGR {
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
            ENETRGR::_0 => false,
            ENETRGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENETRGR {
        match value {
            false => ENETRGR::_0,
            true => ENETRGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENETRGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENETRGR::_1
    }
}
#[doc = "Possible values of the field `WAVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVSELR {
    #[doc = "UP mode without automatic trigger on RC Compare"]
    UP_NO_AUTO,
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN_NO_AUTO,
    #[doc = "UP mode with automatic trigger on RC Compare"]
    UP_AUTO,
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_AUTO,
}
impl WAVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVSELR::UP_NO_AUTO => 0,
            WAVSELR::UPDOWN_NO_AUTO => 1,
            WAVSELR::UP_AUTO => 2,
            WAVSELR::UPDOWN_AUTO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVSELR {
        match value {
            0 => WAVSELR::UP_NO_AUTO,
            1 => WAVSELR::UPDOWN_NO_AUTO,
            2 => WAVSELR::UP_AUTO,
            3 => WAVSELR::UPDOWN_AUTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP_NO_AUTO`"]
    #[inline]
    pub fn is_up_no_auto(&self) -> bool {
        *self == WAVSELR::UP_NO_AUTO
    }
    #[doc = "Checks if the value of the field is `UPDOWN_NO_AUTO`"]
    #[inline]
    pub fn is_updown_no_auto(&self) -> bool {
        *self == WAVSELR::UPDOWN_NO_AUTO
    }
    #[doc = "Checks if the value of the field is `UP_AUTO`"]
    #[inline]
    pub fn is_up_auto(&self) -> bool {
        *self == WAVSELR::UP_AUTO
    }
    #[doc = "Checks if the value of the field is `UPDOWN_AUTO`"]
    #[inline]
    pub fn is_updown_auto(&self) -> bool {
        *self == WAVSELR::UPDOWN_AUTO
    }
}
#[doc = "Possible values of the field `WAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVER {
    #[doc = "Waveform Mode is disabled (Capture Mode is enabled)."]
    _0,
    #[doc = "Waveform Mode is enabled."]
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
#[doc = "Possible values of the field `ACPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPAR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl ACPAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACPAR::NONE => 0,
            ACPAR::SET => 1,
            ACPAR::CLEAR => 2,
            ACPAR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACPAR {
        match value {
            0 => ACPAR::NONE,
            1 => ACPAR::SET,
            2 => ACPAR::CLEAR,
            3 => ACPAR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACPAR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ACPAR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ACPAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == ACPAR::TOGGLE
    }
}
#[doc = "Possible values of the field `ACPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPCR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl ACPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACPCR::NONE => 0,
            ACPCR::SET => 1,
            ACPCR::CLEAR => 2,
            ACPCR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACPCR {
        match value {
            0 => ACPCR::NONE,
            1 => ACPCR::SET,
            2 => ACPCR::CLEAR,
            3 => ACPCR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACPCR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ACPCR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ACPCR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == ACPCR::TOGGLE
    }
}
#[doc = "Possible values of the field `AEEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEEVTR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl AEEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AEEVTR::NONE => 0,
            AEEVTR::SET => 1,
            AEEVTR::CLEAR => 2,
            AEEVTR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AEEVTR {
        match value {
            0 => AEEVTR::NONE,
            1 => AEEVTR::SET,
            2 => AEEVTR::CLEAR,
            3 => AEEVTR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == AEEVTR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == AEEVTR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == AEEVTR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == AEEVTR::TOGGLE
    }
}
#[doc = "Possible values of the field `ASWTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASWTRGR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl ASWTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASWTRGR::NONE => 0,
            ASWTRGR::SET => 1,
            ASWTRGR::CLEAR => 2,
            ASWTRGR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASWTRGR {
        match value {
            0 => ASWTRGR::NONE,
            1 => ASWTRGR::SET,
            2 => ASWTRGR::CLEAR,
            3 => ASWTRGR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ASWTRGR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ASWTRGR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ASWTRGR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == ASWTRGR::TOGGLE
    }
}
#[doc = "Possible values of the field `BCPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPBR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BCPBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BCPBR::NONE => 0,
            BCPBR::SET => 1,
            BCPBR::CLEAR => 2,
            BCPBR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BCPBR {
        match value {
            0 => BCPBR::NONE,
            1 => BCPBR::SET,
            2 => BCPBR::CLEAR,
            3 => BCPBR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BCPBR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BCPBR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BCPBR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BCPBR::TOGGLE
    }
}
#[doc = "Possible values of the field `BCPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPCR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BCPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BCPCR::NONE => 0,
            BCPCR::SET => 1,
            BCPCR::CLEAR => 2,
            BCPCR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BCPCR {
        match value {
            0 => BCPCR::NONE,
            1 => BCPCR::SET,
            2 => BCPCR::CLEAR,
            3 => BCPCR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BCPCR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BCPCR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BCPCR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BCPCR::TOGGLE
    }
}
#[doc = "Possible values of the field `BEEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEEVTR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BEEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BEEVTR::NONE => 0,
            BEEVTR::SET => 1,
            BEEVTR::CLEAR => 2,
            BEEVTR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BEEVTR {
        match value {
            0 => BEEVTR::NONE,
            1 => BEEVTR::SET,
            2 => BEEVTR::CLEAR,
            3 => BEEVTR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BEEVTR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BEEVTR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BEEVTR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BEEVTR::TOGGLE
    }
}
#[doc = "Possible values of the field `BSWTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSWTRGR {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BSWTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BSWTRGR::NONE => 0,
            BSWTRGR::SET => 1,
            BSWTRGR::CLEAR => 2,
            BSWTRGR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BSWTRGR {
        match value {
            0 => BSWTRGR::NONE,
            1 => BSWTRGR::SET,
            2 => BSWTRGR::CLEAR,
            3 => BSWTRGR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BSWTRGR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BSWTRGR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BSWTRGR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BSWTRGR::TOGGLE
    }
}
#[doc = "Values that can be written to the field `TCCLKS`"]
pub enum TCCLKSW {
    #[doc = "TIMER_DIV1_CLOCK"]
    TIMER_DIV1_CLOCK,
    #[doc = "TIMER_DIV2_CLOCK"]
    TIMER_DIV2_CLOCK,
    #[doc = "TIMER_DIV3_CLOCK"]
    TIMER_DIV3_CLOCK,
    #[doc = "TIMER_DIV4_CLOCK"]
    TIMER_DIV4_CLOCK,
    #[doc = "TIMER_DIV5_CLOCK"]
    TIMER_DIV5_CLOCK,
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
            TCCLKSW::TIMER_DIV1_CLOCK => 0,
            TCCLKSW::TIMER_DIV2_CLOCK => 1,
            TCCLKSW::TIMER_DIV3_CLOCK => 2,
            TCCLKSW::TIMER_DIV4_CLOCK => 3,
            TCCLKSW::TIMER_DIV5_CLOCK => 4,
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
    #[doc = "TIMER_DIV1_CLOCK"]
    #[inline]
    pub fn timer_div1_clock(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_DIV1_CLOCK)
    }
    #[doc = "TIMER_DIV2_CLOCK"]
    #[inline]
    pub fn timer_div2_clock(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_DIV2_CLOCK)
    }
    #[doc = "TIMER_DIV3_CLOCK"]
    #[inline]
    pub fn timer_div3_clock(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_DIV3_CLOCK)
    }
    #[doc = "TIMER_DIV4_CLOCK"]
    #[inline]
    pub fn timer_div4_clock(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_DIV4_CLOCK)
    }
    #[doc = "TIMER_DIV5_CLOCK"]
    #[inline]
    pub fn timer_div5_clock(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_DIV5_CLOCK)
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
#[doc = "Values that can be written to the field `CPCSTOP`"]
pub enum CPCSTOPW {
    #[doc = "Counter clock is not stopped when counter reaches RC."]
    _0,
    #[doc = "Counter clock is stopped when counter reaches RC."]
    _1,
}
impl CPCSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPCSTOPW::_0 => false,
            CPCSTOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPCSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPCSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter clock is not stopped when counter reaches RC."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCSTOPW::_0)
    }
    #[doc = "Counter clock is stopped when counter reaches RC."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCSTOPW::_1)
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
#[doc = "Values that can be written to the field `CPCDIS`"]
pub enum CPCDISW {
    #[doc = "Counter clock is not disabled when counter reaches RC."]
    _0,
    #[doc = "Counter clock is disabled when counter reaches RC."]
    _1,
}
impl CPCDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPCDISW::_0 => false,
            CPCDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPCDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter clock is not disabled when counter reaches RC."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCDISW::_0)
    }
    #[doc = "Counter clock is disabled when counter reaches RC."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCDISW::_1)
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
#[doc = "Values that can be written to the field `EEVTEDG`"]
pub enum EEVTEDGW {
    #[doc = "none"]
    NO_EDGE,
    #[doc = "rising edge"]
    POS_EDGE,
    #[doc = "falling edge"]
    NEG_EDGE,
    #[doc = "each edge"]
    BOTH_EDGES,
}
impl EEVTEDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EEVTEDGW::NO_EDGE => 0,
            EEVTEDGW::POS_EDGE => 1,
            EEVTEDGW::NEG_EDGE => 2,
            EEVTEDGW::BOTH_EDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEVTEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _EEVTEDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEVTEDGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(EEVTEDGW::NO_EDGE)
    }
    #[doc = "rising edge"]
    #[inline]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(EEVTEDGW::POS_EDGE)
    }
    #[doc = "falling edge"]
    #[inline]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(EEVTEDGW::NEG_EDGE)
    }
    #[doc = "each edge"]
    #[inline]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EEVTEDGW::BOTH_EDGES)
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
#[doc = "Values that can be written to the field `EEVT`"]
pub enum EEVTW {
    #[doc = "TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    TIOB_INPUT,
    #[doc = "XC0 output"]
    XC0_OUTPUT,
    #[doc = "XC1 output"]
    XC1_OUTPUT,
    #[doc = "XC2 output"]
    XC2_OUTPUT,
}
impl EEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EEVTW::TIOB_INPUT => 0,
            EEVTW::XC0_OUTPUT => 1,
            EEVTW::XC1_OUTPUT => 2,
            EEVTW::XC2_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEVTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    #[inline]
    pub fn tiob_input(self) -> &'a mut W {
        self.variant(EEVTW::TIOB_INPUT)
    }
    #[doc = "XC0 output"]
    #[inline]
    pub fn xc0_output(self) -> &'a mut W {
        self.variant(EEVTW::XC0_OUTPUT)
    }
    #[doc = "XC1 output"]
    #[inline]
    pub fn xc1_output(self) -> &'a mut W {
        self.variant(EEVTW::XC1_OUTPUT)
    }
    #[doc = "XC2 output"]
    #[inline]
    pub fn xc2_output(self) -> &'a mut W {
        self.variant(EEVTW::XC2_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENETRG`"]
pub enum ENETRGW {
    #[doc = "The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    _0,
    #[doc = "The external event resets the counter and starts the counter clock."]
    _1,
}
impl ENETRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENETRGW::_0 => false,
            ENETRGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENETRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENETRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENETRGW::_0)
    }
    #[doc = "The external event resets the counter and starts the counter clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENETRGW::_1)
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
#[doc = "Values that can be written to the field `WAVSEL`"]
pub enum WAVSELW {
    #[doc = "UP mode without automatic trigger on RC Compare"]
    UP_NO_AUTO,
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN_NO_AUTO,
    #[doc = "UP mode with automatic trigger on RC Compare"]
    UP_AUTO,
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_AUTO,
}
impl WAVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVSELW::UP_NO_AUTO => 0,
            WAVSELW::UPDOWN_NO_AUTO => 1,
            WAVSELW::UP_AUTO => 2,
            WAVSELW::UPDOWN_AUTO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline]
    pub fn up_no_auto(self) -> &'a mut W {
        self.variant(WAVSELW::UP_NO_AUTO)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline]
    pub fn updown_no_auto(self) -> &'a mut W {
        self.variant(WAVSELW::UPDOWN_NO_AUTO)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline]
    pub fn up_auto(self) -> &'a mut W {
        self.variant(WAVSELW::UP_AUTO)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline]
    pub fn updown_auto(self) -> &'a mut W {
        self.variant(WAVSELW::UPDOWN_AUTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAVE`"]
pub enum WAVEW {
    #[doc = "Waveform Mode is disabled (Capture Mode is enabled)."]
    _0,
    #[doc = "Waveform Mode is enabled."]
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
    #[doc = "Waveform Mode is disabled (Capture Mode is enabled)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAVEW::_0)
    }
    #[doc = "Waveform Mode is enabled."]
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
#[doc = "Values that can be written to the field `ACPA`"]
pub enum ACPAW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl ACPAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACPAW::NONE => 0,
            ACPAW::SET => 1,
            ACPAW::CLEAR => 2,
            ACPAW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACPAW<'a> {
    w: &'a mut W,
}
impl<'a> _ACPAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACPAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPAW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPAW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPAW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPAW::TOGGLE)
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
#[doc = "Values that can be written to the field `ACPC`"]
pub enum ACPCW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl ACPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACPCW::NONE => 0,
            ACPCW::SET => 1,
            ACPCW::CLEAR => 2,
            ACPCW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACPCW<'a> {
    w: &'a mut W,
}
impl<'a> _ACPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACPCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPCW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPCW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPCW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPCW::TOGGLE)
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
#[doc = "Values that can be written to the field `AEEVT`"]
pub enum AEEVTW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl AEEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AEEVTW::NONE => 0,
            AEEVTW::SET => 1,
            AEEVTW::CLEAR => 2,
            AEEVTW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AEEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _AEEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AEEVTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(AEEVTW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(AEEVTW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(AEEVTW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(AEEVTW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASWTRG`"]
pub enum ASWTRGW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl ASWTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ASWTRGW::NONE => 0,
            ASWTRGW::SET => 1,
            ASWTRGW::CLEAR => 2,
            ASWTRGW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASWTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ASWTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASWTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ASWTRGW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ASWTRGW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ASWTRGW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ASWTRGW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCPB`"]
pub enum BCPBW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BCPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BCPBW::NONE => 0,
            BCPBW::SET => 1,
            BCPBW::CLEAR => 2,
            BCPBW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCPBW<'a> {
    w: &'a mut W,
}
impl<'a> _BCPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCPBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPBW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPBW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPBW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPBW::TOGGLE)
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
#[doc = "Values that can be written to the field `BCPC`"]
pub enum BCPCW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BCPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BCPCW::NONE => 0,
            BCPCW::SET => 1,
            BCPCW::CLEAR => 2,
            BCPCW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCPCW<'a> {
    w: &'a mut W,
}
impl<'a> _BCPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCPCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPCW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPCW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPCW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPCW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BEEVT`"]
pub enum BEEVTW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BEEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BEEVTW::NONE => 0,
            BEEVTW::SET => 1,
            BEEVTW::CLEAR => 2,
            BEEVTW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _BEEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEEVTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BEEVTW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BEEVTW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEEVTW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BEEVTW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BSWTRG`"]
pub enum BSWTRGW {
    #[doc = "none"]
    NONE,
    #[doc = "set"]
    SET,
    #[doc = "clear"]
    CLEAR,
    #[doc = "toggle"]
    TOGGLE,
}
impl BSWTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BSWTRGW::NONE => 0,
            BSWTRGW::SET => 1,
            BSWTRGW::CLEAR => 2,
            BSWTRGW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSWTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _BSWTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSWTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "none"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BSWTRGW::NONE)
    }
    #[doc = "set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BSWTRGW::SET)
    }
    #[doc = "clear"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BSWTRGW::CLEAR)
    }
    #[doc = "toggle"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BSWTRGW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline]
    pub fn cpcstop(&self) -> CPCSTOPR {
        CPCSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline]
    pub fn cpcdis(&self) -> CPCDISR {
        CPCDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline]
    pub fn eevtedg(&self) -> EEVTEDGR {
        EEVTEDGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline]
    pub fn eevt(&self) -> EEVTR {
        EEVTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline]
    pub fn enetrg(&self) -> ENETRGR {
        ENETRGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline]
    pub fn wavsel(&self) -> WAVSELR {
        WAVSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - WAVE"]
    #[inline]
    pub fn wave(&self) -> WAVER {
        WAVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline]
    pub fn acpa(&self) -> ACPAR {
        ACPAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline]
    pub fn acpc(&self) -> ACPCR {
        ACPCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline]
    pub fn aeevt(&self) -> AEEVTR {
        AEEVTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline]
    pub fn aswtrg(&self) -> ASWTRGR {
        ASWTRGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline]
    pub fn bcpb(&self) -> BCPBR {
        BCPBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline]
    pub fn bcpc(&self) -> BCPCR {
        BCPCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline]
    pub fn beevt(&self) -> BEEVTR {
        BEEVTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline]
    pub fn bswtrg(&self) -> BSWTRGR {
        BSWTRGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline]
    pub fn cpcstop(&mut self) -> _CPCSTOPW {
        _CPCSTOPW { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline]
    pub fn cpcdis(&mut self) -> _CPCDISW {
        _CPCDISW { w: self }
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline]
    pub fn eevtedg(&mut self) -> _EEVTEDGW {
        _EEVTEDGW { w: self }
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline]
    pub fn eevt(&mut self) -> _EEVTW {
        _EEVTW { w: self }
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline]
    pub fn enetrg(&mut self) -> _ENETRGW {
        _ENETRGW { w: self }
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline]
    pub fn wavsel(&mut self) -> _WAVSELW {
        _WAVSELW { w: self }
    }
    #[doc = "Bit 15 - WAVE"]
    #[inline]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline]
    pub fn acpa(&mut self) -> _ACPAW {
        _ACPAW { w: self }
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline]
    pub fn acpc(&mut self) -> _ACPCW {
        _ACPCW { w: self }
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline]
    pub fn aeevt(&mut self) -> _AEEVTW {
        _AEEVTW { w: self }
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline]
    pub fn aswtrg(&mut self) -> _ASWTRGW {
        _ASWTRGW { w: self }
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline]
    pub fn bcpb(&mut self) -> _BCPBW {
        _BCPBW { w: self }
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline]
    pub fn bcpc(&mut self) -> _BCPCW {
        _BCPCW { w: self }
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline]
    pub fn beevt(&mut self) -> _BEEVTW {
        _BEEVTW { w: self }
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline]
    pub fn bswtrg(&mut self) -> _BSWTRGW {
        _BSWTRGW { w: self }
    }
}
