#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENR {
    #[doc = "Receiver is effectively disabled, following a CR.RXDIS or CR.SWRST request"]
    OFF,
    #[doc = "Receiver is effectively enabled, following a CR.RXEN request"]
    ON,
}
impl RXENR {
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
            RXENR::OFF => false,
            RXENR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXENR {
        match value {
            false => RXENR::OFF,
            true => RXENR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RXENR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == RXENR::ON
    }
}
#[doc = "Possible values of the field `RXRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYR {
    #[doc = "The register RHR is empty and can't be read"]
    EMPTY,
    #[doc = "The register RHR is full and is ready to be read"]
    FULL,
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
            RXRDYR::EMPTY => false,
            RXRDYR::FULL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXRDYR {
        match value {
            false => RXRDYR::EMPTY,
            true => RXRDYR::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == RXRDYR::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == RXRDYR::FULL
    }
}
#[doc = "Possible values of the field `RXOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXORR {
    #[doc = "No overrun"]
    NO,
    #[doc = "The previous received data has not been read. This data is lost"]
    YES,
}
impl RXORR {
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
            RXORR::NO => false,
            RXORR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXORR {
        match value {
            false => RXORR::NO,
            true => RXORR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == RXORR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == RXORR::YES
    }
}
#[doc = "Possible values of the field `TXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENR {
    #[doc = "Transmitter is effectively disabled, following a CR.TXDIS or CR.SWRST request"]
    OFF,
    #[doc = "Transmitter is effectively enabled, following a CR.TXEN request"]
    ON,
}
impl TXENR {
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
            TXENR::OFF => false,
            TXENR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXENR {
        match value {
            false => TXENR::OFF,
            true => TXENR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == TXENR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == TXENR::ON
    }
}
#[doc = "Possible values of the field `TXRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYR {
    #[doc = "The register THR is full and can't be written"]
    FULL,
    #[doc = "The register THR is empty and is ready to be written"]
    EMPTY,
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
            TXRDYR::FULL => false,
            TXRDYR::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRDYR {
        match value {
            false => TXRDYR::FULL,
            true => TXRDYR::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == TXRDYR::FULL
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == TXRDYR::EMPTY
    }
}
#[doc = "Possible values of the field `TXUR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXURR {
    #[doc = "No underrun"]
    NO,
    #[doc = "The last bit of the last data written to the register THR has been set. Until the next write to THR, data will be sent according to MR.TXSAME field"]
    YES,
}
impl TXURR {
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
            TXURR::NO => false,
            TXURR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXURR {
        match value {
            false => TXURR::NO,
            true => TXURR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == TXURR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == TXURR::YES
    }
}
#[doc = "Possible values of the field `RXORCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXORCHR {
    #[doc = "Overrun first occurred on left channel"]
    LEFT,
    #[doc = "Overrun first occurred on right channel"]
    RIGHT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXORCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXORCHR::LEFT => 0,
            RXORCHR::RIGHT => 1,
            RXORCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXORCHR {
        match value {
            0 => RXORCHR::LEFT,
            1 => RXORCHR::RIGHT,
            i => RXORCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == RXORCHR::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == RXORCHR::RIGHT
    }
}
#[doc = "Possible values of the field `TXURCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXURCHR {
    #[doc = "Underrun first occurred on left channel"]
    LEFT,
    #[doc = "Underrun first occurred on right channel"]
    RIGHT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXURCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXURCHR::LEFT => 0,
            TXURCHR::RIGHT => 1,
            TXURCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXURCHR {
        match value {
            0 => TXURCHR::LEFT,
            1 => TXURCHR::RIGHT,
            i => TXURCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == TXURCHR::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == TXURCHR::RIGHT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Enable"]
    #[inline]
    pub fn rxen(&self) -> RXENR {
        RXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline]
    pub fn rxrdy(&self) -> RXRDYR {
        RXRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline]
    pub fn rxor(&self) -> RXORR {
        RXORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline]
    pub fn txen(&self) -> TXENR {
        TXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline]
    pub fn txrdy(&self) -> TXRDYR {
        TXRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline]
    pub fn txur(&self) -> TXURR {
        TXURR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Receive Overrun Channels"]
    #[inline]
    pub fn rxorch(&self) -> RXORCHR {
        RXORCHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channels"]
    #[inline]
    pub fn txurch(&self) -> TXURCHR {
        TXURCHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
