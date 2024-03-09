use rppal::gpio::Gpio;
use std::process::Command;

fn main() {
    let mut fan_pin = Gpio::new().unwrap().get(18).unwrap().into_output();
    fan_pin.set_low();

    loop {
        let output = Command::new("vcgencmd")
            .arg("measure_temp")
            .output()
            .expect("failed to get temp");

        let result = String::from_utf8(output.stdout).unwrap();

        // Split the string on '='
        let temp_str = result.split('=').nth(1).unwrap();
        let temp_f = temp_str.split("'C").next().unwrap();

        let temp: f32 = temp_f.parse().unwrap();

        if temp > 55.0 {
            // println!("CPU temperature {:.1} C - turning fan ON", temp);
            fan_pin.set_high();
        } else {
            // println!("CPU temperature {:.1} C - turning fan OFF", temp);
            fan_pin.set_low();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
