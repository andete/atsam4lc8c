// (c) 2018 Joost Yervante Damad <joost@damad.be>

#![no_std]
#![no_main]

// ATSAM4LC8C peripheral
extern crate atsam4lc8c;

// Cortex-M peripheral
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

// semihosting and logging
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
#[macro_use(d_println, println)]
extern crate cortex_m_log;

entry!(main);

fn main() -> ! {
    let atsam = atsam4lc8c::Peripherals::take().unwrap();
    let _cortex = cortex_m::Peripherals::take().unwrap();

    let mut log = cortex_m_log::printer::semihosting::InterruptOk::stdout().unwrap();

    d_println!(log, "hello, mars");

    // enable PC07
    atsam.GPIO.gpers2.write(|w| w.p7().set_bit());
    // set PC07 as output
    atsam.GPIO.oders2.write(|w| w.p7().set_bit());

    // set
    //atsam.GPIO.ovrs2.write(|w| w.p7().set_bit());
    // clear
    //atsam.GPIO.ovrc2.write(|w| w.p7().set_bit());
    loop {
        // toggle
        atsam.GPIO.ovrt2.write(|w| w.p7().set_bit());
        // abuse semihosting to slow down :)
        d_println!(log, "hello, world");
    }
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
