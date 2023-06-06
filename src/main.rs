use core::panic;

use rppal::gpio::{ Gpio, IoPin, Level, Mode, PullUpDown };
use rppal::hal::Delay;
use embedded_hal::blocking::delay::{ DelayMs, DelayUs };

const GPIO_COMMUNICATOR_PIN: u8 = 5;
const TIME_OUT_USEC: u16 = 1000;


fn main() {

    let mut dht11 = DHT11::new(GPIO_COMMUNICATOR_PIN);
    let mut delay = Delay::new();

    let mut buf : Vec<u8>= vec![];

    dht11.setup(&mut delay);

    for _ in 0..40 {
        print!("{} ", if dht11.read_bit(&mut delay) { 1 } else { 0 });
    }

}

struct DHT11
{
    pin: IoPin,
}

impl DHT11 {
    pub fn new(pin_number: u8) -> Self {

        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(pin_number).unwrap();
        let mut io_pin = pin.into_io(Mode::Input);
        io_pin.set_pullupdown(PullUpDown::PullUp);

        DHT11 { pin: io_pin }
    }

    /// Set up the connection to the DHT11
    pub fn setup<D>(&mut self, delay: &mut D)
        where D: DelayMs<u16> + DelayUs<u16>
    {

        // send a start signal.
        self.to_output_mode();
        self.set_high_level();
        delay.delay_ms(1);

        self.set_low_level();
        delay.delay_ms(20);

        self.set_high_level();

        delay.delay_us(20);

        self.read_bit(delay);
    }

    fn read_bit<D>(&mut self, delay: &mut D) -> bool
        where D: DelayMs<u16> + DelayUs<u16>
    {
        let low_time = self.wait_pulse(Level::High, delay);
        let high_time = self.wait_pulse(Level::Low, delay);

        high_time > low_time
    }

    /// wait until it changes to the disired voltage.
    fn wait_pulse<D>(&mut self, level: Level, delay: &mut D) -> u32
        where D: DelayMs<u16> + DelayUs<u16>
    {

        if self.pin.mode() != Mode::Input {
            self.to_input_mode();
        }

        let mut count = 0;

        while self.pin.read() != level {
            count += 1;
            if count > TIME_OUT_USEC {
                panic!("time out"); // handle
            }
            delay.delay_us(1);
        }

        u32::from(count)
    }

    /// change a pin to output.
    fn to_output_mode(&mut self) {
        self.pin.set_mode(Mode::Output)
    }

    /// change a pin to output.
    fn to_input_mode(&mut self) {
        self.pin.set_mode(Mode::Input)
    }

    /// set an output pin level to low.
    fn set_low_level(&mut self) {
        if self.pin.mode() != Mode::Output {
            self.to_output_mode();
        }
        self.pin.set_low();
    }

    /// set an output pin level to low.
    fn set_high_level(&mut self) {
        if self.pin.mode() != Mode::Output {
            self.to_output_mode();
        }
        self.pin.set_high();
    }

}
