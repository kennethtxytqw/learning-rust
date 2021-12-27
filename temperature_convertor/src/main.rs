use std::io;

const CONVERSIONS: [fn(f64) -> f64; 2] = [to_celsius, to_fahrenheit];

fn main() {
  println!("Input temperature conversion\n(0) fahrenheit to celsius\n(1) fahrenheit to celsius");
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
  let input: usize = input.trim().parse().expect("Please input a number!");
  if input > CONVERSIONS.len() {
    panic!("Input ({})is larger than selectable range", input)
  }

  let convertor = CONVERSIONS[input];

  println!("Input temperature in fahrenheit");
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
  let input: f64 = input.trim().parse().expect("Please input a number!");

  println!("Output: {}", convertor(input))
}

fn to_celsius(temperature_in_fahrenheit: f64) -> f64 {
  (temperature_in_fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(temperature_in_celsius: f64) -> f64 {
  (temperature_in_celsius * 1.8) + 32.0
}
