#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBFSM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DRDSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDSTATER {
    #[doc = "undocumented"]
    A_IDLE,
    #[doc = "undocumented"]
    A_WAIT_VRISE,
    #[doc = "undocumented"]
    A_WAIT_BCON,
    #[doc = "undocumented"]
    A_HOST,
    #[doc = "undocumented"]
    A_SUSPEND,
    #[doc = "undocumented"]
    A_PERIPHERAL,
    #[doc = "undocumented"]
    A_WAIT_VFALL,
    #[doc = "undocumented"]
    A_VBUS_ERR,
    #[doc = "undocumented"]
    A_WAIT_DISCHARGE,
    #[doc = "undocumented"]
    B_IDLE,
    #[doc = "undocumented"]
    B_PERIPHERAL,
    #[doc = "undocumented"]
    B_WAIT_BEGIN_HNP,
    #[doc = "undocumented"]
    B_WAIT_DISCHARGE,
    #[doc = "undocumented"]
    B_WAIT_ACON,
    #[doc = "undocumented"]
    B_HOST,
    #[doc = "undocumented"]
    B_SRP_INIT,
}
impl DRDSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRDSTATER::A_IDLE => 0,
            DRDSTATER::A_WAIT_VRISE => 1,
            DRDSTATER::A_WAIT_BCON => 2,
            DRDSTATER::A_HOST => 3,
            DRDSTATER::A_SUSPEND => 4,
            DRDSTATER::A_PERIPHERAL => 5,
            DRDSTATER::A_WAIT_VFALL => 6,
            DRDSTATER::A_VBUS_ERR => 7,
            DRDSTATER::A_WAIT_DISCHARGE => 8,
            DRDSTATER::B_IDLE => 9,
            DRDSTATER::B_PERIPHERAL => 10,
            DRDSTATER::B_WAIT_BEGIN_HNP => 11,
            DRDSTATER::B_WAIT_DISCHARGE => 12,
            DRDSTATER::B_WAIT_ACON => 13,
            DRDSTATER::B_HOST => 14,
            DRDSTATER::B_SRP_INIT => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRDSTATER {
        match value {
            0 => DRDSTATER::A_IDLE,
            1 => DRDSTATER::A_WAIT_VRISE,
            2 => DRDSTATER::A_WAIT_BCON,
            3 => DRDSTATER::A_HOST,
            4 => DRDSTATER::A_SUSPEND,
            5 => DRDSTATER::A_PERIPHERAL,
            6 => DRDSTATER::A_WAIT_VFALL,
            7 => DRDSTATER::A_VBUS_ERR,
            8 => DRDSTATER::A_WAIT_DISCHARGE,
            9 => DRDSTATER::B_IDLE,
            10 => DRDSTATER::B_PERIPHERAL,
            11 => DRDSTATER::B_WAIT_BEGIN_HNP,
            12 => DRDSTATER::B_WAIT_DISCHARGE,
            13 => DRDSTATER::B_WAIT_ACON,
            14 => DRDSTATER::B_HOST,
            15 => DRDSTATER::B_SRP_INIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A_IDLE`"]
    #[inline]
    pub fn is_a_idle(&self) -> bool {
        *self == DRDSTATER::A_IDLE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VRISE`"]
    #[inline]
    pub fn is_a_wait_vrise(&self) -> bool {
        *self == DRDSTATER::A_WAIT_VRISE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_BCON`"]
    #[inline]
    pub fn is_a_wait_bcon(&self) -> bool {
        *self == DRDSTATER::A_WAIT_BCON
    }
    #[doc = "Checks if the value of the field is `A_HOST`"]
    #[inline]
    pub fn is_a_host(&self) -> bool {
        *self == DRDSTATER::A_HOST
    }
    #[doc = "Checks if the value of the field is `A_SUSPEND`"]
    #[inline]
    pub fn is_a_suspend(&self) -> bool {
        *self == DRDSTATER::A_SUSPEND
    }
    #[doc = "Checks if the value of the field is `A_PERIPHERAL`"]
    #[inline]
    pub fn is_a_peripheral(&self) -> bool {
        *self == DRDSTATER::A_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VFALL`"]
    #[inline]
    pub fn is_a_wait_vfall(&self) -> bool {
        *self == DRDSTATER::A_WAIT_VFALL
    }
    #[doc = "Checks if the value of the field is `A_VBUS_ERR`"]
    #[inline]
    pub fn is_a_vbus_err(&self) -> bool {
        *self == DRDSTATER::A_VBUS_ERR
    }
    #[doc = "Checks if the value of the field is `A_WAIT_DISCHARGE`"]
    #[inline]
    pub fn is_a_wait_discharge(&self) -> bool {
        *self == DRDSTATER::A_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_IDLE`"]
    #[inline]
    pub fn is_b_idle(&self) -> bool {
        *self == DRDSTATER::B_IDLE
    }
    #[doc = "Checks if the value of the field is `B_PERIPHERAL`"]
    #[inline]
    pub fn is_b_peripheral(&self) -> bool {
        *self == DRDSTATER::B_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `B_WAIT_BEGIN_HNP`"]
    #[inline]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        *self == DRDSTATER::B_WAIT_BEGIN_HNP
    }
    #[doc = "Checks if the value of the field is `B_WAIT_DISCHARGE`"]
    #[inline]
    pub fn is_b_wait_discharge(&self) -> bool {
        *self == DRDSTATER::B_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_WAIT_ACON`"]
    #[inline]
    pub fn is_b_wait_acon(&self) -> bool {
        *self == DRDSTATER::B_WAIT_ACON
    }
    #[doc = "Checks if the value of the field is `B_HOST`"]
    #[inline]
    pub fn is_b_host(&self) -> bool {
        *self == DRDSTATER::B_HOST
    }
    #[doc = "Checks if the value of the field is `B_SRP_INIT`"]
    #[inline]
    pub fn is_b_srp_init(&self) -> bool {
        *self == DRDSTATER::B_SRP_INIT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - DualRoleDevice state"]
    #[inline]
    pub fn drdstate(&self) -> DRDSTATER {
        DRDSTATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
