use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fs::read_to_string;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
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
    let mut x_axis_motor = StepperNEMA17::new(1, 2, [3, 4, 5]);
    let y_axis_motor = Arc::new(RwLock::new(StepperNEMA17::new(18, 15, [7, 5, 3])));
    // Create a clone to be used in a thread
    let y_axis_motor_clone = y_axis_motor.clone();

    let (t_main, r_main) = mpsc::channel();
    let thread = y_axis_motor_thread(y_axis_motor_clone, r_main);

    let one_unit_distance = 1000 as f32;
    let moving_time = 400 as i64; // micro-seconds

    
    for string in parsed_gcode_lines {
        for gcodes in string.gcodes() {
            let words_vector = gcodes.arguments();
            // Some return an empty vec. so check
            if words_vector.len() > 0 {
                // Loop through words and find Letters
                for word_info in words_vector {
                    // CHECK Y
                    if word_info.letter == 'Y' {
                        let last_pos = *y_axis_motor.write().unwrap().mut_last_position_value();
                        let mut diff = last_pos.abs() - word_info.value.abs();
                        diff = (diff * 1000.0).round() / 1000.0;
                        let steps = (diff * one_unit_distance) as i64;
                        // let velocity = one_unit_distance * diff.abs() / moving_time;
                        if steps != 0 {
                            let raw_delay = (moving_time as f32 / steps as f32) * 1000.0;
                            let delay_abs = raw_delay.abs().round() as u64;
                            t_main.send((steps, delay_abs)).unwrap();
                        }
                        *y_axis_motor.write().unwrap().mut_last_position_value() = word_info.value;
                    }
                    
                    // CHECK X                    
                    if word_info.letter == 'X' {
                        let last_pos = *x_axis_motor.mut_last_position_value();
                        let diff = last_pos.abs() - word_info.value.abs();
                        // println!("{}: {:?}", 'X', diff.abs());
                        // println!("{}", 'X');
                        *x_axis_motor.mut_last_position_value() = word_info.value;
                    }
                }
                thread::sleep(Duration::from_millis(300));
            }
        }
    }
    Ok(thread.join().unwrap())
}

fn y_axis_motor_thread(y_axis_motor_clone: Arc<RwLock<StepperNEMA17>>, r_main: Receiver<(i64, u64)>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for received in r_main {
            let y_axis_motor_clone = y_axis_motor_clone.write().unwrap();
            let (steps, delay_abs): (i64, u64) = received;
            // println!("Steps: {:?}, {:?}", steps, delay_abs);
            println!("{}", steps);
            if steps > 0 {
                y_axis_motor_clone.rotate(steps.abs(), delay_abs, Direction::CW).unwrap();
            } else if steps < 0 {
                y_axis_motor_clone.rotate(steps.abs(), delay_abs, Direction::CCW).unwrap();
            }
        }
        // thread::sleep(Duration::from_millis(wait_time));
        // x_axis_motor.rotate(3000, 50, Direction::CCW).unwrap();
    })
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}