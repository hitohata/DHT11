use std::rc::Rc;

use rppal::gpio::{ Gpio, OutputPin, InputPin };

const GPIO_COMMUNICATOR_PIN: u8 = 5;

fn main() {

    let gpio = Gpio::new().unwrap();
    let mut buf : Vec<u8>= vec![];


    {
        let communication_pin = gpio.get(GPIO_COMMUNICATOR_PIN).unwrap();

        let output = communication_pin.into_output();

        output.is_set_low();

        std::thread::sleep(std::time::Duration::from_millis(18));

        output.is_set_high();
    }

    {

        let communication_pin = gpio.get(GPIO_COMMUNICATOR_PIN).unwrap();
        let input = communication_pin.into_input();

        loop {
            if input.is_low() {
                break;
            }
        }

        loop {
            if input.is_high() {
                break;
            }
        }

        let mut count = 0;

        while count < 22 {
            
            loop {
                if input.is_low() {
                    break;
                }
            }

            loop {
                if input.is_high() {
                    break;
                }
            }

            let mut high_time: u8 = 0;

            loop {
                if input.is_high() {
                    high_time += 1;
                    std::thread::sleep(std::time::Duration::from_micros(1));
                } else {
                    break;
                }
            }

            buf.push(high_time);
            
            // if input.is_high() {
            //     buf.push(1);
            // } else {
            //     buf.push(0);
            // };

            count += 1;
        }

    }

    println!("{:?}", buf);

}
