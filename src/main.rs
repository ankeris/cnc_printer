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

    // Constants
    const ONE_UNIT_DISTANCE: f32 = 500.0;
    const MOVING_TIME: i64 = 600; // micro-seconds

    // Implementation of new NEMA17 X, Y Axis control Motors
    let x_axis_motor = Arc::new(RwLock::new(StepperNEMA17::new(24, 23, [26, 19, 6])));
    let y_axis_motor = Arc::new(RwLock::new(StepperNEMA17::new(18, 15, [4, 3, 2])));

    // Create motor clones to be used in a threads
    let y_axis_motor_clone = y_axis_motor.clone();
    let x_axis_motor_clone = x_axis_motor.clone();

    // Spawn a thread and initialize it with y_axis clone motor
    let (y_thread_sender, y_thread_receiver) = mpsc::channel();
    let (x_thread_sender, x_thread_receiver) = mpsc::channel();
    let y_thread = create_motor_thread(y_axis_motor_clone, y_thread_receiver);
    let x_thread = create_motor_thread(x_axis_motor_clone, x_thread_receiver);

    for string in parsed_gcode_lines {
        for gcodes in string.gcodes() {
            let words_vector = gcodes.arguments();
            // Some return an empty vec. so check
            if words_vector.len() > 0 {
                println!("{:?}", words_vector);
                // Loop through words and find Letters
                for word_info in words_vector {
                    // CHECK Y
                    if word_info.letter == 'Y' {
                        let last_pos = *y_axis_motor.write().unwrap().mut_last_position_value();
                        let diff = last_pos.abs() - word_info.value.abs();
                        let steps = (diff * ONE_UNIT_DISTANCE) as i64;
                        if steps != 0 {
                            let raw_delay = (MOVING_TIME as f32 / steps as f32) * 1000.0;
                            let delay_abs = raw_delay.abs().round() as u64;
                            y_thread_sender.send((steps.abs(), delay_abs)).unwrap();
                        }
                        *y_axis_motor.write().unwrap().mut_last_position_value() = word_info.value;
                    }
                    
                    // CHECK X                    
                    if word_info.letter == 'X' {
                        let last_pos = *x_axis_motor.write().unwrap().mut_last_position_value();
                        let diff = last_pos.abs() - word_info.value.abs();
                        let steps = (diff * ONE_UNIT_DISTANCE) as i64;
                        if steps != 0 {
                            let raw_delay = (MOVING_TIME as f32 / steps as f32) * 1000.0;
                            let delay_abs = raw_delay.abs().round() as u64;
                            x_thread_sender.send((steps.abs(), delay_abs)).unwrap();
                            // println!("{}: {:?}", 'X', raw_delay);
                        }
                        *x_axis_motor.write().unwrap().mut_last_position_value() = word_info.value;
                    }
                }

                thread::sleep(Duration::from_millis(1000));
            }
        }
    }
    // y_thread_sender.send((1600 * 2, 500)).unwrap();
    // x_axis_motor.rotate(1600 * 2, 500, Direction::CCW).unwrap();
    Ok({
        y_thread.join().unwrap();
        x_thread.join().unwrap();
    })
}

fn create_motor_thread(motor_clone: Arc<RwLock<StepperNEMA17>>, thread_receiver: Receiver<(i64, u64)>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for received in thread_receiver {
            let motor_clone = motor_clone.write().unwrap();
            let (steps, delay_abs): (i64, u64) = received;
            // println!("Y: {}", steps);
            if steps > 0 {
                motor_clone.rotate(steps.abs(), delay_abs, Direction::CCW).unwrap();
            } else if steps < 0 {
                motor_clone.rotate(steps.abs(), delay_abs, Direction::CW).unwrap();
            }
        }
    })
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}