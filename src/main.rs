#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use arduino_uno::hal::i2c::Direction;
use arduino_uno::hal::port::portb;
use arduino_uno::hal::port::mode::{Input, Output, PullUp};
use arduino_uno::I2cMaster;
use nb;
use panic_halt as _;
use ufmt;

const HYT939: u8 = 0x28;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    let mut ddr = pins.ddr;
    let sda = pins.a4.into_pull_up_input(&mut ddr);
    let scl = pins.a5.into_pull_up_input(&mut ddr);

    led.set_high().void_unwrap();

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut ddr),
        57600.into_baudrate(),
    );

    let mut i2c = I2cMaster::new(
        dp.TWI,
        sda,
        scl,
        50000,
    );

    let res= i2c.ping_slave(HYT939, Direction::Write);

    match res {
        Ok(b) => {
            if b {
                blink(&mut led, 5);
                // ufmt::uwriteln!(&mut serial, "Ping successful!\n").void_unwrap();
            } else {
                blink(&mut led, 1);
            };
        },
        Err(_) => blink(&mut led, 3),
    }

    // initialize buffer for measurements
    let mut buffer: &mut [u8; 4] = &mut [0, 0, 0, 0];

    loop {
        // let (humidity_raw, temperature_raw) = measure(&mut i2c, &mut buffer);
        // ufmt::uwriteln!(&mut serial, "humidity raw: {}", humidity_raw).void_unwrap();
        // ufmt::uwriteln!(&mut serial, "temperature raw: {}", temperature_raw).void_unwrap();
        // arduino_uno::delay_ms(3000);

        // Wait for a command to arrive at the serial port
        let b = nb::block!(serial.read()).void_unwrap();

        match b {
            // 109 is an "m"
            109 => {
                let (humidity_raw, temperature_raw) = measure(&mut i2c, &mut buffer);
                ufmt::uwriteln!(&mut serial, "{},{}", humidity_raw, temperature_raw).void_unwrap();
            },
            // 105 is an "i"
            105 => ufmt::uwriteln!(&mut serial, "htlogger").void_unwrap(),
            _   => ufmt::uwriteln!(&mut serial, "command {} is unknown", b).void_unwrap(),
        }

    }
}

fn convert_humidity_raw(x: &u16) -> f32 {
    // let x: u16 = 6777;
    *x as f32 / 16838.0 * 100.0
}

fn measure(i2c: &mut I2cMaster<Input<PullUp>>, buffer: &mut [u8; 4]) -> (u16, u16) {
    // Ping the port to initialize measurement (not sure if necessary)
    i2c.write(HYT939, &[0u8]).unwrap();

    // Wait for measurement (not sure if necessary)
    arduino_uno::delay_ms(100);

    // the first two bytes in the buffer will represent the humidity, the last
    // two bytes represent the temperature.
    i2c.read(HYT939, buffer).unwrap();

    // HUMIDITY 

    // reinterpret the first two bytes in the buffer as u16. Here bits 14 and 15
    // represent the status:
    //   bit 15: CMode Bit, if 1 - element is in command mode
    //   bit 14: Stale Bit, if 1 - no new value has been created since last
    //           reading
    // We make this mutable since we have to mask the two status bits
    let mut humidity_raw: u16 = ((buffer[0] as u16) << 8) | (buffer[1] as u16);

    // to get the stale bit we have to mask the u16 value with
    // 0b0100_0000_0000_0000 (0x4000)
    let _stale_bit = &humidity_raw & 0x4000;
    // ufmt::uwriteln!(&mut serial, "sb: {}", stale_bit);

    // We can now mask the two status bits with 0b0011_1111_1111_1111 (0x3fff)
    humidity_raw &= 0x3fff;

    // Convert the raw humidity u16 value to relative humidity in %
    // 0x0000 (0d0000) represents 0 %, 0x3ffff (0d16383) represents 100 %
    // TODO: FOR SOME REASON F32 OPERATIONS RESULT IN 0.0 VALUES
    // let humidity = convert_humidity_raw(&humidity_raw);

    // TEMPERATURE

    // the last two bits are junk
    // 0110 0000   0100 1000
    // --byte2--   --byte3--
    //                    ^^junk 
    let mut temperature_raw: u16 = ((buffer[2] as u16) << 6) | ((buffer[3] as u16) >> 2);
    
    // we then have to mask the last to bits
    temperature_raw &= 0x3fff;

    // conversion (from -40 to 100 deg C)

    
    // for now lets just return the raw values and do the calculation on the
    // computer
    (humidity_raw, temperature_raw)
}

fn blink(led: &mut portb::PB5<Output>, n: u16) {
    for _ in 0..n {
        led.set_high().void_unwrap();
        arduino_uno::delay_ms(200);
        led.set_low().void_unwrap();
        arduino_uno::delay_ms(200);
    }
}