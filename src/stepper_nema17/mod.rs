// # # # # # # # # # # # # # # # # # # # # # 
// # Motor:   Nema 17 (Usongshine rework)
// # Driver:  A4988
// # # # # # # # # # # # # # # # # # # # # # 

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

use crate::enums::{Direction, MicroStepping};

#[derive(Debug)]
pub struct StepperNEMA17 {
    step_pin: u8,
    direction_pin: u8,
    micro_stepping_pins: [u8; 3],
    micro_stepping_values: MicroStepping,
    last_position_value: f32,
}

impl StepperNEMA17 {
    #[inline]
    fn _default_micro_stepping_values(&self) -> MicroStepping {
        (1, 0, 0)
    }

    #[inline]
    pub fn new(step_pin: u8, direction_pin: u8, micro_stepping_pins: [u8; 3]) -> StepperNEMA17 {
        StepperNEMA17 {
            step_pin: step_pin,
            direction_pin,
            micro_stepping_pins,
            micro_stepping_values: (1, 0, 0),
            last_position_value: 0.0,
        }
    }

    pub fn _set_micro_stepping(&mut self, micro_step_values: MicroStepping) {
        self.micro_stepping_values = micro_step_values;
    }

    pub fn mut_last_position_value(&mut self) -> &mut f32 {
        &mut self.last_position_value
    }

    pub fn rotate(&self, steps: i64, speed: u64, direction: Direction) -> Result<(), Box<dyn Error>> {
        let mut st_pin = Gpio::new()?.get(self.step_pin)?.into_output();
        let mut dir_pin = Gpio::new()?.get(self.direction_pin)?.into_output();

        let mut ms1_step: OutputPin = Gpio::new()?.get(self.micro_stepping_pins[0])?.into_output();
        let mut ms2_step: OutputPin = Gpio::new()?.get(self.micro_stepping_pins[1])?.into_output();
        let mut ms3_step: OutputPin = Gpio::new()?.get(self.micro_stepping_pins[2])?.into_output();

        ms1_step.set_high();
        ms2_step.set_low();
        ms3_step.set_high();

        match direction {
            Direction::CW => {
                dir_pin.set_high();
            }
            Direction::CCW => {
                dir_pin.set_low();
            }
        }

        Ok(
            for _x in 0..steps {
                st_pin.set_high();
                thread::sleep(Duration::from_micros(speed));
                st_pin.set_low();
                thread::sleep(Duration::from_micros(speed));
            }
        )
    }
}