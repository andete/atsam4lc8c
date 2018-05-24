#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IMR_USART_MODE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RXRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl RXRDYR {
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
            RXRDYR::_0 => false,
            RXRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXRDYR {
        match value {
            false => RXRDYR::_0,
            true => RXRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXRDYR::_1
    }
}
#[doc = "Possible values of the field `TXRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl TXRDYR {
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
            TXRDYR::_0 => false,
            TXRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRDYR {
        match value {
            false => TXRDYR::_0,
            true => TXRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXRDYR::_1
    }
}
#[doc = "Possible values of the field `RXBRK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRKR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl RXBRKR {
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
            RXBRKR::_0 => false,
            RXBRKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXBRKR {
        match value {
            false => RXBRKR::_0,
            true => RXBRKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXBRKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXBRKR::_1
    }
}
#[doc = "Possible values of the field `OVRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRER {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl OVRER {
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
            OVRER::_0 => false,
            OVRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRER {
        match value {
            false => OVRER::_0,
            true => OVRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVRER::_1
    }
}
#[doc = "Possible values of the field `FRAME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMER {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl FRAMER {
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
            FRAMER::_0 => false,
            FRAMER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAMER {
        match value {
            false => FRAMER::_0,
            true => FRAMER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRAMER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRAMER::_1
    }
}
#[doc = "Possible values of the field `PARE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARER {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl PARER {
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
            PARER::_0 => false,
            PARER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARER {
        match value {
            false => PARER::_0,
            true => PARER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PARER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PARER::_1
    }
}
#[doc = "Possible values of the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl TIMEOUTR {
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
            TIMEOUTR::_0 => false,
            TIMEOUTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMEOUTR {
        match value {
            false => TIMEOUTR::_0,
            true => TIMEOUTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMEOUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMEOUTR::_1
    }
}
#[doc = "Possible values of the field `TXEMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTYR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl TXEMPTYR {
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
            TXEMPTYR::_0 => false,
            TXEMPTYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEMPTYR {
        match value {
            false => TXEMPTYR::_0,
            true => TXEMPTYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTYR::_1
    }
}
#[doc = "Possible values of the field `ITER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITERR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl ITERR {
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
            ITERR::_0 => false,
            ITERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITERR {
        match value {
            false => ITERR::_0,
            true => ITERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ITERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ITERR::_1
    }
}
#[doc = "Possible values of the field `TXBUFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFER {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl TXBUFER {
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
            TXBUFER::_0 => false,
            TXBUFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXBUFER {
        match value {
            false => TXBUFER::_0,
            true => TXBUFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXBUFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXBUFER::_1
    }
}
#[doc = "Possible values of the field `RXBUFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFFR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl RXBUFFR {
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
            RXBUFFR::_0 => false,
            RXBUFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXBUFFR {
        match value {
            false => RXBUFFR::_0,
            true => RXBUFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXBUFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXBUFFR::_1
    }
}
#[doc = "Possible values of the field `NACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl NACKR {
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
            NACKR::_0 => false,
            NACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKR {
        match value {
            false => NACKR::_0,
            true => NACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NACKR::_1
    }
}
#[doc = "Possible values of the field `RIIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIICR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl RIICR {
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
            RIICR::_0 => false,
            RIICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIICR {
        match value {
            false => RIICR::_0,
            true => RIICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RIICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RIICR::_1
    }
}
#[doc = "Possible values of the field `DSRIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRICR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl DSRICR {
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
            DSRICR::_0 => false,
            DSRICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSRICR {
        match value {
            false => DSRICR::_0,
            true => DSRICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DSRICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DSRICR::_1
    }
}
#[doc = "Possible values of the field `DCDIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDICR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl DCDICR {
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
            DCDICR::_0 => false,
            DCDICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDICR {
        match value {
            false => DCDICR::_0,
            true => DCDICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCDICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCDICR::_1
    }
}
#[doc = "Possible values of the field `CTSIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSICR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl CTSICR {
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
            CTSICR::_0 => false,
            CTSICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSICR {
        match value {
            false => CTSICR::_0,
            true => CTSICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CTSICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CTSICR::_1
    }
}
#[doc = r" Value of the field"]
pub struct MANER {
    bits: bool,
}
impl MANER {
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
#[doc = "Possible values of the field `MANEA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MANEAR {
    #[doc = "The interrupt is disabled"]
    _0,
    #[doc = "The interrupt is enabled"]
    _1,
}
impl MANEAR {
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
            MANEAR::_0 => false,
            MANEAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MANEAR {
        match value {
            false => MANEAR::_0,
            true => MANEAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MANEAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MANEAR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline]
    pub fn rxrdy(&self) -> RXRDYR {
        RXRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline]
    pub fn txrdy(&self) -> TXRDYR {
        TXRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Mask"]
    #[inline]
    pub fn rxbrk(&self) -> RXBRKR {
        RXBRKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline]
    pub fn ovre(&self) -> OVRER {
        OVRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline]
    pub fn frame(&self) -> FRAMER {
        FRAMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline]
    pub fn pare(&self) -> PARER {
        PARER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Time-out Interrupt Mask"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline]
    pub fn txempty(&self) -> TXEMPTYR {
        TXEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Iteration Interrupt Mask"]
    #[inline]
    pub fn iter(&self) -> ITERR {
        ITERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Mask"]
    #[inline]
    pub fn txbufe(&self) -> TXBUFER {
        TXBUFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Mask"]
    #[inline]
    pub fn rxbuff(&self) -> RXBUFFR {
        RXBUFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Mask"]
    #[inline]
    pub fn nack(&self) -> NACKR {
        NACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline]
    pub fn riic(&self) -> RIICR {
        RIICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline]
    pub fn dsric(&self) -> DSRICR {
        DSRICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline]
    pub fn dcdic(&self) -> DCDICR {
        DCDICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Mask"]
    #[inline]
    pub fn ctsic(&self) -> CTSICR {
        CTSICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Manchester Error Interrupt Mask"]
    #[inline]
    pub fn mane(&self) -> MANER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MANER { bits }
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Mask"]
    #[inline]
    pub fn manea(&self) -> MANEAR {
        MANEAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
