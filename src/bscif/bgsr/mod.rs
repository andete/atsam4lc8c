#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BGSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BGBUFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBUFRDYR {
    #[doc = "undocumented"]
    FLASH,
    #[doc = "undocumented"]
    PLL,
    #[doc = "undocumented"]
    VREG,
    #[doc = "undocumented"]
    BUFRR,
    #[doc = "undocumented"]
    ADC,
    #[doc = "undocumented"]
    LCD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BGBUFRDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BGBUFRDYR::FLASH => 1,
            BGBUFRDYR::PLL => 2,
            BGBUFRDYR::VREG => 4,
            BGBUFRDYR::BUFRR => 8,
            BGBUFRDYR::ADC => 16,
            BGBUFRDYR::LCD => 32,
            BGBUFRDYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BGBUFRDYR {
        match value {
            1 => BGBUFRDYR::FLASH,
            2 => BGBUFRDYR::PLL,
            4 => BGBUFRDYR::VREG,
            8 => BGBUFRDYR::BUFRR,
            16 => BGBUFRDYR::ADC,
            32 => BGBUFRDYR::LCD,
            i => BGBUFRDYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline]
    pub fn is_flash(&self) -> bool {
        *self == BGBUFRDYR::FLASH
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == BGBUFRDYR::PLL
    }
    #[doc = "Checks if the value of the field is `VREG`"]
    #[inline]
    pub fn is_vreg(&self) -> bool {
        *self == BGBUFRDYR::VREG
    }
    #[doc = "Checks if the value of the field is `BUFRR`"]
    #[inline]
    pub fn is_bufrr(&self) -> bool {
        *self == BGBUFRDYR::BUFRR
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline]
    pub fn is_adc(&self) -> bool {
        *self == BGBUFRDYR::ADC
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline]
    pub fn is_lcd(&self) -> bool {
        *self == BGBUFRDYR::LCD
    }
}
#[doc = r" Value of the field"]
pub struct BGRDYR {
    bits: bool,
}
impl BGRDYR {
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
pub struct LPBGRDYR {
    bits: bool,
}
impl LPBGRDYR {
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
pub struct VREFR {
    bits: u8,
}
impl VREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Bandgap Buffer Ready"]
    #[inline]
    pub fn bgbufrdy(&self) -> BGBUFRDYR {
        BGBUFRDYR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Bandgap Voltage Reference Ready"]
    #[inline]
    pub fn bgrdy(&self) -> BGRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BGRDYR { bits }
    }
    #[doc = "Bit 17 - Low Power Bandgap Voltage Reference Ready"]
    #[inline]
    pub fn lpbgrdy(&self) -> LPBGRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPBGRDYR { bits }
    }
    #[doc = "Bits 18:19 - Voltage Reference Used by the System"]
    #[inline]
    pub fn vref(&self) -> VREFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VREFR { bits }
    }
}
