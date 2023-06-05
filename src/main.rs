use core::panic;

use rppal::gpio::{ Gpio, OutputPin, InputPin, Level };

const GPIO_COMMUNICATOR_PIN: u8 = 5;
const TIME_OUT_USEC: u16 = 1000;

fn main() {

    let gpio = Gpio::new().unwrap();

    let dht11 = DHT11::new(gpio);

    let mut buf : Vec<u8>= vec![];

    dht11.setup();

    for i in 0..40 {
        println!("{}: {:?}", i + 1, dht11.read_bit());
    }

}

struct DHT11 {
    gpio: Gpio
}

impl DHT11 {
    pub fn new(gpio: Gpio) -> Self {

        DHT11 { gpio }
    }

    /// Set up the connection to the DHT11
    pub fn setup(&self) {

        // send a start signal.
        {
            let output = self.pin_output(); // TODO: handling
            output.is_set_low();
            self.wait_mil_sec(20);
            output.is_set_high();
            self.wait_micro_sec(40);
        }

        self.read_bit();
    }

    fn read_bit(&self) -> bool {
        let low_time = self.wait_pulse(Level::High);
        let high_time = self.wait_pulse(Level::Low);

        high_time > low_time
    }

    /// wait until it changes to the disired voltage.
    fn wait_pulse(&self, level: Level) -> u32 {

        let mut count = 0_u16;
        let input = self.pin_input();

        while input.read() != level {
            count += 1;
            if count > TIME_OUT_USEC {
                panic!("time out"); // handle
            }
            self.wait_micro_sec(1);
        }

        u32::from(count)
    }

    fn pin_output(&self) -> OutputPin {
        let pin = self.gpio.get(GPIO_COMMUNICATOR_PIN).unwrap(); // TODO; handling
        pin.into_output()
    }

    fn pin_input(&self) -> InputPin {
        let pin = self.gpio.get(GPIO_COMMUNICATOR_PIN).unwrap(); // TODO; handling
        pin.into_input()
    }

    #[inline]
    fn wait_mil_sec(&self, duration: u64) {
        std::thread::sleep(std::time::Duration::from_micros(duration))
    }

    #[inline]
    fn wait_micro_sec(&self, duration: u64) {
        std::thread::sleep(std::time::Duration::from_micros(duration))
    }
}
