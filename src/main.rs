mod dht11_error;

use std::fmt;
use rppal::gpio::{ Gpio, IoPin, Level, Mode, PullUpDown };
use rppal::hal::Delay;
use embedded_hal::blocking::delay::{ DelayMs, DelayUs };
use dht11_error::DHT11Error;

const GPIO_COMMUNICATOR_PIN: u8 = 5;
const TIME_OUT_USEC: u16 = 1000;
const RETRY: u8 = 3;


fn main() {

    let mut dht11 = DHT11::new(GPIO_COMMUNICATOR_PIN).unwrap();
    let mut delay = Delay::new();

    let result = dht11.read(&mut delay);


    println!("{:?}", result);

}

struct DHT11
{
    pin: IoPin,
}

#[derive(Debug)]
struct MeasurementResult {
    pub temperature: f32,
    pub humidity: f32
}

impl fmt::Display for MeasurementResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Temperature: {}\nHumidity: {}", self.temperature, self.humidity)
    }
}

impl DHT11 {
    pub fn new(pin_number: u8) -> Result<Self, DHT11Error> {

        let gpio = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(_) => return Err(DHT11Error::DHT11InitializationError("GPIO initialization failed.".to_string()))
        };

        let pin = match gpio.get(pin_number) {
            Ok(pin) => pin,
            Err(_) => return Err(DHT11Error::DHT11InitializationError(format!("Pin, {}, initialization failed", pin_number)))
        };

        let mut io_pin = pin.into_io(Mode::Input);
        io_pin.set_pullupdown(PullUpDown::PullUp);

        Ok(DHT11 { pin: io_pin })
    }

    pub fn read<D>(&mut self, delay: &mut D) -> Result<MeasurementResult, DHT11Error>
        where D: DelayMs<u16> + DelayUs<u16>
    {

        let mut i = 1;

        let measurement_result: MeasurementResult = loop {

            match self.setup(delay) {
                Ok(_) => (),
                Err(err) => {
                    if i > RETRY {
                        return Err(err)
                    } else {
                        i += 1;
                        continue;
                    }
                }
            } 

            match self.read_data(delay) {
                Err(err) => {
                    if i > RETRY {
                        return Err(err)
                    } else {
                        i += 1;
                        continue;
                    }
                },
                Ok(data) => {
                    let humidity = format!("{}.{}", data[0], data[1]).parse::<f32>()?;
                    let temperature = format!("{}.{}", data[2], data[3]).parse::<f32>()?;
                    break MeasurementResult{
                        temperature, humidity
                    };

                }
            }
        };
    
        Ok(measurement_result)
    }



    /// read the sensor data. 
    /// /[0/]: humidity integral
    /// /[1/]: humidity decimal
    /// /[2/]: temperature integral
    /// /[3/]: temperature decimal
    /// /[4/]: checksum
    fn read_data<D>(&mut self, delay: &mut D) -> Result<[u8; 5], DHT11Error>
        where D: DelayMs<u16> + DelayUs<u16>
    {
        let mut buffer = [0_u8; 5];

        for i in 0..40 {
            if self.read_bit(delay)? {
                buffer[i / 8] |= 1 << ((7 - i % 8));
            }
        }

        let checksum = buffer[0]
            .wrapping_add(buffer[1])
            .wrapping_add(buffer[2])
            .wrapping_add(buffer[3]);

        if checksum == buffer[4] {
            Ok(buffer)
        } else {
            Err(DHT11Error::ChecksumError)
        }
    }

    /// Set up the connection to the DHT11
    fn setup<D>(&mut self, delay: &mut D) -> Result<(), DHT11Error>
        where D: DelayMs<u16> + DelayUs<u16>
    {

        // send a start signal.
        self.to_output_mode();
        self.set_high_level();
        delay.delay_ms(1);

        self.set_low_level();
        delay.delay_ms(20);

        self.to_input_mode();
        delay.delay_us(20);

        self.read_bit(delay)?;
        
        Ok(())
    }

    fn read_bit<D>(&mut self, delay: &mut D) -> Result<bool, DHT11Error>
        where D: DelayMs<u16> + DelayUs<u16>
    {
        let low_time = self.wait_pulse(Level::High, delay)?;
        let high_time = self.wait_pulse(Level::Low, delay)?;

        Ok(high_time > low_time)
    }

    /// wait until it changes to the disired voltage.
    fn wait_pulse<D>(&mut self, level: Level, delay: &mut D) -> Result<u32, DHT11Error>
        where D: DelayMs<u16> + DelayUs<u16>
    {

        if self.pin.mode() != Mode::Input {
            self.to_input_mode();
        }

        let mut count = 0;

        while self.pin.read() != level {
            count += 1;
            if count > TIME_OUT_USEC {
                return Err(DHT11Error::SensorReadingTimeOut)
            }
            delay.delay_us(1);
        }

        Ok(u32::from(count))
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
