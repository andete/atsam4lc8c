#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub udcon: UDCON,
    #[doc = "0x04 - Device Global Interupt Register"]
    pub udint: UDINT,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub udintclr: UDINTCLR,
    #[doc = "0x0c - Device Global Interrupt Set Regsiter"]
    pub udintset: UDINTSET,
    #[doc = "0x10 - Device Global Interrupt Enable Register"]
    pub udinte: UDINTE,
    #[doc = "0x14 - Device Global Interrupt Enable Clear Register"]
    pub udinteclr: UDINTECLR,
    #[doc = "0x18 - Device Global Interrupt Enable Set Register"]
    pub udinteset: UDINTESET,
    #[doc = "0x1c - Endpoint Enable/Reset Register"]
    pub uerst: UERST,
    #[doc = "0x20 - Device Frame Number Register"]
    pub udfnum: UDFNUM,
    _reserved0: [u8; 220usize],
    #[doc = "0x100 - Endpoint Configuration Register"]
    pub uecfg0: UECFG0,
    #[doc = "0x104 - Endpoint Configuration Register"]
    pub uecfg1: UECFG1,
    #[doc = "0x108 - Endpoint Configuration Register"]
    pub uecfg2: UECFG2,
    #[doc = "0x10c - Endpoint Configuration Register"]
    pub uecfg3: UECFG3,
    #[doc = "0x110 - Endpoint Configuration Register"]
    pub uecfg4: UECFG4,
    #[doc = "0x114 - Endpoint Configuration Register"]
    pub uecfg5: UECFG5,
    #[doc = "0x118 - Endpoint Configuration Register"]
    pub uecfg6: UECFG6,
    #[doc = "0x11c - Endpoint Configuration Register"]
    pub uecfg7: UECFG7,
    _reserved1: [u8; 16usize],
    #[doc = "0x130 - Endpoint Status Register"]
    pub uesta0: UESTA0,
    #[doc = "0x134 - Endpoint Status Register"]
    pub uesta1: UESTA1,
    #[doc = "0x138 - Endpoint Status Register"]
    pub uesta2: UESTA2,
    #[doc = "0x13c - Endpoint Status Register"]
    pub uesta3: UESTA3,
    #[doc = "0x140 - Endpoint Status Register"]
    pub uesta4: UESTA4,
    #[doc = "0x144 - Endpoint Status Register"]
    pub uesta5: UESTA5,
    #[doc = "0x148 - Endpoint Status Register"]
    pub uesta6: UESTA6,
    #[doc = "0x14c - Endpoint Status Register"]
    pub uesta7: UESTA7,
    _reserved2: [u8; 16usize],
    #[doc = "0x160 - Endpoint Status Clear Register"]
    pub uesta0clr: UESTA0CLR,
    #[doc = "0x164 - Endpoint Status Clear Register"]
    pub uesta1clr: UESTA1CLR,
    #[doc = "0x168 - Endpoint Status Clear Register"]
    pub uesta2clr: UESTA2CLR,
    #[doc = "0x16c - Endpoint Status Clear Register"]
    pub uesta3clr: UESTA3CLR,
    #[doc = "0x170 - Endpoint Status Clear Register"]
    pub uesta4clr: UESTA4CLR,
    #[doc = "0x174 - Endpoint Status Clear Register"]
    pub uesta5clr: UESTA5CLR,
    #[doc = "0x178 - Endpoint Status Clear Register"]
    pub uesta6clr: UESTA6CLR,
    #[doc = "0x17c - Endpoint Status Clear Register"]
    pub uesta7clr: UESTA7CLR,
    _reserved3: [u8; 16usize],
    #[doc = "0x190 - Endpoint Status Set Register"]
    pub uesta0set: UESTA0SET,
    #[doc = "0x194 - Endpoint Status Set Register"]
    pub uesta1set: UESTA1SET,
    #[doc = "0x198 - Endpoint Status Set Register"]
    pub uesta2set: UESTA2SET,
    #[doc = "0x19c - Endpoint Status Set Register"]
    pub uesta3set: UESTA3SET,
    #[doc = "0x1a0 - Endpoint Status Set Register"]
    pub uesta4set: UESTA4SET,
    #[doc = "0x1a4 - Endpoint Status Set Register"]
    pub uesta5set: UESTA5SET,
    #[doc = "0x1a8 - Endpoint Status Set Register"]
    pub uesta6set: UESTA6SET,
    #[doc = "0x1ac - Endpoint Status Set Register"]
    pub uesta7set: UESTA7SET,
    _reserved4: [u8; 16usize],
    #[doc = "0x1c0 - Endpoint Control Register"]
    pub uecon0: UECON0,
    #[doc = "0x1c4 - Endpoint Control Register"]
    pub uecon1: UECON1,
    #[doc = "0x1c8 - Endpoint Control Register"]
    pub uecon2: UECON2,
    #[doc = "0x1cc - Endpoint Control Register"]
    pub uecon3: UECON3,
    #[doc = "0x1d0 - Endpoint Control Register"]
    pub uecon4: UECON4,
    #[doc = "0x1d4 - Endpoint Control Register"]
    pub uecon5: UECON5,
    #[doc = "0x1d8 - Endpoint Control Register"]
    pub uecon6: UECON6,
    #[doc = "0x1dc - Endpoint Control Register"]
    pub uecon7: UECON7,
    _reserved5: [u8; 16usize],
    #[doc = "0x1f0 - Endpoint Control Set Register"]
    pub uecon0set: UECON0SET,
    #[doc = "0x1f4 - Endpoint Control Set Register"]
    pub uecon1set: UECON1SET,
    #[doc = "0x1f8 - Endpoint Control Set Register"]
    pub uecon2set: UECON2SET,
    #[doc = "0x1fc - Endpoint Control Set Register"]
    pub uecon3set: UECON3SET,
    #[doc = "0x200 - Endpoint Control Set Register"]
    pub uecon4set: UECON4SET,
    #[doc = "0x204 - Endpoint Control Set Register"]
    pub uecon5set: UECON5SET,
    #[doc = "0x208 - Endpoint Control Set Register"]
    pub uecon6set: UECON6SET,
    #[doc = "0x20c - Endpoint Control Set Register"]
    pub uecon7set: UECON7SET,
    _reserved6: [u8; 16usize],
    #[doc = "0x220 - Endpoint Control Clear Register"]
    pub uecon0clr: UECON0CLR,
    #[doc = "0x224 - TXINE Clear"]
    pub uecon1clr: UECON1CLR,
    #[doc = "0x228 - TXINE Clear"]
    pub uecon2clr: UECON2CLR,
    #[doc = "0x22c - TXINE Clear"]
    pub uecon3clr: UECON3CLR,
    #[doc = "0x230 - TXINE Clear"]
    pub uecon4clr: UECON4CLR,
    #[doc = "0x234 - TXINE Clear"]
    pub uecon5clr: UECON5CLR,
    #[doc = "0x238 - TXINE Clear"]
    pub uecon6clr: UECON6CLR,
    #[doc = "0x23c - TXINE Clear"]
    pub uecon7clr: UECON7CLR,
    _reserved7: [u8; 448usize],
    #[doc = "0x400 - Host General Control Register"]
    pub uhcon: UHCON,
    #[doc = "0x404 - Host Global Interrupt Register"]
    pub uhint: UHINT,
    #[doc = "0x408 - Host Global Interrrupt Clear Register"]
    pub uhintclr: UHINTCLR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub uhintset: UHINTSET,
    #[doc = "0x410 - Host Global Interrupt Enable Register"]
    pub uhinte: UHINTE,
    #[doc = "0x414 - Host Global Interrupt Enable Clear Register"]
    pub uhinteclr: UHINTECLR,
    #[doc = "0x418 - Host Global Interrupt Enable Set Register"]
    pub uhinteset: UHINTESET,
    #[doc = "0x41c - Pipe Reset Register"]
    pub uprst: UPRST,
    #[doc = "0x420 - Host Frame Number Register"]
    pub uhfnum: UHFNUM,
    #[doc = "0x424 - Host Start of Frame Control Register"]
    pub uhsofc: UHSOFC,
    _reserved8: [u8; 216usize],
    #[doc = "0x500 - Pipe Configuration Register"]
    pub upcfg0: UPCFG0,
    #[doc = "0x504 - Pipe Configuration Register"]
    pub upcfg1: UPCFG1,
    #[doc = "0x508 - Pipe Configuration Register"]
    pub upcfg2: UPCFG2,
    #[doc = "0x50c - Pipe Configuration Register"]
    pub upcfg3: UPCFG3,
    #[doc = "0x510 - Pipe Configuration Register"]
    pub upcfg4: UPCFG4,
    #[doc = "0x514 - Pipe Configuration Register"]
    pub upcfg5: UPCFG5,
    #[doc = "0x518 - Pipe Configuration Register"]
    pub upcfg6: UPCFG6,
    #[doc = "0x51c - Pipe Configuration Register"]
    pub upcfg7: UPCFG7,
    _reserved9: [u8; 16usize],
    #[doc = "0x530 - Pipe Status Register"]
    pub upsta0: UPSTA0,
    #[doc = "0x534 - Pipe Status Register"]
    pub upsta1: UPSTA1,
    #[doc = "0x538 - Pipe Status Register"]
    pub upsta2: UPSTA2,
    #[doc = "0x53c - Pipe Status Register"]
    pub upsta3: UPSTA3,
    #[doc = "0x540 - Pipe Status Register"]
    pub upsta4: UPSTA4,
    #[doc = "0x544 - Pipe Status Register"]
    pub upsta5: UPSTA5,
    #[doc = "0x548 - Pipe Status Register"]
    pub upsta6: UPSTA6,
    #[doc = "0x54c - Pipe Status Register"]
    pub upsta7: UPSTA7,
    _reserved10: [u8; 16usize],
    #[doc = "0x560 - Pipe Status Clear Register"]
    pub upsta0clr: UPSTA0CLR,
    #[doc = "0x564 - Pipe Status Clear Register"]
    pub upsta1clr: UPSTA1CLR,
    #[doc = "0x568 - Pipe Status Clear Register"]
    pub upsta2clr: UPSTA2CLR,
    #[doc = "0x56c - Pipe Status Clear Register"]
    pub upsta3clr: UPSTA3CLR,
    #[doc = "0x570 - Pipe Status Clear Register"]
    pub upsta4clr: UPSTA4CLR,
    #[doc = "0x574 - Pipe Status Clear Register"]
    pub upsta5clr: UPSTA5CLR,
    #[doc = "0x578 - Pipe Status Clear Register"]
    pub upsta6clr: UPSTA6CLR,
    #[doc = "0x57c - Pipe Status Clear Register"]
    pub upsta7clr: UPSTA7CLR,
    _reserved11: [u8; 16usize],
    #[doc = "0x590 - Pipe Status Set Register"]
    pub upsta0set: UPSTA0SET,
    #[doc = "0x594 - Pipe Status Set Register"]
    pub upsta1set: UPSTA1SET,
    #[doc = "0x598 - Pipe Status Set Register"]
    pub upsta2set: UPSTA2SET,
    #[doc = "0x59c - Pipe Status Set Register"]
    pub upsta3set: UPSTA3SET,
    #[doc = "0x5a0 - Pipe Status Set Register"]
    pub upsta4set: UPSTA4SET,
    #[doc = "0x5a4 - Pipe Status Set Register"]
    pub upsta5set: UPSTA5SET,
    #[doc = "0x5a8 - Pipe Status Set Register"]
    pub upsta6set: UPSTA6SET,
    #[doc = "0x5ac - Pipe Status Set Register"]
    pub upsta7set: UPSTA7SET,
    _reserved12: [u8; 16usize],
    #[doc = "0x5c0 - Pipe Control Register"]
    pub upcon0: UPCON0,
    #[doc = "0x5c4 - Pipe Control Register"]
    pub upcon1: UPCON1,
    #[doc = "0x5c8 - Pipe Control Register"]
    pub upcon2: UPCON2,
    #[doc = "0x5cc - Pipe Control Register"]
    pub upcon3: UPCON3,
    #[doc = "0x5d0 - Pipe Control Register"]
    pub upcon4: UPCON4,
    #[doc = "0x5d4 - Pipe Control Register"]
    pub upcon5: UPCON5,
    #[doc = "0x5d8 - Pipe Control Register"]
    pub upcon6: UPCON6,
    #[doc = "0x5dc - Pipe Control Register"]
    pub upcon7: UPCON7,
    _reserved13: [u8; 16usize],
    #[doc = "0x5f0 - Pipe Control Set Register"]
    pub upcon0set: UPCON0SET,
    #[doc = "0x5f4 - Pipe Control Set Register"]
    pub upcon1set: UPCON1SET,
    #[doc = "0x5f8 - Pipe Control Set Register"]
    pub upcon2set: UPCON2SET,
    #[doc = "0x5fc - Pipe Control Set Register"]
    pub upcon3set: UPCON3SET,
    #[doc = "0x600 - Pipe Control Set Register"]
    pub upcon4set: UPCON4SET,
    #[doc = "0x604 - Pipe Control Set Register"]
    pub upcon5set: UPCON5SET,
    #[doc = "0x608 - Pipe Control Set Register"]
    pub upcon6set: UPCON6SET,
    #[doc = "0x60c - Pipe Control Set Register"]
    pub upcon7set: UPCON7SET,
    _reserved14: [u8; 16usize],
    #[doc = "0x620 - Pipe Control Clear Register"]
    pub upcon0clr: UPCON0CLR,
    #[doc = "0x624 - Pipe Control Clear Register"]
    pub upcon1clr: UPCON1CLR,
    #[doc = "0x628 - Pipe Control Clear Register"]
    pub upcon2clr: UPCON2CLR,
    #[doc = "0x62c - Pipe Control Clear Register"]
    pub upcon3clr: UPCON3CLR,
    #[doc = "0x630 - Pipe Control Clear Register"]
    pub upcon4clr: UPCON4CLR,
    #[doc = "0x634 - Pipe Control Clear Register"]
    pub upcon5clr: UPCON5CLR,
    #[doc = "0x638 - Pipe Control Clear Register"]
    pub upcon6clr: UPCON6CLR,
    #[doc = "0x63c - Pipe Control Clear Register"]
    pub upcon7clr: UPCON7CLR,
    _reserved15: [u8; 16usize],
    #[doc = "0x650 - Pipe In Request"]
    pub upinrq0: UPINRQ0,
    #[doc = "0x654 - Pipe In Request"]
    pub upinrq1: UPINRQ1,
    #[doc = "0x658 - Pipe In Request"]
    pub upinrq2: UPINRQ2,
    #[doc = "0x65c - Pipe In Request"]
    pub upinrq3: UPINRQ3,
    #[doc = "0x660 - Pipe In Request"]
    pub upinrq4: UPINRQ4,
    #[doc = "0x664 - Pipe In Request"]
    pub upinrq5: UPINRQ5,
    #[doc = "0x668 - Pipe In Request"]
    pub upinrq6: UPINRQ6,
    #[doc = "0x66c - Pipe In Request"]
    pub upinrq7: UPINRQ7,
    _reserved16: [u8; 400usize],
    #[doc = "0x800 - General Control Register"]
    pub usbcon: USBCON,
    #[doc = "0x804 - General Status Register"]
    pub usbsta: USBSTA,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbstaclr: USBSTACLR,
    #[doc = "0x80c - General Status Set Register"]
    pub usbstaset: USBSTASET,
    _reserved17: [u8; 8usize],
    #[doc = "0x818 - IP Version Register"]
    pub uvers: UVERS,
    #[doc = "0x81c - IP Features Register"]
    pub ufeatures: UFEATURES,
    #[doc = "0x820 - IP PB address size Register"]
    pub uaddrsize: UADDRSIZE,
    #[doc = "0x824 - IP Name Part One: HUSB"]
    pub uname1: UNAME1,
    #[doc = "0x828 - IP Name Part Two: HOST"]
    pub uname2: UNAME2,
    #[doc = "0x82c - USB internal finite state machine"]
    pub usbfsm: USBFSM,
    #[doc = "0x830 - Endpoint descriptor table"]
    pub udesc: UDESC,
}
#[doc = "Device General Control Register"]
pub struct UDCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device General Control Register"]
pub mod udcon;
#[doc = "Device Global Interupt Register"]
pub struct UDINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interupt Register"]
pub mod udint;
#[doc = "Device Global Interrupt Clear Register"]
pub struct UDINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Clear Register"]
pub mod udintclr;
#[doc = "Device Global Interrupt Set Regsiter"]
pub struct UDINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Set Regsiter"]
pub mod udintset;
#[doc = "Device Global Interrupt Enable Register"]
pub struct UDINTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Enable Register"]
pub mod udinte;
#[doc = "Device Global Interrupt Enable Clear Register"]
pub struct UDINTECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Enable Clear Register"]
pub mod udinteclr;
#[doc = "Device Global Interrupt Enable Set Register"]
pub struct UDINTESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Enable Set Register"]
pub mod udinteset;
#[doc = "Endpoint Enable/Reset Register"]
pub struct UERST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Enable/Reset Register"]
pub mod uerst;
#[doc = "Device Frame Number Register"]
pub struct UDFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Frame Number Register"]
pub mod udfnum;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg0;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg1;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg2;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg3;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg4;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg5;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg6;
#[doc = "Endpoint Configuration Register"]
pub struct UECFG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Configuration Register"]
pub mod uecfg7;
#[doc = "Endpoint Status Register"]
pub struct UESTA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta0;
#[doc = "Endpoint Status Register"]
pub struct UESTA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta1;
#[doc = "Endpoint Status Register"]
pub struct UESTA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta2;
#[doc = "Endpoint Status Register"]
pub struct UESTA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta3;
#[doc = "Endpoint Status Register"]
pub struct UESTA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta4;
#[doc = "Endpoint Status Register"]
pub struct UESTA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta5;
#[doc = "Endpoint Status Register"]
pub struct UESTA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta6;
#[doc = "Endpoint Status Register"]
pub struct UESTA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod uesta7;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta0clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta1clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta2clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta3clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta4clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta5clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta6clr;
#[doc = "Endpoint Status Clear Register"]
pub struct UESTA7CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Clear Register"]
pub mod uesta7clr;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta0set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta1set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta2set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta3set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta4set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta5set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta6set;
#[doc = "Endpoint Status Set Register"]
pub struct UESTA7SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Set Register"]
pub mod uesta7set;
#[doc = "Endpoint Control Register"]
pub struct UECON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon0;
#[doc = "Endpoint Control Register"]
pub struct UECON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon1;
#[doc = "Endpoint Control Register"]
pub struct UECON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon2;
#[doc = "Endpoint Control Register"]
pub struct UECON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon3;
#[doc = "Endpoint Control Register"]
pub struct UECON4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon4;
#[doc = "Endpoint Control Register"]
pub struct UECON5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon5;
#[doc = "Endpoint Control Register"]
pub struct UECON6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon6;
#[doc = "Endpoint Control Register"]
pub struct UECON7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register"]
pub mod uecon7;
#[doc = "Endpoint Control Set Register"]
pub struct UECON0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon0set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon1set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon2set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon3set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon4set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon5set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon6set;
#[doc = "Endpoint Control Set Register"]
pub struct UECON7SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Set Register"]
pub mod uecon7set;
#[doc = "Endpoint Control Clear Register"]
pub struct UECON0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Clear Register"]
pub mod uecon0clr;
#[doc = "TXINE Clear"]
pub struct UECON1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon1clr;
#[doc = "TXINE Clear"]
pub struct UECON2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon2clr;
#[doc = "TXINE Clear"]
pub struct UECON3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon3clr;
#[doc = "TXINE Clear"]
pub struct UECON4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon4clr;
#[doc = "TXINE Clear"]
pub struct UECON5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon5clr;
#[doc = "TXINE Clear"]
pub struct UECON6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon6clr;
#[doc = "TXINE Clear"]
pub struct UECON7CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXINE Clear"]
pub mod uecon7clr;
#[doc = "Host General Control Register"]
pub struct UHCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host General Control Register"]
pub mod uhcon;
#[doc = "Host Global Interrupt Register"]
pub struct UHINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Register"]
pub mod uhint;
#[doc = "Host Global Interrrupt Clear Register"]
pub struct UHINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrrupt Clear Register"]
pub mod uhintclr;
#[doc = "Host Global Interrupt Set Register"]
pub struct UHINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Set Register"]
pub mod uhintset;
#[doc = "Host Global Interrupt Enable Register"]
pub struct UHINTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Enable Register"]
pub mod uhinte;
#[doc = "Host Global Interrupt Enable Clear Register"]
pub struct UHINTECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Enable Clear Register"]
pub mod uhinteclr;
#[doc = "Host Global Interrupt Enable Set Register"]
pub struct UHINTESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Enable Set Register"]
pub mod uhinteset;
#[doc = "Pipe Reset Register"]
pub struct UPRST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Reset Register"]
pub mod uprst;
#[doc = "Host Frame Number Register"]
pub struct UHFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Frame Number Register"]
pub mod uhfnum;
#[doc = "Host Start of Frame Control Register"]
pub struct UHSOFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Start of Frame Control Register"]
pub mod uhsofc;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg0;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg1;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg2;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg3;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg4;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg5;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg6;
#[doc = "Pipe Configuration Register"]
pub struct UPCFG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Configuration Register"]
pub mod upcfg7;
#[doc = "Pipe Status Register"]
pub struct UPSTA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta0;
#[doc = "Pipe Status Register"]
pub struct UPSTA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta1;
#[doc = "Pipe Status Register"]
pub struct UPSTA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta2;
#[doc = "Pipe Status Register"]
pub struct UPSTA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta3;
#[doc = "Pipe Status Register"]
pub struct UPSTA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta4;
#[doc = "Pipe Status Register"]
pub struct UPSTA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta5;
#[doc = "Pipe Status Register"]
pub struct UPSTA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta6;
#[doc = "Pipe Status Register"]
pub struct UPSTA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Register"]
pub mod upsta7;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta0clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta1clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta2clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta3clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta4clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta5clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta6clr;
#[doc = "Pipe Status Clear Register"]
pub struct UPSTA7CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Clear Register"]
pub mod upsta7clr;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta0set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta1set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta2set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta3set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta4set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta5set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta6set;
#[doc = "Pipe Status Set Register"]
pub struct UPSTA7SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Status Set Register"]
pub mod upsta7set;
#[doc = "Pipe Control Register"]
pub struct UPCON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon0;
#[doc = "Pipe Control Register"]
pub struct UPCON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon1;
#[doc = "Pipe Control Register"]
pub struct UPCON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon2;
#[doc = "Pipe Control Register"]
pub struct UPCON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon3;
#[doc = "Pipe Control Register"]
pub struct UPCON4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon4;
#[doc = "Pipe Control Register"]
pub struct UPCON5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon5;
#[doc = "Pipe Control Register"]
pub struct UPCON6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon6;
#[doc = "Pipe Control Register"]
pub struct UPCON7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Register"]
pub mod upcon7;
#[doc = "Pipe Control Set Register"]
pub struct UPCON0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon0set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon1set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon2set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon3set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon4set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon5set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon6set;
#[doc = "Pipe Control Set Register"]
pub struct UPCON7SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Set Register"]
pub mod upcon7set;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon0clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon1clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon2clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon3clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon4clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon5clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon6clr;
#[doc = "Pipe Control Clear Register"]
pub struct UPCON7CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe Control Clear Register"]
pub mod upcon7clr;
#[doc = "Pipe In Request"]
pub struct UPINRQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq0;
#[doc = "Pipe In Request"]
pub struct UPINRQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq1;
#[doc = "Pipe In Request"]
pub struct UPINRQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq2;
#[doc = "Pipe In Request"]
pub struct UPINRQ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq3;
#[doc = "Pipe In Request"]
pub struct UPINRQ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq4;
#[doc = "Pipe In Request"]
pub struct UPINRQ5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq5;
#[doc = "Pipe In Request"]
pub struct UPINRQ6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq6;
#[doc = "Pipe In Request"]
pub struct UPINRQ7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pipe In Request"]
pub mod upinrq7;
#[doc = "General Control Register"]
pub struct USBCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Control Register"]
pub mod usbcon;
#[doc = "General Status Register"]
pub struct USBSTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Register"]
pub mod usbsta;
#[doc = "General Status Clear Register"]
pub struct USBSTACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Clear Register"]
pub mod usbstaclr;
#[doc = "General Status Set Register"]
pub struct USBSTASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Set Register"]
pub mod usbstaset;
#[doc = "IP Version Register"]
pub struct UVERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Version Register"]
pub mod uvers;
#[doc = "IP Features Register"]
pub struct UFEATURES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Features Register"]
pub mod ufeatures;
#[doc = "IP PB address size Register"]
pub struct UADDRSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP PB address size Register"]
pub mod uaddrsize;
#[doc = "IP Name Part One: HUSB"]
pub struct UNAME1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Name Part One: HUSB"]
pub mod uname1;
#[doc = "IP Name Part Two: HOST"]
pub struct UNAME2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Name Part Two: HOST"]
pub mod uname2;
#[doc = "USB internal finite state machine"]
pub struct USBFSM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB internal finite state machine"]
pub mod usbfsm;
#[doc = "Endpoint descriptor table"]
pub struct UDESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint descriptor table"]
pub mod udesc;
