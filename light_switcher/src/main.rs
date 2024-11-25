#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod timer;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Initialize the TC0 device to count milliseconds
    timer::millis_init(dp.TC0);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop {
       ufmt::uwriteln!(&mut serial, "Millis since start: {:?}\r", timer::millis()).unwrap();
       arduino_hal::delay_ms(1000);
    }
}
