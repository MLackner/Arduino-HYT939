# Arduino Uno with HYT939

This software is supposed to run on an Arduino Uno equipped with a HYT939
humidity/temperature sensor.

## Compiling

Information about the compilation process is available in [The AVR-Rust
Guidebook](https://book.avr-rust.com/001-introduction.html).

Compile via
```
cargo build --release
```
and flath to the device
```
avrdude -patmega328p -carduino -P <your-usb-port> -b115200 -D -Uflash:w:target/avr-atmega328p/release/arduino_hyt939.elf
```

## Resources

* HYT939 application notes: [application notes](https://asset.re-in.de/add/160267/c1/-/en/000505678ML01/AN_IST-AG-Evaluations-Kit-1-St.-LabKit-HYT-Messbereich-0-100-rF.pdf)
* https://creativcoder.dev/rust-on-arduino-uno