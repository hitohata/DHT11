use core::panic;

use rppal::gpio::{ Gpio, IoPin, Level, Mode };

const GPIO_COMMUNICATOR_PIN: u8 = 5;
const TIME_OUT_USEC: u16 = 1000;

fn main() {

    let mut dht11 = DHT11::new(GPIO_COMMUNICATOR_PIN);

    let mut buf : Vec<u8>= vec![];

    dht11.setup();

    for i in 0..40 {
        println!("{}: {:?}", i + 1, dht11.read_bit());
    }

}

struct DHT11
{
    pin: IoPin
}

impl DHT11 {
    pub fn new(pin_number: u8) -> Self {

        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(pin_number).unwrap();

        DHT11 { pin: pin.into_io(Mode::Input) }
    }

    /// Set up the connection to the DHT11
    pub fn setup(&mut self) {

        // send a start signal.
        self.to_output_mode();
        self.set_low_level();
        self.wait_mil_sec(20);

        self.to_input_mode();
        self.wait_micro_sec(40);
        self.read_bit();
    }

    fn read_bit(&mut self) -> bool {
        let low_time = self.wait_pulse(Level::High);
        let high_time = self.wait_pulse(Level::Low);

        println!("low: {}, high: {}", low_time, high_time);

        high_time > low_time
    }

    /// wait until it changes to the disired voltage.
    fn wait_pulse(&mut self, level: Level) -> u32 {

        if self.pin.mode() != Mode::Input {
            self.to_input_mode();
        }

        let mut count = 0_u16;

        println!("{:?}", self.pin.read());

        while self.pin.read() != level {
            count += 1;
            if count > TIME_OUT_USEC {
                panic!("time out"); // handle
            }
            self.wait_micro_sec(1);
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
        self.set_low_level();
    }

    #[inline]
    fn wait_mil_sec(&self, duration: u64) {
        std::thread::sleep(std::time::Duration::from_millis(duration))
    }

    #[inline]
    fn wait_micro_sec(&self, duration: u64) {
        std::thread::sleep(std::time::Duration::from_micros(duration))
    }
}
