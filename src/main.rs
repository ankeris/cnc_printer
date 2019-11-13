use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fs::read_to_string;
// use rppal::gpio::Gpio;

// mod stepper_28byj48;
// use stepper_28byj48::Stepper28BYJ48;
use gcode::parse;
mod stepper_nema17;
use stepper_nema17::StepperNEMA17;

mod enums;
use enums::{Direction};
// - - - - - - - - - - - - - - - - - - - - - - - - 
// Note that - GPIO uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// - - - - - - - - - - - - - - - - - - - - - - - -

fn main() -> Result<(), Box<dyn Error>> {
    let gcode_from_file = read_to_string("src/test.ngc").unwrap();
    let smth = parse(string_to_static_str(gcode_from_file));

    for string in smth {
        for string2 in string.gcodes() {
            if string2.arguments().len() > 0 {
                println!("{:?}", string2.arguments());
            }
        }
    }
    
    // Example Implementation of new NEMA17 Motor
    let my_motor = StepperNEMA17::new(18, 15, [7, 5, 3]);
    let wait_time = 1000;

    my_motor.rotate(5000, 50, Direction::CW).unwrap();
    thread::sleep(Duration::from_millis(wait_time));
    my_motor.rotate(12500, 50, Direction::CCW).unwrap();
    thread::sleep(Duration::from_millis(wait_time));
    my_motor.rotate(12500, 50, Direction::CW).unwrap();
    thread::sleep(Duration::from_millis(wait_time));
    my_motor.rotate(5000, 50, Direction::CCW).unwrap();

    Ok(())
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}