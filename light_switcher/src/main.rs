#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let sound_sensor_input = pins.d7.into_floating_input();

    loop {
        ufmt::uwriteln!(&mut serial, "Clap heard: {:?}\r", sound_sensor_input.is_high()).unwrap();
        arduino_hal::delay_us(1000);
    }
}
