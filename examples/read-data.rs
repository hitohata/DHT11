use dht11::DHT11;
use rppal::hal::Delay;

fn main() {
    let mut dht11 = DHT11::new(5).unwrap();
    let mut delay = Delay::new();

    loop {
        println!("{:?}", dht11.read(&mut delay));
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
