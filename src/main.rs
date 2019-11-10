use std::error::Error;
use std::thread;
use std::time::Duration;
// use rppal::gpio::Gpio;

// mod stepper_28byj48;
// use stepper_28byj48::Stepper28BYJ48;

mod stepper_nema17;
use stepper_nema17::StepperNEMA17;

mod enums;
use enums::{Direction};
// - - - - - - - - - - - - - - - - - - - - - - - - 
// Note that - GPIO uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// - - - - - - - - - - - - - - - - - - - - - - - -

fn main() -> Result<(), Box<dyn Error>> {
    // Example Implementation of new NEMA17 Motor
    let my_motor = StepperNEMA17::new(18, 15, [7, 5, 3]);
    let wait_time = 1000;
    my_motor.rotate(500, 300, Direction::CW);
    thread::sleep(Duration::from_millis(wait_time));
    my_motor.rotate(500, 300, Direction::CCW);
    thread::sleep(Duration::from_millis(wait_time));
    my_motor.rotate(500, 300, Direction::CW);
    thread::sleep(Duration::from_millis(wait_time));
    my_motor.rotate(500, 300, Direction::CCW);

    Ok(())
}

