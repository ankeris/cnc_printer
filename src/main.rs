use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_LED: u8 = 14;

const STEP_SEQUENCE: [[i8; 4]; 8] = [
    [1,0,0,1],
	[1,0,0,0],
	[1,1,0,0],
	[0,1,0,0],
	[0,1,1,0],
	[0,0,1,0],
	[0,0,1,1],
	[0,0,0,1]
];

fn main() -> Result<(), Box<dyn Error>> {
    let degree: f32 = 360 as f32 * 11.377777777777;

    // Retrieve the GPIO pin and configure it as an output.
    let mut _pin18: OutputPin = Gpio::new()?.get(18)?.into_output();
    let mut _pin17: OutputPin = Gpio::new()?.get(17)?.into_output();
    let mut _pin27: OutputPin = Gpio::new()?.get(27)?.into_output();
    let mut _pin22: OutputPin = Gpio::new()?.get(22)?.into_output();

    let mut motor_sequence_setup: [OutputPin; 4] = [
        _pin18,
        _pin17,
        _pin27,
        _pin22
    ];

    let mut row_pos: usize = 0;
    
    let smth: i64 = degree.round() as i64;

    for _row in 0..smth {
        for (idx, pin) in motor_sequence_setup.iter_mut().enumerate() {
            if STEP_SEQUENCE[row_pos][idx] != 0 {
                pin.set_high();
            }
            else {
                pin.set_low();
            }
        }

        row_pos += 1;
    
        if row_pos >= STEP_SEQUENCE.len() {
            row_pos = 0
        }

        thread::sleep(Duration::from_millis(2));
    }

    Ok(for pin in motor_sequence_setup.iter_mut() {
            pin.set_low();
    })
}