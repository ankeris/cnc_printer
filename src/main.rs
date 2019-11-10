use std::error::Error;
use std::thread;
use std::time::Duration;

mod stepper_28byj48;
use stepper_28byj48::Stepper28BYJ48;

mod enums;
use enums::Direction;
// - - - - - - - - - - - - - - - - - - - - - - - - 
// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// - - - - - - - - - - - - - - - - - - - - - - - -

fn main() -> Result<(), Box<dyn Error>> {
    let my_motor = Stepper28BYJ48::new([18,17,27,22]);

    my_motor.rotate(50.0, Direction::CW);
    thread::sleep(Duration::from_millis(200));
    my_motor.rotate(50.0, Direction::CCW);

    Ok(())
}