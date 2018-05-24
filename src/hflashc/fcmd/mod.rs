#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCMD {
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
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "No Operation"]
    NOP,
    #[doc = "Write Page"]
    WP,
    #[doc = "Erase Page"]
    EP,
    #[doc = "Clear Page Buffer"]
    CPB,
    #[doc = "Lock Region containing page"]
    LP,
    #[doc = "Unlock Region containing page"]
    UP,
    #[doc = "Erase All, including secuity and fuse bits"]
    EA,
    #[doc = "Write General-Purpose fuse Bit"]
    WGPB,
    #[doc = "Erase General-Purpose fuse Bit"]
    EGPB,
    #[doc = "Set Security Bit"]
    SSB,
    #[doc = "Program GPFuse Byte"]
    PGPFB,
    #[doc = "Erase All GP Fuses"]
    EAGPF,
    #[doc = "Quick Page Read"]
    QPR,
    #[doc = "Write User Page"]
    WUP,
    #[doc = "Erase User Page"]
    EUP,
    #[doc = "Quick Page Read User Page"]
    QPRUP,
    #[doc = "High Speed Mode Enable"]
    HSEN,
    #[doc = "High Speed Mode Disable"]
    HSDIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::NOP => 0,
            CMDR::WP => 1,
            CMDR::EP => 2,
            CMDR::CPB => 3,
            CMDR::LP => 4,
            CMDR::UP => 5,
            CMDR::EA => 6,
            CMDR::WGPB => 7,
            CMDR::EGPB => 8,
            CMDR::SSB => 9,
            CMDR::PGPFB => 10,
            CMDR::EAGPF => 11,
            CMDR::QPR => 12,
            CMDR::WUP => 13,
            CMDR::EUP => 14,
            CMDR::QPRUP => 15,
            CMDR::HSEN => 16,
            CMDR::HSDIS => 17,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            0 => CMDR::NOP,
            1 => CMDR::WP,
            2 => CMDR::EP,
            3 => CMDR::CPB,
            4 => CMDR::LP,
            5 => CMDR::UP,
            6 => CMDR::EA,
            7 => CMDR::WGPB,
            8 => CMDR::EGPB,
            9 => CMDR::SSB,
            10 => CMDR::PGPFB,
            11 => CMDR::EAGPF,
            12 => CMDR::QPR,
            13 => CMDR::WUP,
            14 => CMDR::EUP,
            15 => CMDR::QPRUP,
            16 => CMDR::HSEN,
            17 => CMDR::HSDIS,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == CMDR::NOP
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline]
    pub fn is_wp(&self) -> bool {
        *self == CMDR::WP
    }
    #[doc = "Checks if the value of the field is `EP`"]
    #[inline]
    pub fn is_ep(&self) -> bool {
        *self == CMDR::EP
    }
    #[doc = "Checks if the value of the field is `CPB`"]
    #[inline]
    pub fn is_cpb(&self) -> bool {
        *self == CMDR::CPB
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline]
    pub fn is_lp(&self) -> bool {
        *self == CMDR::LP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == CMDR::UP
    }
    #[doc = "Checks if the value of the field is `EA`"]
    #[inline]
    pub fn is_ea(&self) -> bool {
        *self == CMDR::EA
    }
    #[doc = "Checks if the value of the field is `WGPB`"]
    #[inline]
    pub fn is_wgpb(&self) -> bool {
        *self == CMDR::WGPB
    }
    #[doc = "Checks if the value of the field is `EGPB`"]
    #[inline]
    pub fn is_egpb(&self) -> bool {
        *self == CMDR::EGPB
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline]
    pub fn is_ssb(&self) -> bool {
        *self == CMDR::SSB
    }
    #[doc = "Checks if the value of the field is `PGPFB`"]
    #[inline]
    pub fn is_pgpfb(&self) -> bool {
        *self == CMDR::PGPFB
    }
    #[doc = "Checks if the value of the field is `EAGPF`"]
    #[inline]
    pub fn is_eagpf(&self) -> bool {
        *self == CMDR::EAGPF
    }
    #[doc = "Checks if the value of the field is `QPR`"]
    #[inline]
    pub fn is_qpr(&self) -> bool {
        *self == CMDR::QPR
    }
    #[doc = "Checks if the value of the field is `WUP`"]
    #[inline]
    pub fn is_wup(&self) -> bool {
        *self == CMDR::WUP
    }
    #[doc = "Checks if the value of the field is `EUP`"]
    #[inline]
    pub fn is_eup(&self) -> bool {
        *self == CMDR::EUP
    }
    #[doc = "Checks if the value of the field is `QPRUP`"]
    #[inline]
    pub fn is_qprup(&self) -> bool {
        *self == CMDR::QPRUP
    }
    #[doc = "Checks if the value of the field is `HSEN`"]
    #[inline]
    pub fn is_hsen(&self) -> bool {
        *self == CMDR::HSEN
    }
    #[doc = "Checks if the value of the field is `HSDIS`"]
    #[inline]
    pub fn is_hsdis(&self) -> bool {
        *self == CMDR::HSDIS
    }
}
#[doc = r" Value of the field"]
pub struct PAGENR {
    bits: u16,
}
impl PAGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "undocumented"]
    KEY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYR::KEY => 165,
            KEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYR {
        match value {
            165 => KEYR::KEY,
            i => KEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline]
    pub fn is_key(&self) -> bool {
        *self == KEYR::KEY
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "No Operation"]
    NOP,
    #[doc = "Write Page"]
    WP,
    #[doc = "Erase Page"]
    EP,
    #[doc = "Clear Page Buffer"]
    CPB,
    #[doc = "Lock Region containing page"]
    LP,
    #[doc = "Unlock Region containing page"]
    UP,
    #[doc = "Erase All, including secuity and fuse bits"]
    EA,
    #[doc = "Write General-Purpose fuse Bit"]
    WGPB,
    #[doc = "Erase General-Purpose fuse Bit"]
    EGPB,
    #[doc = "Set Security Bit"]
    SSB,
    #[doc = "Program GPFuse Byte"]
    PGPFB,
    #[doc = "Erase All GP Fuses"]
    EAGPF,
    #[doc = "Quick Page Read"]
    QPR,
    #[doc = "Write User Page"]
    WUP,
    #[doc = "Erase User Page"]
    EUP,
    #[doc = "Quick Page Read User Page"]
    QPRUP,
    #[doc = "High Speed Mode Enable"]
    HSEN,
    #[doc = "High Speed Mode Disable"]
    HSDIS,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::NOP => 0,
            CMDW::WP => 1,
            CMDW::EP => 2,
            CMDW::CPB => 3,
            CMDW::LP => 4,
            CMDW::UP => 5,
            CMDW::EA => 6,
            CMDW::WGPB => 7,
            CMDW::EGPB => 8,
            CMDW::SSB => 9,
            CMDW::PGPFB => 10,
            CMDW::EAGPF => 11,
            CMDW::QPR => 12,
            CMDW::WUP => 13,
            CMDW::EUP => 14,
            CMDW::QPRUP => 15,
            CMDW::HSEN => 16,
            CMDW::HSDIS => 17,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Operation"]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(CMDW::NOP)
    }
    #[doc = "Write Page"]
    #[inline]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDW::WP)
    }
    #[doc = "Erase Page"]
    #[inline]
    pub fn ep(self) -> &'a mut W {
        self.variant(CMDW::EP)
    }
    #[doc = "Clear Page Buffer"]
    #[inline]
    pub fn cpb(self) -> &'a mut W {
        self.variant(CMDW::CPB)
    }
    #[doc = "Lock Region containing page"]
    #[inline]
    pub fn lp(self) -> &'a mut W {
        self.variant(CMDW::LP)
    }
    #[doc = "Unlock Region containing page"]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(CMDW::UP)
    }
    #[doc = "Erase All, including secuity and fuse bits"]
    #[inline]
    pub fn ea(self) -> &'a mut W {
        self.variant(CMDW::EA)
    }
    #[doc = "Write General-Purpose fuse Bit"]
    #[inline]
    pub fn wgpb(self) -> &'a mut W {
        self.variant(CMDW::WGPB)
    }
    #[doc = "Erase General-Purpose fuse Bit"]
    #[inline]
    pub fn egpb(self) -> &'a mut W {
        self.variant(CMDW::EGPB)
    }
    #[doc = "Set Security Bit"]
    #[inline]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMDW::SSB)
    }
    #[doc = "Program GPFuse Byte"]
    #[inline]
    pub fn pgpfb(self) -> &'a mut W {
        self.variant(CMDW::PGPFB)
    }
    #[doc = "Erase All GP Fuses"]
    #[inline]
    pub fn eagpf(self) -> &'a mut W {
        self.variant(CMDW::EAGPF)
    }
    #[doc = "Quick Page Read"]
    #[inline]
    pub fn qpr(self) -> &'a mut W {
        self.variant(CMDW::QPR)
    }
    #[doc = "Write User Page"]
    #[inline]
    pub fn wup(self) -> &'a mut W {
        self.variant(CMDW::WUP)
    }
    #[doc = "Erase User Page"]
    #[inline]
    pub fn eup(self) -> &'a mut W {
        self.variant(CMDW::EUP)
    }
    #[doc = "Quick Page Read User Page"]
    #[inline]
    pub fn qprup(self) -> &'a mut W {
        self.variant(CMDW::QPRUP)
    }
    #[doc = "High Speed Mode Enable"]
    #[inline]
    pub fn hsen(self) -> &'a mut W {
        self.variant(CMDW::HSEN)
    }
    #[doc = "High Speed Mode Disable"]
    #[inline]
    pub fn hsdis(self) -> &'a mut W {
        self.variant(CMDW::HSDIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAGENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAGENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KEY`"]
pub enum KEYW {
    #[doc = "`10100101`"]
    KEY,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYW::KEY => 165,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`10100101`"]
    #[inline]
    pub fn key(self) -> &'a mut W {
        self.variant(KEYW::KEY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:23 - Page number"]
    #[inline]
    pub fn pagen(&self) -> PAGENR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PAGENR { bits }
    }
    #[doc = "Bits 24:31 - Write protection key"]
    #[inline]
    pub fn key(&self) -> KEYR {
        KEYR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
    #[doc = "Bits 8:23 - Page number"]
    #[inline]
    pub fn pagen(&mut self) -> _PAGENW {
        _PAGENW { w: self }
    }
    #[doc = "Bits 24:31 - Write protection key"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
