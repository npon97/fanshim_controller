use rppal::gpio::Gpio;
use rppal::sensors::TemperatureSensor;

fn main() {
  let mut fan_pin = Gpio::new().unwrap().get(25).unwrap().into_output();
  fan_pin.set_low();

  let sensor = TemperatureSensor::new().unwrap();

  loop {
    let temp = sensor.read_temperature().unwrap();
    if temp > 55.0 {
      println!("CPU temperature {:.1} C - turning fan ON", temp);
      fan_pin.set_high();
    } else {
      println!("CPU temperature {:.1} C - turning fan OFF", temp); 
      fan_pin.set_low();
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
  }
}
