#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSR_SPI_SLAVE_MODE {
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
    #[doc = "No complete character has been received since the last read of RHR or the receiver is disabled. If characters werebeing received when the receiver was disabled, RXRDY changes to 1 when the receiver is enabled"]
    _0,
    #[doc = "At least one complete character has been received and RHR has not yet been read"]
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
    #[doc = "A character is in the THR waiting to be transferred to the Transmit Shift Register, or an STTBRK command has been requested, or the transmitter is disabled. As soon as the transmitter is enabled, TXRDY becomes 1"]
    _0,
    #[doc = "There is no character in the THR"]
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
    #[doc = "No Break received or End of Break detected since the last RSTSTA"]
    _0,
    #[doc = "Break Received or End of Break detected since the last RSTSTA"]
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
    #[doc = "No overrun error has occurred since since the last RSTSTA"]
    _0,
    #[doc = "At least one overrun error has occurred since the last RSTSTA"]
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
    #[doc = "No stop bit has been detected low since the last RSTSTA"]
    _0,
    #[doc = "At least one stop bit has been detected low since the last RSTSTA"]
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
    #[doc = "No parity error has been detected since the last RSTSTA"]
    _0,
    #[doc = "At least one parity error has been detected since the last RSTSTA"]
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
    #[doc = "There has not been a time-out since the last Start Time-out command or the Time-out Register is 0"]
    _0,
    #[doc = "There has been a time-out since the last Start Time-out command"]
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
    #[doc = "There are characters in either THR or the Transmit Shift Register, or the transmitter is disabled"]
    _0,
    #[doc = "There is at least one character in either THR or the Transmit Shift Register"]
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
#[doc = "Possible values of the field `UNRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNRER {
    #[doc = "No SPI underrun error has occurred since the last RSTSTA"]
    _0,
    #[doc = "At least one SPI underrun error has occurred since the last RSTSTA"]
    _1,
}
impl UNRER {
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
            UNRER::_0 => false,
            UNRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNRER {
        match value {
            false => UNRER::_0,
            true => UNRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UNRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UNRER::_1
    }
}
#[doc = "Possible values of the field `TXBUFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFER {
    #[doc = "The signal Buffer Empty from the Transmit PDC channel is inactive"]
    _0,
    #[doc = "The signal Buffer Empty from the Transmit PDC channel is active"]
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
    #[doc = "The signal Buffer Full from the Receive PDC channel is inactive"]
    _0,
    #[doc = "The signal Buffer Full from the Receive PDC channel is active"]
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
    #[doc = "No Non Acknowledge has not been detected since the last RSTNACK"]
    _0,
    #[doc = "At least one Non Acknowledge has been detected since the last RSTNACK"]
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
    #[doc = "No input change has been detected on the RI pin since the last read of CSR"]
    _0,
    #[doc = "At least one input change has been detected on the RI pin since the last read of CSR"]
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
    #[doc = "No input change has been detected on the DSR pin since the last read of CSR"]
    _0,
    #[doc = "At least one input change has been detected on the DSR pin since the last read of CSR"]
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
    #[doc = "No input change has been detected on the DCD pin since the last read of CSR"]
    _0,
    #[doc = "At least one input change has been detected on the DCD pin since the last read of CSR"]
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
    #[doc = "No input change has been detected on the CTS pin since the last read of CSR"]
    _0,
    #[doc = "At least one input change has been detected on the CTS pin since the last read of CSR"]
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
#[doc = "Possible values of the field `RI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIR {
    #[doc = "RI is at 0"]
    _0,
    #[doc = "RI is at 1"]
    _1,
}
impl RIR {
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
            RIR::_0 => false,
            RIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIR {
        match value {
            false => RIR::_0,
            true => RIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RIR::_1
    }
}
#[doc = "Possible values of the field `DSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRR {
    #[doc = "DSR is at 0"]
    _0,
    #[doc = "DSR is at 1"]
    _1,
}
impl DSRR {
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
            DSRR::_0 => false,
            DSRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSRR {
        match value {
            false => DSRR::_0,
            true => DSRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DSRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DSRR::_1
    }
}
#[doc = "Possible values of the field `DCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDR {
    #[doc = "DCD is at 0"]
    _0,
    #[doc = "DCD is at 1"]
    _1,
}
impl DCDR {
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
            DCDR::_0 => false,
            DCDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDR {
        match value {
            false => DCDR::_0,
            true => DCDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCDR::_1
    }
}
#[doc = "Possible values of the field `CTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSR {
    #[doc = "CTS is at 0"]
    _0,
    #[doc = "CTS is at 1"]
    _1,
}
impl CTSR {
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
            CTSR::_0 => false,
            CTSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSR {
        match value {
            false => CTSR::_0,
            true => CTSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CTSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CTSR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline]
    pub fn rxrdy(&self) -> RXRDYR {
        RXRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline]
    pub fn txrdy(&self) -> TXRDYR {
        TXRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Break Received/End of Break"]
    #[inline]
    pub fn rxbrk(&self) -> RXBRKR {
        RXBRKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline]
    pub fn ovre(&self) -> OVRER {
        OVRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline]
    pub fn frame(&self) -> FRAMER {
        FRAMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline]
    pub fn pare(&self) -> PARER {
        PARER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Receiver Time-out"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline]
    pub fn txempty(&self) -> TXEMPTYR {
        TXEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SPI Underrun Error"]
    #[inline]
    pub fn unre(&self) -> UNRER {
        UNRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmission Buffer Empty"]
    #[inline]
    pub fn txbufe(&self) -> TXBUFER {
        TXBUFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Reception Buffer Full"]
    #[inline]
    pub fn rxbuff(&self) -> RXBUFFR {
        RXBUFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Non Acknowledge"]
    #[inline]
    pub fn nack(&self) -> NACKR {
        NACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Flag"]
    #[inline]
    pub fn riic(&self) -> RIICR {
        RIICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Flag"]
    #[inline]
    pub fn dsric(&self) -> DSRICR {
        DSRICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Flag"]
    #[inline]
    pub fn dcdic(&self) -> DCDICR {
        DCDICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag"]
    #[inline]
    pub fn ctsic(&self) -> CTSICR {
        CTSICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Image of RI Input"]
    #[inline]
    pub fn ri(&self) -> RIR {
        RIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Image of DSR Input"]
    #[inline]
    pub fn dsr(&self) -> DSRR {
        DSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Image of DCD Input"]
    #[inline]
    pub fn dcd(&self) -> DCDR {
        DCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline]
    pub fn cts(&self) -> CTSR {
        CTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
