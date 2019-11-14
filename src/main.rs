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
    let parsed_gcode_lines = parse(string_to_static_str(gcode_from_file));

    // Implementation of new NEMA17 Y Axis controll Motor
    let mut y_axis_motor = StepperNEMA17::new(18, 15, [7, 5, 3]);

    for string in parsed_gcode_lines {
        for gcodes in string.gcodes() {
            let words_vector = gcodes.arguments();
            // some return empty vec. so check
            if words_vector.len() > 0 {
                // Loop through words and find Letters
                for word_info in words_vector {

                    // Move Y axis
                    if word_info.letter == 'Y' {
                        let last_pos = *y_axis_motor.mut_last_position_value();
                        let diff = f32::abs_sub(last_pos, word_info.value);
                        println!("{:?}", diff);
                        *y_axis_motor.mut_last_position_value() = word_info.value;
                    }
                }
            }
        }
    }
    
    
    let wait_time = 1000;
    y_axis_motor.rotate(6000, 50, Direction::CW).unwrap();
    thread::sleep(Duration::from_millis(wait_time));
    y_axis_motor.rotate(6000, 50, Direction::CCW).unwrap();
    // y_axis_motor.rotate(3000, 50, Direction::CW).unwrap();
    // thread::sleep(Duration::from_millis(wait_time));
    // y_axis_motor.rotate(5000, 50, Direction::CCW).unwrap();
    // thread::sleep(Duration::from_millis(wait_time));
    // y_axis_motor.rotate(5000, 50, Direction::CW).unwrap();
    // thread::sleep(Duration::from_millis(wait_time));
    // y_axis_motor.rotate(3000, 50, Direction::CCW).unwrap();

    Ok(())
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}