#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR_SPI_MODE {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Normal"]
    NORMAL,
    #[doc = "RS485"]
    RS485,
    #[doc = "Hardware Handshaking"]
    HARDWARE,
    #[doc = "Modem"]
    MODEM,
    #[doc = "IS07816 Protocol: T = 0"]
    ISO7816_T0,
    #[doc = "IS07816 Protocol: T = 1"]
    ISO7816_T1,
    #[doc = "IrDA"]
    IRDA,
    #[doc = "LIN Master"]
    LIN_MASTER,
    #[doc = "LIN Slave"]
    LIN_SLAVE,
    #[doc = "SPI Master"]
    SPI_MASTER,
    #[doc = "SPI Slave"]
    SPI_SLAVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NORMAL => 0,
            MODER::RS485 => 1,
            MODER::HARDWARE => 2,
            MODER::MODEM => 3,
            MODER::ISO7816_T0 => 4,
            MODER::ISO7816_T1 => 6,
            MODER::IRDA => 8,
            MODER::LIN_MASTER => 10,
            MODER::LIN_SLAVE => 11,
            MODER::SPI_MASTER => 14,
            MODER::SPI_SLAVE => 15,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NORMAL,
            1 => MODER::RS485,
            2 => MODER::HARDWARE,
            3 => MODER::MODEM,
            4 => MODER::ISO7816_T0,
            6 => MODER::ISO7816_T1,
            8 => MODER::IRDA,
            10 => MODER::LIN_MASTER,
            11 => MODER::LIN_SLAVE,
            14 => MODER::SPI_MASTER,
            15 => MODER::SPI_SLAVE,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == MODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline]
    pub fn is_rs485(&self) -> bool {
        *self == MODER::RS485
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline]
    pub fn is_hardware(&self) -> bool {
        *self == MODER::HARDWARE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline]
    pub fn is_modem(&self) -> bool {
        *self == MODER::MODEM
    }
    #[doc = "Checks if the value of the field is `ISO7816_T0`"]
    #[inline]
    pub fn is_iso7816_t0(&self) -> bool {
        *self == MODER::ISO7816_T0
    }
    #[doc = "Checks if the value of the field is `ISO7816_T1`"]
    #[inline]
    pub fn is_iso7816_t1(&self) -> bool {
        *self == MODER::ISO7816_T1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline]
    pub fn is_irda(&self) -> bool {
        *self == MODER::IRDA
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline]
    pub fn is_lin_master(&self) -> bool {
        *self == MODER::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline]
    pub fn is_lin_slave(&self) -> bool {
        *self == MODER::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline]
    pub fn is_spi_master(&self) -> bool {
        *self == MODER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODER::SPI_SLAVE
    }
}
#[doc = "Possible values of the field `USCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USCLKSR {
    #[doc = "MCK"]
    MCK,
    #[doc = "MCK / DIV"]
    MCK_DIV,
    #[doc = "SCK"]
    SCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USCLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USCLKSR::MCK => 0,
            USCLKSR::MCK_DIV => 1,
            USCLKSR::SCK => 3,
            USCLKSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USCLKSR {
        match value {
            0 => USCLKSR::MCK,
            1 => USCLKSR::MCK_DIV,
            3 => USCLKSR::SCK,
            i => USCLKSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == USCLKSR::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV`"]
    #[inline]
    pub fn is_mck_div(&self) -> bool {
        *self == USCLKSR::MCK_DIV
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline]
    pub fn is_sck(&self) -> bool {
        *self == USCLKSR::SCK
    }
}
#[doc = "Possible values of the field `CHRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRLR {
    #[doc = "5 bits"]
    _5,
    #[doc = "6 bits"]
    _6,
    #[doc = "7 bits"]
    _7,
    #[doc = "8 bits"]
    _8,
}
impl CHRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHRLR::_5 => 0,
            CHRLR::_6 => 1,
            CHRLR::_7 => 2,
            CHRLR::_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHRLR {
        match value {
            0 => CHRLR::_5,
            1 => CHRLR::_6,
            2 => CHRLR::_7,
            3 => CHRLR::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == CHRLR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == CHRLR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == CHRLR::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == CHRLR::_8
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    _0,
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    _1,
}
impl CPHAR {
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
            CPHAR::_0 => false,
            CPHAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::_0,
            true => CPHAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPHAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPHAR::_1
    }
}
#[doc = "Possible values of the field `PAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARR {
    #[doc = "Even parity"]
    EVEN,
    #[doc = "Odd parity"]
    ODD,
    #[doc = "Parity forced to 0 (Space)"]
    SPACE,
    #[doc = "Parity forced to 1 (Mark)"]
    MARK,
    #[doc = "No Parity"]
    NONE,
    #[doc = "No Parity"]
    _5,
    #[doc = "Multi-drop mode"]
    MULTI,
    #[doc = "Multi-drop mode"]
    _7,
}
impl PARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARR::EVEN => 0,
            PARR::ODD => 1,
            PARR::SPACE => 2,
            PARR::MARK => 3,
            PARR::NONE => 4,
            PARR::_5 => 5,
            PARR::MULTI => 6,
            PARR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARR {
        match value {
            0 => PARR::EVEN,
            1 => PARR::ODD,
            2 => PARR::SPACE,
            3 => PARR::MARK,
            4 => PARR::NONE,
            5 => PARR::_5,
            6 => PARR::MULTI,
            7 => PARR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PARR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PARR::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline]
    pub fn is_space(&self) -> bool {
        *self == PARR::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline]
    pub fn is_mark(&self) -> bool {
        *self == PARR::MARK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PARR::NONE
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == PARR::_5
    }
    #[doc = "Checks if the value of the field is `MULTI`"]
    #[inline]
    pub fn is_multi(&self) -> bool {
        *self == PARR::MULTI
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == PARR::_7
    }
}
#[doc = "Possible values of the field `NBSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBSTOPR {
    #[doc = "1 stop bit"]
    _1,
    #[doc = "1.5 stop bits (Only valid if SYNC=0)"]
    _1_5,
    #[doc = "2 stop bits"]
    _2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NBSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NBSTOPR::_1 => 0,
            NBSTOPR::_1_5 => 1,
            NBSTOPR::_2 => 2,
            NBSTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NBSTOPR {
        match value {
            0 => NBSTOPR::_1,
            1 => NBSTOPR::_1_5,
            2 => NBSTOPR::_2,
            i => NBSTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NBSTOPR::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline]
    pub fn is_1_5(&self) -> bool {
        *self == NBSTOPR::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == NBSTOPR::_2
    }
}
#[doc = "Possible values of the field `CHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMODER {
    #[doc = "Normal Mode"]
    NORMAL,
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin"]
    ECHO,
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input"]
    LOCAL_LOOP,
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin"]
    REMOTE_LOOP,
}
impl CHMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHMODER::NORMAL => 0,
            CHMODER::ECHO => 1,
            CHMODER::LOCAL_LOOP => 2,
            CHMODER::REMOTE_LOOP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHMODER {
        match value {
            0 => CHMODER::NORMAL,
            1 => CHMODER::ECHO,
            2 => CHMODER::LOCAL_LOOP,
            3 => CHMODER::REMOTE_LOOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == CHMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `ECHO`"]
    #[inline]
    pub fn is_echo(&self) -> bool {
        *self == CHMODER::ECHO
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOP`"]
    #[inline]
    pub fn is_local_loop(&self) -> bool {
        *self == CHMODER::LOCAL_LOOP
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOP`"]
    #[inline]
    pub fn is_remote_loop(&self) -> bool {
        *self == CHMODER::REMOTE_LOOP
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "The inactive state value of SPCK is logic level zero"]
    ZERO,
    #[doc = "The inactive state value of SPCK is logic level one"]
    ONE,
}
impl CPOLR {
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
            CPOLR::ZERO => false,
            CPOLR::ONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::ZERO,
            true => CPOLR::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CPOLR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CPOLR::ONE
    }
}
#[doc = "Possible values of the field `MODE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE9R {
    #[doc = "CHRL defines character length"]
    _0,
    #[doc = "9-bit character length"]
    _1,
}
impl MODE9R {
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
            MODE9R::_0 => false,
            MODE9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODE9R {
        match value {
            false => MODE9R::_0,
            true => MODE9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MODE9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MODE9R::_1
    }
}
#[doc = "Possible values of the field `CLKO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOR {
    #[doc = "The USART does not drive the SCK pin"]
    _0,
    #[doc = "The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    _1,
}
impl CLKOR {
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
            CLKOR::_0 => false,
            CLKOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKOR {
        match value {
            false => CLKOR::_0,
            true => CLKOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKOR::_1
    }
}
#[doc = "Possible values of the field `OVER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERR {
    #[doc = "16x Oversampling"]
    X16,
    #[doc = "8x Oversampling"]
    X8,
}
impl OVERR {
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
            OVERR::X16 => false,
            OVERR::X8 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERR {
        match value {
            false => OVERR::X16,
            true => OVERR::X8,
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline]
    pub fn is_x16(&self) -> bool {
        *self == OVERR::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline]
    pub fn is_x8(&self) -> bool {
        *self == OVERR::X8
    }
}
#[doc = "Possible values of the field `INACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INACKR {
    #[doc = "The NACK is generated"]
    _0,
    #[doc = "The NACK is not generated"]
    _1,
}
impl INACKR {
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
            INACKR::_0 => false,
            INACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INACKR {
        match value {
            false => INACKR::_0,
            true => INACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INACKR::_1
    }
}
#[doc = "Possible values of the field `DSNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSNACKR {
    #[doc = "NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    _0,
    #[doc = "Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    _1,
}
impl DSNACKR {
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
            DSNACKR::_0 => false,
            DSNACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSNACKR {
        match value {
            false => DSNACKR::_0,
            true => DSNACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DSNACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DSNACKR::_1
    }
}
#[doc = r" Value of the field"]
pub struct INVDATAR {
    bits: bool,
}
impl INVDATAR {
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
#[doc = r" Value of the field"]
pub struct MAX_ITERATIONR {
    bits: u8,
}
impl MAX_ITERATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERR {
    #[doc = "The USART does not filter the receive line"]
    _0,
    #[doc = "The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    _1,
}
impl FILTERR {
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
            FILTERR::_0 => false,
            FILTERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTERR {
        match value {
            false => FILTERR::_0,
            true => FILTERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTERR::_1
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Normal"]
    NORMAL,
    #[doc = "RS485"]
    RS485,
    #[doc = "Hardware Handshaking"]
    HARDWARE,
    #[doc = "Modem"]
    MODEM,
    #[doc = "IS07816 Protocol: T = 0"]
    ISO7816_T0,
    #[doc = "IS07816 Protocol: T = 1"]
    ISO7816_T1,
    #[doc = "IrDA"]
    IRDA,
    #[doc = "LIN Master"]
    LIN_MASTER,
    #[doc = "LIN Slave"]
    LIN_SLAVE,
    #[doc = "SPI Master"]
    SPI_MASTER,
    #[doc = "SPI Slave"]
    SPI_SLAVE,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NORMAL => 0,
            MODEW::RS485 => 1,
            MODEW::HARDWARE => 2,
            MODEW::MODEM => 3,
            MODEW::ISO7816_T0 => 4,
            MODEW::ISO7816_T1 => 6,
            MODEW::IRDA => 8,
            MODEW::LIN_MASTER => 10,
            MODEW::LIN_SLAVE => 11,
            MODEW::SPI_MASTER => 14,
            MODEW::SPI_SLAVE => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODEW::NORMAL)
    }
    #[doc = "RS485"]
    #[inline]
    pub fn rs485(self) -> &'a mut W {
        self.variant(MODEW::RS485)
    }
    #[doc = "Hardware Handshaking"]
    #[inline]
    pub fn hardware(self) -> &'a mut W {
        self.variant(MODEW::HARDWARE)
    }
    #[doc = "Modem"]
    #[inline]
    pub fn modem(self) -> &'a mut W {
        self.variant(MODEW::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline]
    pub fn iso7816_t0(self) -> &'a mut W {
        self.variant(MODEW::ISO7816_T0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline]
    pub fn iso7816_t1(self) -> &'a mut W {
        self.variant(MODEW::ISO7816_T1)
    }
    #[doc = "IrDA"]
    #[inline]
    pub fn irda(self) -> &'a mut W {
        self.variant(MODEW::IRDA)
    }
    #[doc = "LIN Master"]
    #[inline]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(MODEW::LIN_MASTER)
    }
    #[doc = "LIN Slave"]
    #[inline]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(MODEW::LIN_SLAVE)
    }
    #[doc = "SPI Master"]
    #[inline]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODEW::SPI_MASTER)
    }
    #[doc = "SPI Slave"]
    #[inline]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODEW::SPI_SLAVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USCLKS`"]
pub enum USCLKSW {
    #[doc = "MCK"]
    MCK,
    #[doc = "MCK / DIV"]
    MCK_DIV,
    #[doc = "SCK"]
    SCK,
}
impl USCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USCLKSW::MCK => 0,
            USCLKSW::MCK_DIV => 1,
            USCLKSW::SCK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _USCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USCLKSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MCK"]
    #[inline]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKSW::MCK)
    }
    #[doc = "MCK / DIV"]
    #[inline]
    pub fn mck_div(self) -> &'a mut W {
        self.variant(USCLKSW::MCK_DIV)
    }
    #[doc = "SCK"]
    #[inline]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKSW::SCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHRL`"]
pub enum CHRLW {
    #[doc = "5 bits"]
    _5,
    #[doc = "6 bits"]
    _6,
    #[doc = "7 bits"]
    _7,
    #[doc = "8 bits"]
    _8,
}
impl CHRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHRLW::_5 => 0,
            CHRLW::_6 => 1,
            CHRLW::_7 => 2,
            CHRLW::_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CHRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5 bits"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(CHRLW::_5)
    }
    #[doc = "6 bits"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(CHRLW::_6)
    }
    #[doc = "7 bits"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(CHRLW::_7)
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHRLW::_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    _0,
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    _1,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::_0 => false,
            CPHAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHAW::_0)
    }
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHAW::_1)
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
#[doc = "Values that can be written to the field `PAR`"]
pub enum PARW {
    #[doc = "Even parity"]
    EVEN,
    #[doc = "Odd parity"]
    ODD,
    #[doc = "Parity forced to 0 (Space)"]
    SPACE,
    #[doc = "Parity forced to 1 (Mark)"]
    MARK,
    #[doc = "No Parity"]
    NONE,
    #[doc = "No Parity"]
    _5,
    #[doc = "Multi-drop mode"]
    MULTI,
    #[doc = "Multi-drop mode"]
    _7,
}
impl PARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARW::EVEN => 0,
            PARW::ODD => 1,
            PARW::SPACE => 2,
            PARW::MARK => 3,
            PARW::NONE => 4,
            PARW::_5 => 5,
            PARW::MULTI => 6,
            PARW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARW<'a> {
    w: &'a mut W,
}
impl<'a> _PARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Even parity"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PARW::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARW::ODD)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline]
    pub fn space(self) -> &'a mut W {
        self.variant(PARW::SPACE)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARW::MARK)
    }
    #[doc = "No Parity"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PARW::NONE)
    }
    #[doc = "No Parity"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(PARW::_5)
    }
    #[doc = "Multi-drop mode"]
    #[inline]
    pub fn multi(self) -> &'a mut W {
        self.variant(PARW::MULTI)
    }
    #[doc = "Multi-drop mode"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(PARW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NBSTOP`"]
pub enum NBSTOPW {
    #[doc = "1 stop bit"]
    _1,
    #[doc = "1.5 stop bits (Only valid if SYNC=0)"]
    _1_5,
    #[doc = "2 stop bits"]
    _2,
}
impl NBSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NBSTOPW::_1 => 0,
            NBSTOPW::_1_5 => 1,
            NBSTOPW::_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NBSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _NBSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NBSTOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 stop bit"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NBSTOPW::_1)
    }
    #[doc = "1.5 stop bits (Only valid if SYNC=0)"]
    #[inline]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(NBSTOPW::_1_5)
    }
    #[doc = "2 stop bits"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NBSTOPW::_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHMODE`"]
pub enum CHMODEW {
    #[doc = "Normal Mode"]
    NORMAL,
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin"]
    ECHO,
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input"]
    LOCAL_LOOP,
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin"]
    REMOTE_LOOP,
}
impl CHMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHMODEW::NORMAL => 0,
            CHMODEW::ECHO => 1,
            CHMODEW::LOCAL_LOOP => 2,
            CHMODEW::REMOTE_LOOP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal Mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODEW::NORMAL)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin"]
    #[inline]
    pub fn echo(self) -> &'a mut W {
        self.variant(CHMODEW::ECHO)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input"]
    #[inline]
    pub fn local_loop(self) -> &'a mut W {
        self.variant(CHMODEW::LOCAL_LOOP)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin"]
    #[inline]
    pub fn remote_loop(self) -> &'a mut W {
        self.variant(CHMODEW::REMOTE_LOOP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "The inactive state value of SPCK is logic level zero"]
    ZERO,
    #[doc = "The inactive state value of SPCK is logic level one"]
    ONE,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::ZERO => false,
            CPOLW::ONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The inactive state value of SPCK is logic level zero"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CPOLW::ZERO)
    }
    #[doc = "The inactive state value of SPCK is logic level one"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CPOLW::ONE)
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
#[doc = "Values that can be written to the field `MODE9`"]
pub enum MODE9W {
    #[doc = "CHRL defines character length"]
    _0,
    #[doc = "9-bit character length"]
    _1,
}
impl MODE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODE9W::_0 => false,
            MODE9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE9W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CHRL defines character length"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE9W::_0)
    }
    #[doc = "9-bit character length"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE9W::_1)
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
#[doc = "Values that can be written to the field `CLKO`"]
pub enum CLKOW {
    #[doc = "The USART does not drive the SCK pin"]
    _0,
    #[doc = "The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    _1,
}
impl CLKOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKOW::_0 => false,
            CLKOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The USART does not drive the SCK pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOW::_0)
    }
    #[doc = "The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOW::_1)
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
#[doc = "Values that can be written to the field `OVER`"]
pub enum OVERW {
    #[doc = "16x Oversampling"]
    X16,
    #[doc = "8x Oversampling"]
    X8,
}
impl OVERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERW::X16 => false,
            OVERW::X8 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16x Oversampling"]
    #[inline]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVERW::X16)
    }
    #[doc = "8x Oversampling"]
    #[inline]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVERW::X8)
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
#[doc = "Values that can be written to the field `INACK`"]
pub enum INACKW {
    #[doc = "The NACK is generated"]
    _0,
    #[doc = "The NACK is not generated"]
    _1,
}
impl INACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INACKW::_0 => false,
            INACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INACKW<'a> {
    w: &'a mut W,
}
impl<'a> _INACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The NACK is generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INACKW::_0)
    }
    #[doc = "The NACK is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INACKW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSNACK`"]
pub enum DSNACKW {
    #[doc = "NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    _0,
    #[doc = "Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    _1,
}
impl DSNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSNACKW::_0 => false,
            DSNACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DSNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSNACKW::_0)
    }
    #[doc = "Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSNACKW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _INVDATAW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_ITERATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_ITERATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTER`"]
pub enum FILTERW {
    #[doc = "The USART does not filter the receive line"]
    _0,
    #[doc = "The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    _1,
}
impl FILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTERW::_0 => false,
            FILTERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The USART does not filter the receive line"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTERW::_0)
    }
    #[doc = "The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTERW::_1)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Usart Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline]
    pub fn usclks(&self) -> USCLKSR {
        USCLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline]
    pub fn chrl(&self) -> CHRLR {
        CHRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - SPI CLock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline]
    pub fn par(&self) -> PARR {
        PARR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline]
    pub fn nbstop(&self) -> NBSTOPR {
        NBSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline]
    pub fn chmode(&self) -> CHMODER {
        CHMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline]
    pub fn mode9(&self) -> MODE9R {
        MODE9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline]
    pub fn clko(&self) -> CLKOR {
        CLKOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline]
    pub fn over(&self) -> OVERR {
        OVERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline]
    pub fn inack(&self) -> INACKR {
        INACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline]
    pub fn dsnack(&self) -> DSNACKR {
        DSNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Inverted data"]
    #[inline]
    pub fn invdata(&self) -> INVDATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVDATAR { bits }
    }
    #[doc = "Bits 24:26 - Max interation"]
    #[inline]
    pub fn max_iteration(&self) -> MAX_ITERATIONR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_ITERATIONR { bits }
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline]
    pub fn filter(&self) -> FILTERR {
        FILTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:3 - Usart Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline]
    pub fn usclks(&mut self) -> _USCLKSW {
        _USCLKSW { w: self }
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline]
    pub fn chrl(&mut self) -> _CHRLW {
        _CHRLW { w: self }
    }
    #[doc = "Bit 8 - SPI CLock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline]
    pub fn par(&mut self) -> _PARW {
        _PARW { w: self }
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline]
    pub fn nbstop(&mut self) -> _NBSTOPW {
        _NBSTOPW { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline]
    pub fn chmode(&mut self) -> _CHMODEW {
        _CHMODEW { w: self }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline]
    pub fn mode9(&mut self) -> _MODE9W {
        _MODE9W { w: self }
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline]
    pub fn clko(&mut self) -> _CLKOW {
        _CLKOW { w: self }
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline]
    pub fn over(&mut self) -> _OVERW {
        _OVERW { w: self }
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline]
    pub fn inack(&mut self) -> _INACKW {
        _INACKW { w: self }
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline]
    pub fn dsnack(&mut self) -> _DSNACKW {
        _DSNACKW { w: self }
    }
    #[doc = "Bit 23 - Inverted data"]
    #[inline]
    pub fn invdata(&mut self) -> _INVDATAW {
        _INVDATAW { w: self }
    }
    #[doc = "Bits 24:26 - Max interation"]
    #[inline]
    pub fn max_iteration(&mut self) -> _MAX_ITERATIONW {
        _MAX_ITERATIONW { w: self }
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
}
