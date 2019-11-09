use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;


#[derive(Debug)]
pub struct Stepper28BYJ48 {
    pins_in_right_order: [i8; 4],
}

impl Stepper28BYJ48 {
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

    #[inline]
    fn new(pins_in_right_order: [i8; 4]) -> Stepper28BYJ48 {
        Pin { pins_in_right_order }
    }

    fn motor_sequence_setup(&self) -> mut [OutputPin; 4]{
        let mut pin1: OutputPin = Gpio::new()?.get(18)?.into_output();
        let mut pin2: OutputPin = Gpio::new()?.get(17)?.into_output();
        let mut pin3: OutputPin = Gpio::new()?.get(27)?.into_output();
        let mut pin4: OutputPin = Gpio::new()?.get(22)?.into_output();

        mut [
            pin1,
            pin2,
            pin3,
            pin4
        ]
    }

    #[inline]
    pub fn rotate(&self, degree: f32) {
        let steps_calculation: f32 = degree as f32 * 11.377777777777;

        let mut row_pos: usize = 0;
        
        let steps: i64 = steps_calculation.round() as i64;

        for _row in 0..steps {
            for (idx, pin) in self.motor_sequence_setup().iter_mut().enumerate() {
                if self.STEP_SEQUENCE[row_pos][idx] != 0 {
                    pin.set_high();
                }
                else {
                    pin.set_low();
                }
            }

            row_pos += 1;
        
            if row_pos >= self.STEP_SEQUENCE.len() {
                row_pos = 0
            }

            thread::sleep(Duration::from_millis(2));
        }

        Ok(for pin in self.motor_sequence_setup().iter_mut() {
                pin.set_low();
        })
    }
}