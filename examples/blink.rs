// (c) 2018 Joost Yervante Damad <joost@damad.be>

#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

extern crate atsam4lc8c;
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_semihosting;

use core::intrinsics;

// print hello world via cortex-M semihosting
fn print() {
    // File descriptor (on the host)
    const STDOUT: usize = 1; // NOTE the host stdout may not always be fd 1
    static MSG: &'static [u8] = b"Hello, world!\n";

    // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
    let r = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };
}

entry!(main);

fn main() -> ! {
    
    let mut atsam = atsam4lc8c::Peripherals::take().unwrap();
    let mut cortex = cortex_m::Peripherals::take().unwrap();

    // on the openocd debug console
    print();

    // enable PC07
    atsam.GPIO.gpers2.write(|w| w.p7().set_bit());
    atsam.GPIO.oders2.write(|w| w.p7().set_bit());
    

    //atsam.GPIO.ovrs2.write(|w| w.p7().set_bit());
    //atsam.GPIO.ovrc2.write(|w| w.p7().set_bit());
    loop {
        // toggle
        atsam.GPIO.ovrt2.write(|w| w.p7().set_bit());
        // abuse semihosting to slow down :)
        print();
    }
    
}

// TODO -> panic to semihosting

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32,
                               _column: u32) -> ! {
    unsafe { intrinsics::abort() }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
