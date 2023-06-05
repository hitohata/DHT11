use core::panic;

use rppal::gpio::{ Gpio, OutputPin, InputPin };

const GPIO_COMMUNICATOR_PIN: u8 = 5;
const TIME_OUT_USEC: usize = 100;

fn main() {

    let gpio = Gpio::new().unwrap();

    let dht11 = DHT11::new(gpio);

    let mut buf : Vec<u8>= vec![];

    dht11.setup();

    // dht11.is_response_one();

    // for i in 0..40 {
    //     if dht11.is_response_one() {
    //         buf.push(1);
    //     } else {
    //         buf.push(0)
    //     }
    //     println!("{:?}", buf);
    // }

    // println!("{:?}", buf);

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
        let output = self.pin_output(); // TODO: handling
        output.is_set_low();
        self.wait_mil_sec(20);

        // wait a setting up
        self.pin_input();
        self.wait_micro_sec(40);
    }

    fn is_response_one(&self) -> bool {

        let pin = self.gpio.get(GPIO_COMMUNICATOR_PIN).unwrap();
        let input = pin.into_input();
        let mut low_count: usize = 0; 
        let mut high_count: usize = 0;

        loop {
            if low_count > TIME_OUT_USEC {
                panic!("The low voltage state exceeds 100 micro sec.");
            }

            if input.is_high() {
                break;
            } else {
                low_count += 1;
            }
            std::thread::sleep(std::time::Duration::from_micros(1));
        }


        loop {
            if high_count > TIME_OUT_USEC {
                panic!("The high voltage state exceeds 100 micro sec.");
            }

            if input.is_low() {
                break;
            } else {
                high_count += 1;
            }
            std::thread::sleep(std::time::Duration::from_micros(1));
        }

        if high_count > low_count {
            true
        } else {
            false
        }

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
