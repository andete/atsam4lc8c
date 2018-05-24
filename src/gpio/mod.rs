#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Enable Register"]
    pub gper0: GPER,
    #[doc = "0x04 - GPIO Enable Register - Set"]
    pub gpers0: GPERS,
    #[doc = "0x08 - GPIO Enable Register - Clear"]
    pub gperc0: GPERC,
    #[doc = "0x0c - GPIO Enable Register - Toggle"]
    pub gpert0: GPERT,
    #[doc = "0x10 - Peripheral Mux Register 0"]
    pub pmr00: PMR0,
    #[doc = "0x14 - Peripheral Mux Register 0 - Set"]
    pub pmr0s0: PMR0S,
    #[doc = "0x18 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c0: PMR0C,
    #[doc = "0x1c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t0: PMR0T,
    #[doc = "0x20 - Peripheral Mux Register 1"]
    pub pmr10: PMR1,
    #[doc = "0x24 - Peripheral Mux Register 1 - Set"]
    pub pmr1s0: PMR1S,
    #[doc = "0x28 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c0: PMR1C,
    #[doc = "0x2c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t0: PMR1T,
    #[doc = "0x30 - Peripheral Mux Register 2"]
    pub pmr20: PMR2,
    #[doc = "0x34 - Peripheral Mux Register 2 - Set"]
    pub pmr2s0: PMR2S,
    #[doc = "0x38 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c0: PMR2C,
    #[doc = "0x3c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t0: PMR2T,
    #[doc = "0x40 - Output Driver Enable Register"]
    pub oder0: ODER,
    #[doc = "0x44 - Output Driver Enable Register - Set"]
    pub oders0: ODERS,
    #[doc = "0x48 - Output Driver Enable Register - Clear"]
    pub oderc0: ODERC,
    #[doc = "0x4c - Output Driver Enable Register - Toggle"]
    pub odert0: ODERT,
    #[doc = "0x50 - Output Value Register"]
    pub ovr0: OVR,
    #[doc = "0x54 - Output Value Register - Set"]
    pub ovrs0: OVRS,
    #[doc = "0x58 - Output Value Register - Clear"]
    pub ovrc0: OVRC,
    #[doc = "0x5c - Output Value Register - Toggle"]
    pub ovrt0: OVRT,
    #[doc = "0x60 - Pin Value Register"]
    pub pvr0: PVR,
    _reserved0: [u8; 12usize],
    #[doc = "0x70 - Pull-up Enable Register"]
    pub puer0: PUER,
    #[doc = "0x74 - Pull-up Enable Register - Set"]
    pub puers0: PUERS,
    #[doc = "0x78 - Pull-up Enable Register - Clear"]
    pub puerc0: PUERC,
    #[doc = "0x7c - Pull-up Enable Register - Toggle"]
    pub puert0: PUERT,
    #[doc = "0x80 - Pull-down Enable Register"]
    pub pder0: PDER,
    #[doc = "0x84 - Pull-down Enable Register - Set"]
    pub pders0: PDERS,
    #[doc = "0x88 - Pull-down Enable Register - Clear"]
    pub pderc0: PDERC,
    #[doc = "0x8c - Pull-down Enable Register - Toggle"]
    pub pdert0: PDERT,
    #[doc = "0x90 - Interrupt Enable Register"]
    pub ier0: IER,
    #[doc = "0x94 - Interrupt Enable Register - Set"]
    pub iers0: IERS,
    #[doc = "0x98 - Interrupt Enable Register - Clear"]
    pub ierc0: IERC,
    #[doc = "0x9c - Interrupt Enable Register - Toggle"]
    pub iert0: IERT,
    #[doc = "0xa0 - Interrupt Mode Register 0"]
    pub imr00: IMR0,
    #[doc = "0xa4 - Interrupt Mode Register 0 - Set"]
    pub imr0s0: IMR0S,
    #[doc = "0xa8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c0: IMR0C,
    #[doc = "0xac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t0: IMR0T,
    #[doc = "0xb0 - Interrupt Mode Register 1"]
    pub imr10: IMR1,
    #[doc = "0xb4 - Interrupt Mode Register 1 - Set"]
    pub imr1s0: IMR1S,
    #[doc = "0xb8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c0: IMR1C,
    #[doc = "0xbc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t0: IMR1T,
    #[doc = "0xc0 - Glitch Filter Enable Register"]
    pub gfer0: GFER,
    #[doc = "0xc4 - Glitch Filter Enable Register - Set"]
    pub gfers0: GFERS,
    #[doc = "0xc8 - Glitch Filter Enable Register - Clear"]
    pub gferc0: GFERC,
    #[doc = "0xcc - Glitch Filter Enable Register - Toggle"]
    pub gfert0: GFERT,
    #[doc = "0xd0 - Interrupt Flag Register"]
    pub ifr0: IFR,
    _reserved1: [u8; 4usize],
    #[doc = "0xd8 - Interrupt Flag Register - Clear"]
    pub ifrc0: IFRC,
    _reserved2: [u8; 4usize],
    #[doc = "0xe0 - Open Drain Mode Register"]
    pub odmer0: ODMER,
    #[doc = "0xe4 - Open Drain Mode Register - Set"]
    pub odmers0: ODMERS,
    #[doc = "0xe8 - Open Drain Mode Register - Clear"]
    pub odmerc0: ODMERC,
    #[doc = "0xec - Open Drain Mode Register - Toggle"]
    pub odmert0: ODMERT,
    _reserved3: [u8; 16usize],
    #[doc = "0x100 - Output Driving Capability Register 0"]
    pub odcr00: ODCR0,
    #[doc = "0x104 - Output Driving Capability Register 0 - Set"]
    pub odcr0s0: ODCR0S,
    #[doc = "0x108 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c0: ODCR0C,
    #[doc = "0x10c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t0: ODCR0T,
    #[doc = "0x110 - Output Driving Capability Register 1"]
    pub odcr10: ODCR1,
    #[doc = "0x114 - Output Driving Capability Register 1 - Set"]
    pub odcr1s0: ODCR1S,
    #[doc = "0x118 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c0: ODCR1C,
    #[doc = "0x11c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t0: ODCR1T,
    _reserved4: [u8; 16usize],
    #[doc = "0x130 - Output Slew Rate Register 0"]
    pub osrr00: OSRR0,
    #[doc = "0x134 - Output Slew Rate Register 0 - Set"]
    pub osrr0s0: OSRR0S,
    #[doc = "0x138 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c0: OSRR0C,
    #[doc = "0x13c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t0: OSRR0T,
    _reserved5: [u8; 32usize],
    #[doc = "0x160 - Schmitt Trigger Enable Register"]
    pub ster0: STER,
    #[doc = "0x164 - Schmitt Trigger Enable Register - Set"]
    pub sters0: STERS,
    #[doc = "0x168 - Schmitt Trigger Enable Register - Clear"]
    pub sterc0: STERC,
    #[doc = "0x16c - Schmitt Trigger Enable Register - Toggle"]
    pub stert0: STERT,
    _reserved6: [u8; 16usize],
    #[doc = "0x180 - Event Enable Register"]
    pub ever0: EVER,
    #[doc = "0x184 - Event Enable Register - Set"]
    pub evers0: EVERS,
    #[doc = "0x188 - Event Enable Register - Clear"]
    pub everc0: EVERC,
    #[doc = "0x18c - Event Enable Register - Toggle"]
    pub evert0: EVERT,
    _reserved7: [u8; 16usize],
    #[doc = "0x1a0 - Lock Register"]
    pub lock0: LOCK,
    #[doc = "0x1a4 - Lock Register - Set"]
    pub locks0: LOCKS,
    #[doc = "0x1a8 - Lock Register - Clear"]
    pub lockc0: LOCKC,
    #[doc = "0x1ac - Lock Register - Toggle"]
    pub lockt0: LOCKT,
    _reserved8: [u8; 48usize],
    #[doc = "0x1e0 - Unlock Register"]
    pub unlock0: UNLOCK,
    #[doc = "0x1e4 - Access Status Register"]
    pub asr0: ASR,
    _reserved9: [u8; 16usize],
    #[doc = "0x1f8 - Parameter Register"]
    pub parameter0: PARAMETER,
    #[doc = "0x1fc - Version Register"]
    pub version0: VERSION,
    #[doc = "0x200 - GPIO Enable Register"]
    pub gper1: GPER,
    #[doc = "0x204 - GPIO Enable Register - Set"]
    pub gpers1: GPERS,
    #[doc = "0x208 - GPIO Enable Register - Clear"]
    pub gperc1: GPERC,
    #[doc = "0x20c - GPIO Enable Register - Toggle"]
    pub gpert1: GPERT,
    #[doc = "0x210 - Peripheral Mux Register 0"]
    pub pmr01: PMR0,
    #[doc = "0x214 - Peripheral Mux Register 0 - Set"]
    pub pmr0s1: PMR0S,
    #[doc = "0x218 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c1: PMR0C,
    #[doc = "0x21c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t1: PMR0T,
    #[doc = "0x220 - Peripheral Mux Register 1"]
    pub pmr11: PMR1,
    #[doc = "0x224 - Peripheral Mux Register 1 - Set"]
    pub pmr1s1: PMR1S,
    #[doc = "0x228 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c1: PMR1C,
    #[doc = "0x22c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t1: PMR1T,
    #[doc = "0x230 - Peripheral Mux Register 2"]
    pub pmr21: PMR2,
    #[doc = "0x234 - Peripheral Mux Register 2 - Set"]
    pub pmr2s1: PMR2S,
    #[doc = "0x238 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c1: PMR2C,
    #[doc = "0x23c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t1: PMR2T,
    #[doc = "0x240 - Output Driver Enable Register"]
    pub oder1: ODER,
    #[doc = "0x244 - Output Driver Enable Register - Set"]
    pub oders1: ODERS,
    #[doc = "0x248 - Output Driver Enable Register - Clear"]
    pub oderc1: ODERC,
    #[doc = "0x24c - Output Driver Enable Register - Toggle"]
    pub odert1: ODERT,
    #[doc = "0x250 - Output Value Register"]
    pub ovr1: OVR,
    #[doc = "0x254 - Output Value Register - Set"]
    pub ovrs1: OVRS,
    #[doc = "0x258 - Output Value Register - Clear"]
    pub ovrc1: OVRC,
    #[doc = "0x25c - Output Value Register - Toggle"]
    pub ovrt1: OVRT,
    #[doc = "0x260 - Pin Value Register"]
    pub pvr1: PVR,
    _reserved10: [u8; 12usize],
    #[doc = "0x270 - Pull-up Enable Register"]
    pub puer1: PUER,
    #[doc = "0x274 - Pull-up Enable Register - Set"]
    pub puers1: PUERS,
    #[doc = "0x278 - Pull-up Enable Register - Clear"]
    pub puerc1: PUERC,
    #[doc = "0x27c - Pull-up Enable Register - Toggle"]
    pub puert1: PUERT,
    #[doc = "0x280 - Pull-down Enable Register"]
    pub pder1: PDER,
    #[doc = "0x284 - Pull-down Enable Register - Set"]
    pub pders1: PDERS,
    #[doc = "0x288 - Pull-down Enable Register - Clear"]
    pub pderc1: PDERC,
    #[doc = "0x28c - Pull-down Enable Register - Toggle"]
    pub pdert1: PDERT,
    #[doc = "0x290 - Interrupt Enable Register"]
    pub ier1: IER,
    #[doc = "0x294 - Interrupt Enable Register - Set"]
    pub iers1: IERS,
    #[doc = "0x298 - Interrupt Enable Register - Clear"]
    pub ierc1: IERC,
    #[doc = "0x29c - Interrupt Enable Register - Toggle"]
    pub iert1: IERT,
    #[doc = "0x2a0 - Interrupt Mode Register 0"]
    pub imr01: IMR0,
    #[doc = "0x2a4 - Interrupt Mode Register 0 - Set"]
    pub imr0s1: IMR0S,
    #[doc = "0x2a8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c1: IMR0C,
    #[doc = "0x2ac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t1: IMR0T,
    #[doc = "0x2b0 - Interrupt Mode Register 1"]
    pub imr11: IMR1,
    #[doc = "0x2b4 - Interrupt Mode Register 1 - Set"]
    pub imr1s1: IMR1S,
    #[doc = "0x2b8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c1: IMR1C,
    #[doc = "0x2bc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t1: IMR1T,
    #[doc = "0x2c0 - Glitch Filter Enable Register"]
    pub gfer1: GFER,
    #[doc = "0x2c4 - Glitch Filter Enable Register - Set"]
    pub gfers1: GFERS,
    #[doc = "0x2c8 - Glitch Filter Enable Register - Clear"]
    pub gferc1: GFERC,
    #[doc = "0x2cc - Glitch Filter Enable Register - Toggle"]
    pub gfert1: GFERT,
    #[doc = "0x2d0 - Interrupt Flag Register"]
    pub ifr1: IFR,
    _reserved11: [u8; 4usize],
    #[doc = "0x2d8 - Interrupt Flag Register - Clear"]
    pub ifrc1: IFRC,
    _reserved12: [u8; 4usize],
    #[doc = "0x2e0 - Open Drain Mode Register"]
    pub odmer1: ODMER,
    #[doc = "0x2e4 - Open Drain Mode Register - Set"]
    pub odmers1: ODMERS,
    #[doc = "0x2e8 - Open Drain Mode Register - Clear"]
    pub odmerc1: ODMERC,
    #[doc = "0x2ec - Open Drain Mode Register - Toggle"]
    pub odmert1: ODMERT,
    _reserved13: [u8; 16usize],
    #[doc = "0x300 - Output Driving Capability Register 0"]
    pub odcr01: ODCR0,
    #[doc = "0x304 - Output Driving Capability Register 0 - Set"]
    pub odcr0s1: ODCR0S,
    #[doc = "0x308 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c1: ODCR0C,
    #[doc = "0x30c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t1: ODCR0T,
    #[doc = "0x310 - Output Driving Capability Register 1"]
    pub odcr11: ODCR1,
    #[doc = "0x314 - Output Driving Capability Register 1 - Set"]
    pub odcr1s1: ODCR1S,
    #[doc = "0x318 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c1: ODCR1C,
    #[doc = "0x31c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t1: ODCR1T,
    _reserved14: [u8; 16usize],
    #[doc = "0x330 - Output Slew Rate Register 0"]
    pub osrr01: OSRR0,
    #[doc = "0x334 - Output Slew Rate Register 0 - Set"]
    pub osrr0s1: OSRR0S,
    #[doc = "0x338 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c1: OSRR0C,
    #[doc = "0x33c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t1: OSRR0T,
    _reserved15: [u8; 32usize],
    #[doc = "0x360 - Schmitt Trigger Enable Register"]
    pub ster1: STER,
    #[doc = "0x364 - Schmitt Trigger Enable Register - Set"]
    pub sters1: STERS,
    #[doc = "0x368 - Schmitt Trigger Enable Register - Clear"]
    pub sterc1: STERC,
    #[doc = "0x36c - Schmitt Trigger Enable Register - Toggle"]
    pub stert1: STERT,
    _reserved16: [u8; 16usize],
    #[doc = "0x380 - Event Enable Register"]
    pub ever1: EVER,
    #[doc = "0x384 - Event Enable Register - Set"]
    pub evers1: EVERS,
    #[doc = "0x388 - Event Enable Register - Clear"]
    pub everc1: EVERC,
    #[doc = "0x38c - Event Enable Register - Toggle"]
    pub evert1: EVERT,
    _reserved17: [u8; 16usize],
    #[doc = "0x3a0 - Lock Register"]
    pub lock1: LOCK,
    #[doc = "0x3a4 - Lock Register - Set"]
    pub locks1: LOCKS,
    #[doc = "0x3a8 - Lock Register - Clear"]
    pub lockc1: LOCKC,
    #[doc = "0x3ac - Lock Register - Toggle"]
    pub lockt1: LOCKT,
    _reserved18: [u8; 48usize],
    #[doc = "0x3e0 - Unlock Register"]
    pub unlock1: UNLOCK,
    #[doc = "0x3e4 - Access Status Register"]
    pub asr1: ASR,
    _reserved19: [u8; 16usize],
    #[doc = "0x3f8 - Parameter Register"]
    pub parameter1: PARAMETER,
    #[doc = "0x3fc - Version Register"]
    pub version1: VERSION,
    #[doc = "0x400 - GPIO Enable Register"]
    pub gper2: GPER,
    #[doc = "0x404 - GPIO Enable Register - Set"]
    pub gpers2: GPERS,
    #[doc = "0x408 - GPIO Enable Register - Clear"]
    pub gperc2: GPERC,
    #[doc = "0x40c - GPIO Enable Register - Toggle"]
    pub gpert2: GPERT,
    #[doc = "0x410 - Peripheral Mux Register 0"]
    pub pmr02: PMR0,
    #[doc = "0x414 - Peripheral Mux Register 0 - Set"]
    pub pmr0s2: PMR0S,
    #[doc = "0x418 - Peripheral Mux Register 0 - Clear"]
    pub pmr0c2: PMR0C,
    #[doc = "0x41c - Peripheral Mux Register 0 - Toggle"]
    pub pmr0t2: PMR0T,
    #[doc = "0x420 - Peripheral Mux Register 1"]
    pub pmr12: PMR1,
    #[doc = "0x424 - Peripheral Mux Register 1 - Set"]
    pub pmr1s2: PMR1S,
    #[doc = "0x428 - Peripheral Mux Register 1 - Clear"]
    pub pmr1c2: PMR1C,
    #[doc = "0x42c - Peripheral Mux Register 1 - Toggle"]
    pub pmr1t2: PMR1T,
    #[doc = "0x430 - Peripheral Mux Register 2"]
    pub pmr22: PMR2,
    #[doc = "0x434 - Peripheral Mux Register 2 - Set"]
    pub pmr2s2: PMR2S,
    #[doc = "0x438 - Peripheral Mux Register 2 - Clear"]
    pub pmr2c2: PMR2C,
    #[doc = "0x43c - Peripheral Mux Register 2 - Toggle"]
    pub pmr2t2: PMR2T,
    #[doc = "0x440 - Output Driver Enable Register"]
    pub oder2: ODER,
    #[doc = "0x444 - Output Driver Enable Register - Set"]
    pub oders2: ODERS,
    #[doc = "0x448 - Output Driver Enable Register - Clear"]
    pub oderc2: ODERC,
    #[doc = "0x44c - Output Driver Enable Register - Toggle"]
    pub odert2: ODERT,
    #[doc = "0x450 - Output Value Register"]
    pub ovr2: OVR,
    #[doc = "0x454 - Output Value Register - Set"]
    pub ovrs2: OVRS,
    #[doc = "0x458 - Output Value Register - Clear"]
    pub ovrc2: OVRC,
    #[doc = "0x45c - Output Value Register - Toggle"]
    pub ovrt2: OVRT,
    #[doc = "0x460 - Pin Value Register"]
    pub pvr2: PVR,
    _reserved20: [u8; 12usize],
    #[doc = "0x470 - Pull-up Enable Register"]
    pub puer2: PUER,
    #[doc = "0x474 - Pull-up Enable Register - Set"]
    pub puers2: PUERS,
    #[doc = "0x478 - Pull-up Enable Register - Clear"]
    pub puerc2: PUERC,
    #[doc = "0x47c - Pull-up Enable Register - Toggle"]
    pub puert2: PUERT,
    #[doc = "0x480 - Pull-down Enable Register"]
    pub pder2: PDER,
    #[doc = "0x484 - Pull-down Enable Register - Set"]
    pub pders2: PDERS,
    #[doc = "0x488 - Pull-down Enable Register - Clear"]
    pub pderc2: PDERC,
    #[doc = "0x48c - Pull-down Enable Register - Toggle"]
    pub pdert2: PDERT,
    #[doc = "0x490 - Interrupt Enable Register"]
    pub ier2: IER,
    #[doc = "0x494 - Interrupt Enable Register - Set"]
    pub iers2: IERS,
    #[doc = "0x498 - Interrupt Enable Register - Clear"]
    pub ierc2: IERC,
    #[doc = "0x49c - Interrupt Enable Register - Toggle"]
    pub iert2: IERT,
    #[doc = "0x4a0 - Interrupt Mode Register 0"]
    pub imr02: IMR0,
    #[doc = "0x4a4 - Interrupt Mode Register 0 - Set"]
    pub imr0s2: IMR0S,
    #[doc = "0x4a8 - Interrupt Mode Register 0 - Clear"]
    pub imr0c2: IMR0C,
    #[doc = "0x4ac - Interrupt Mode Register 0 - Toggle"]
    pub imr0t2: IMR0T,
    #[doc = "0x4b0 - Interrupt Mode Register 1"]
    pub imr12: IMR1,
    #[doc = "0x4b4 - Interrupt Mode Register 1 - Set"]
    pub imr1s2: IMR1S,
    #[doc = "0x4b8 - Interrupt Mode Register 1 - Clear"]
    pub imr1c2: IMR1C,
    #[doc = "0x4bc - Interrupt Mode Register 1 - Toggle"]
    pub imr1t2: IMR1T,
    #[doc = "0x4c0 - Glitch Filter Enable Register"]
    pub gfer2: GFER,
    #[doc = "0x4c4 - Glitch Filter Enable Register - Set"]
    pub gfers2: GFERS,
    #[doc = "0x4c8 - Glitch Filter Enable Register - Clear"]
    pub gferc2: GFERC,
    #[doc = "0x4cc - Glitch Filter Enable Register - Toggle"]
    pub gfert2: GFERT,
    #[doc = "0x4d0 - Interrupt Flag Register"]
    pub ifr2: IFR,
    _reserved21: [u8; 4usize],
    #[doc = "0x4d8 - Interrupt Flag Register - Clear"]
    pub ifrc2: IFRC,
    _reserved22: [u8; 4usize],
    #[doc = "0x4e0 - Open Drain Mode Register"]
    pub odmer2: ODMER,
    #[doc = "0x4e4 - Open Drain Mode Register - Set"]
    pub odmers2: ODMERS,
    #[doc = "0x4e8 - Open Drain Mode Register - Clear"]
    pub odmerc2: ODMERC,
    #[doc = "0x4ec - Open Drain Mode Register - Toggle"]
    pub odmert2: ODMERT,
    _reserved23: [u8; 16usize],
    #[doc = "0x500 - Output Driving Capability Register 0"]
    pub odcr02: ODCR0,
    #[doc = "0x504 - Output Driving Capability Register 0 - Set"]
    pub odcr0s2: ODCR0S,
    #[doc = "0x508 - Output Driving Capability Register 0 - Clear"]
    pub odcr0c2: ODCR0C,
    #[doc = "0x50c - Output Driving Capability Register 0 - Toggle"]
    pub odcr0t2: ODCR0T,
    #[doc = "0x510 - Output Driving Capability Register 1"]
    pub odcr12: ODCR1,
    #[doc = "0x514 - Output Driving Capability Register 1 - Set"]
    pub odcr1s2: ODCR1S,
    #[doc = "0x518 - Output Driving Capability Register 1 - Clear"]
    pub odcr1c2: ODCR1C,
    #[doc = "0x51c - Output Driving Capability Register 1 - Toggle"]
    pub odcr1t2: ODCR1T,
    _reserved24: [u8; 16usize],
    #[doc = "0x530 - Output Slew Rate Register 0"]
    pub osrr02: OSRR0,
    #[doc = "0x534 - Output Slew Rate Register 0 - Set"]
    pub osrr0s2: OSRR0S,
    #[doc = "0x538 - Output Slew Rate Register 0 - Clear"]
    pub osrr0c2: OSRR0C,
    #[doc = "0x53c - Output Slew Rate Register 0 - Toggle"]
    pub osrr0t2: OSRR0T,
    _reserved25: [u8; 32usize],
    #[doc = "0x560 - Schmitt Trigger Enable Register"]
    pub ster2: STER,
    #[doc = "0x564 - Schmitt Trigger Enable Register - Set"]
    pub sters2: STERS,
    #[doc = "0x568 - Schmitt Trigger Enable Register - Clear"]
    pub sterc2: STERC,
    #[doc = "0x56c - Schmitt Trigger Enable Register - Toggle"]
    pub stert2: STERT,
    _reserved26: [u8; 16usize],
    #[doc = "0x580 - Event Enable Register"]
    pub ever2: EVER,
    #[doc = "0x584 - Event Enable Register - Set"]
    pub evers2: EVERS,
    #[doc = "0x588 - Event Enable Register - Clear"]
    pub everc2: EVERC,
    #[doc = "0x58c - Event Enable Register - Toggle"]
    pub evert2: EVERT,
    _reserved27: [u8; 16usize],
    #[doc = "0x5a0 - Lock Register"]
    pub lock2: LOCK,
    #[doc = "0x5a4 - Lock Register - Set"]
    pub locks2: LOCKS,
    #[doc = "0x5a8 - Lock Register - Clear"]
    pub lockc2: LOCKC,
    #[doc = "0x5ac - Lock Register - Toggle"]
    pub lockt2: LOCKT,
    _reserved28: [u8; 48usize],
    #[doc = "0x5e0 - Unlock Register"]
    pub unlock2: UNLOCK,
    #[doc = "0x5e4 - Access Status Register"]
    pub asr2: ASR,
    _reserved29: [u8; 16usize],
    #[doc = "0x5f8 - Parameter Register"]
    pub parameter2: PARAMETER,
    #[doc = "0x5fc - Version Register"]
    pub version2: VERSION,
}
#[doc = "GPIO Enable Register"]
pub struct GPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register"]
pub mod gper;
#[doc = "GPIO Enable Register - Set"]
pub struct GPERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register - Set"]
pub mod gpers;
#[doc = "GPIO Enable Register - Clear"]
pub struct GPERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register - Clear"]
pub mod gperc;
#[doc = "GPIO Enable Register - Toggle"]
pub struct GPERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register - Toggle"]
pub mod gpert;
#[doc = "Peripheral Mux Register 0"]
pub struct PMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 0"]
pub mod pmr0;
#[doc = "Peripheral Mux Register 0 - Set"]
pub struct PMR0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 0 - Set"]
pub mod pmr0s;
#[doc = "Peripheral Mux Register 0 - Clear"]
pub struct PMR0C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 0 - Clear"]
pub mod pmr0c;
#[doc = "Peripheral Mux Register 0 - Toggle"]
pub struct PMR0T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 0 - Toggle"]
pub mod pmr0t;
#[doc = "Peripheral Mux Register 1"]
pub struct PMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 1"]
pub mod pmr1;
#[doc = "Peripheral Mux Register 1 - Set"]
pub struct PMR1S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 1 - Set"]
pub mod pmr1s;
#[doc = "Peripheral Mux Register 1 - Clear"]
pub struct PMR1C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 1 - Clear"]
pub mod pmr1c;
#[doc = "Peripheral Mux Register 1 - Toggle"]
pub struct PMR1T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 1 - Toggle"]
pub mod pmr1t;
#[doc = "Peripheral Mux Register 2"]
pub struct PMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 2"]
pub mod pmr2;
#[doc = "Peripheral Mux Register 2 - Set"]
pub struct PMR2S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 2 - Set"]
pub mod pmr2s;
#[doc = "Peripheral Mux Register 2 - Clear"]
pub struct PMR2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 2 - Clear"]
pub mod pmr2c;
#[doc = "Peripheral Mux Register 2 - Toggle"]
pub struct PMR2T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Mux Register 2 - Toggle"]
pub mod pmr2t;
#[doc = "Output Driver Enable Register"]
pub struct ODER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driver Enable Register"]
pub mod oder;
#[doc = "Output Driver Enable Register - Set"]
pub struct ODERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driver Enable Register - Set"]
pub mod oders;
#[doc = "Output Driver Enable Register - Clear"]
pub struct ODERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driver Enable Register - Clear"]
pub mod oderc;
#[doc = "Output Driver Enable Register - Toggle"]
pub struct ODERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driver Enable Register - Toggle"]
pub mod odert;
#[doc = "Output Value Register"]
pub struct OVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Value Register"]
pub mod ovr;
#[doc = "Output Value Register - Set"]
pub struct OVRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Value Register - Set"]
pub mod ovrs;
#[doc = "Output Value Register - Clear"]
pub struct OVRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Value Register - Clear"]
pub mod ovrc;
#[doc = "Output Value Register - Toggle"]
pub struct OVRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Value Register - Toggle"]
pub mod ovrt;
#[doc = "Pin Value Register"]
pub struct PVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Value Register"]
pub mod pvr;
#[doc = "Pull-up Enable Register"]
pub struct PUER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "Pull-up Enable Register - Set"]
pub struct PUERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Enable Register - Set"]
pub mod puers;
#[doc = "Pull-up Enable Register - Clear"]
pub struct PUERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Enable Register - Clear"]
pub mod puerc;
#[doc = "Pull-up Enable Register - Toggle"]
pub struct PUERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-up Enable Register - Toggle"]
pub mod puert;
#[doc = "Pull-down Enable Register"]
pub struct PDER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-down Enable Register"]
pub mod pder;
#[doc = "Pull-down Enable Register - Set"]
pub struct PDERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-down Enable Register - Set"]
pub mod pders;
#[doc = "Pull-down Enable Register - Clear"]
pub struct PDERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-down Enable Register - Clear"]
pub mod pderc;
#[doc = "Pull-down Enable Register - Toggle"]
pub struct PDERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull-down Enable Register - Toggle"]
pub mod pdert;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Enable Register - Set"]
pub struct IERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register - Set"]
pub mod iers;
#[doc = "Interrupt Enable Register - Clear"]
pub struct IERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register - Clear"]
pub mod ierc;
#[doc = "Interrupt Enable Register - Toggle"]
pub struct IERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register - Toggle"]
pub mod iert;
#[doc = "Interrupt Mode Register 0"]
pub struct IMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 0"]
pub mod imr0;
#[doc = "Interrupt Mode Register 0 - Set"]
pub struct IMR0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 0 - Set"]
pub mod imr0s;
#[doc = "Interrupt Mode Register 0 - Clear"]
pub struct IMR0C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 0 - Clear"]
pub mod imr0c;
#[doc = "Interrupt Mode Register 0 - Toggle"]
pub struct IMR0T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 0 - Toggle"]
pub mod imr0t;
#[doc = "Interrupt Mode Register 1"]
pub struct IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 1"]
pub mod imr1;
#[doc = "Interrupt Mode Register 1 - Set"]
pub struct IMR1S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 1 - Set"]
pub mod imr1s;
#[doc = "Interrupt Mode Register 1 - Clear"]
pub struct IMR1C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 1 - Clear"]
pub mod imr1c;
#[doc = "Interrupt Mode Register 1 - Toggle"]
pub struct IMR1T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mode Register 1 - Toggle"]
pub mod imr1t;
#[doc = "Glitch Filter Enable Register"]
pub struct GFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Filter Enable Register"]
pub mod gfer;
#[doc = "Glitch Filter Enable Register - Set"]
pub struct GFERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Filter Enable Register - Set"]
pub mod gfers;
#[doc = "Glitch Filter Enable Register - Clear"]
pub struct GFERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Filter Enable Register - Clear"]
pub mod gferc;
#[doc = "Glitch Filter Enable Register - Toggle"]
pub struct GFERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Filter Enable Register - Toggle"]
pub mod gfert;
#[doc = "Interrupt Flag Register"]
pub struct IFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod ifr;
#[doc = "Interrupt Flag Register - Clear"]
pub struct IFRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register - Clear"]
pub mod ifrc;
#[doc = "Open Drain Mode Register"]
pub struct ODMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Mode Register"]
pub mod odmer;
#[doc = "Open Drain Mode Register - Set"]
pub struct ODMERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Mode Register - Set"]
pub mod odmers;
#[doc = "Open Drain Mode Register - Clear"]
pub struct ODMERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Mode Register - Clear"]
pub mod odmerc;
#[doc = "Open Drain Mode Register - Toggle"]
pub struct ODMERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Mode Register - Toggle"]
pub mod odmert;
#[doc = "Output Driving Capability Register 0"]
pub struct ODCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 0"]
pub mod odcr0;
#[doc = "Output Driving Capability Register 0 - Set"]
pub struct ODCR0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 0 - Set"]
pub mod odcr0s;
#[doc = "Output Driving Capability Register 0 - Clear"]
pub struct ODCR0C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 0 - Clear"]
pub mod odcr0c;
#[doc = "Output Driving Capability Register 0 - Toggle"]
pub struct ODCR0T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 0 - Toggle"]
pub mod odcr0t;
#[doc = "Output Driving Capability Register 1"]
pub struct ODCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 1"]
pub mod odcr1;
#[doc = "Output Driving Capability Register 1 - Set"]
pub struct ODCR1S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 1 - Set"]
pub mod odcr1s;
#[doc = "Output Driving Capability Register 1 - Clear"]
pub struct ODCR1C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 1 - Clear"]
pub mod odcr1c;
#[doc = "Output Driving Capability Register 1 - Toggle"]
pub struct ODCR1T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Driving Capability Register 1 - Toggle"]
pub mod odcr1t;
#[doc = "Output Slew Rate Register 0"]
pub struct OSRR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Slew Rate Register 0"]
pub mod osrr0;
#[doc = "Output Slew Rate Register 0 - Set"]
pub struct OSRR0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Slew Rate Register 0 - Set"]
pub mod osrr0s;
#[doc = "Output Slew Rate Register 0 - Clear"]
pub struct OSRR0C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Slew Rate Register 0 - Clear"]
pub mod osrr0c;
#[doc = "Output Slew Rate Register 0 - Toggle"]
pub struct OSRR0T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Slew Rate Register 0 - Toggle"]
pub mod osrr0t;
#[doc = "Schmitt Trigger Enable Register"]
pub struct STER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schmitt Trigger Enable Register"]
pub mod ster;
#[doc = "Schmitt Trigger Enable Register - Set"]
pub struct STERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schmitt Trigger Enable Register - Set"]
pub mod sters;
#[doc = "Schmitt Trigger Enable Register - Clear"]
pub struct STERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schmitt Trigger Enable Register - Clear"]
pub mod sterc;
#[doc = "Schmitt Trigger Enable Register - Toggle"]
pub struct STERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schmitt Trigger Enable Register - Toggle"]
pub mod stert;
#[doc = "Event Enable Register"]
pub struct EVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable Register"]
pub mod ever;
#[doc = "Event Enable Register - Set"]
pub struct EVERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable Register - Set"]
pub mod evers;
#[doc = "Event Enable Register - Clear"]
pub struct EVERC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable Register - Clear"]
pub mod everc;
#[doc = "Event Enable Register - Toggle"]
pub struct EVERT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable Register - Toggle"]
pub mod evert;
#[doc = "Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Register"]
pub mod lock;
#[doc = "Lock Register - Set"]
pub struct LOCKS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Register - Set"]
pub mod locks;
#[doc = "Lock Register - Clear"]
pub struct LOCKC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Register - Clear"]
pub mod lockc;
#[doc = "Lock Register - Toggle"]
pub struct LOCKT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Register - Toggle"]
pub mod lockt;
#[doc = "Unlock Register"]
pub struct UNLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unlock Register"]
pub mod unlock;
#[doc = "Access Status Register"]
pub struct ASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Status Register"]
pub mod asr;
#[doc = "Parameter Register"]
pub struct PARAMETER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
