use std::io;


fn main() {
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    let mode = mode.trim();

    if mode == "c" {
        let mut celsius_temp = String::new();
        io::stdin().read_line(&mut celsius_temp).expect("Failed");
        let celsius_temp: f32 = celsius_temp.trim().parse().expect("Error while parse temperature");
        let fahrenheit_temp: f32 = celsius_temp * 9.0 / 5.0 + 32.0;
        println!("Fahrenheit temp is {fahrenheit_temp}");
    } else if mode == "f" {
        let mut fahrenheit_temp = String::new();
        io::stdin().read_line(&mut fahrenheit_temp).expect("Failed");
        let fahrenheit_temp: f32 = fahrenheit_temp.trim().parse().expect("Error while parse temperature");
        let celsius_temp: f32 = (fahrenheit_temp - 32.0) * 5.0 / 9.0;
        println!("Celsius temp is {celsius_temp}");
    } else {
        println!("Unexpected mode {mode}");
    }
}