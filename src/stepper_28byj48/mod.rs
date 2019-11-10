// # # # # # # # # # # # # # # # # # # # # # 
// # Motor:   28BYJ_49
// # Driver:  ULN2003
// # # # # # # # # # # # # # # # # # # # # # 

// # # # I N I T.  E X A M P L E # # # # # #
// #
// # let my_motor = Stepper28BYJ48::new([18,17,27,22]);
// # my_motor.rotate(50.0, Direction::CW);
// # thread::sleep(Duration::from_millis(200));
// # my_motor.rotate(50.0, Direction::CCW);
// #
// # # # # # # # # # # # # # # # # # # # # #

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

use crate::enums::Direction;

static STEP_SEQUENCE: [[i8; 4]; 8] = [
    [1,0,0,1],
    [1,0,0,0],
    [1,1,0,0],
    [0,1,0,0],
    [0,1,1,0],
    [0,0,1,0],
    [0,0,1,1],
    [0,0,0,1]
];

#[derive(Debug)]
pub struct Stepper28BYJ48 {
    pins_in_right_order: [u8; 4],
}

impl Stepper28BYJ48 {

    #[inline]
    pub(crate) fn new(pins_in_right_order: [u8; 4]) -> Stepper28BYJ48 {
        Stepper28BYJ48 { pins_in_right_order }
    }

    #[inline]
    pub fn motor_sequence_setup(&self) -> Result<([OutputPin; 4]), Box<dyn Error>> {
        let p_i_o: [u8; 4] = self.pins_in_right_order;
        let pin1: OutputPin = Gpio::new()?.get(p_i_o[0])?.into_output();
        let pin2: OutputPin = Gpio::new()?.get(p_i_o[1])?.into_output();
        let pin3: OutputPin = Gpio::new()?.get(p_i_o[2])?.into_output();
        let pin4: OutputPin = Gpio::new()?.get(p_i_o[3])?.into_output();

        Ok([
            pin1,
            pin2,
            pin3,
            pin4
        ])
    }

    #[inline]
    pub fn rotate(&self, degree: f32, direction: Direction) {
        let steps_calculation: f32 = degree as f32 * 11.377777777777;
        let mut row_pos: usize = 0;
        let steps: i64 = steps_calculation.round() as i64;

        let mut motor_sequence_setup = self.motor_sequence_setup().unwrap();
        
        for _row in 0..steps {
            for (idx, pin) in motor_sequence_setup.iter_mut().enumerate() {
                if STEP_SEQUENCE[row_pos][idx] != 0 {
                    pin.set_high();
                }
                else {
                    pin.set_low();
                }
            }

            match direction {
                Direction::CW => {
                    row_pos += 1;
                    if row_pos >= STEP_SEQUENCE.len() {
                        row_pos = 0
                    }
                }
                Direction::CCW => {
                    if row_pos <= 0 {
                        row_pos = STEP_SEQUENCE.len()
                    }
                    row_pos -= 1;
                }
            }

            thread::sleep(Duration::from_millis(2));
        }

        
        for pin in motor_sequence_setup.iter_mut() {
            pin.set_low();
        };
    }
}