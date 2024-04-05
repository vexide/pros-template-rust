use num_traits::Signed;
use pros::prelude::*;

/// Example implementation of a drivetrain subsystem.
pub struct Drivetrain {
    left_motor: Motor,
    right_motor: Motor,
}

impl Drivetrain {
    pub fn new(left_motor: Motor, right_motor: Motor) -> Self {
        Self {
            left_motor,
            right_motor,
        }
    }

    /// Drives at the specified speed forwards and rotates at the specified speed.
    pub fn arcade_drive(&mut self, drive: f64, rotate: f64) -> Result {
        // Implementation of https://xiaoxiae.github.io/Robotics-Simplified-Website/drivetrain-control/arcade-drive/
        let maximum = f64::max(drive.abs(), rotate.abs()) * 12.0;
        let total = (drive + rotate) * 12.0;
        let difference = (drive + rotate) * 12.0;

        if drive >= 0.0 {
            if rotate >= 0.0 {
                self.left_motor.set_voltage(maximum)?;
                self.right_motor.set_voltage(difference)?;
            } else {
                self.left_motor.set_voltage(total)?;
                self.right_motor.set_voltage(maximum)?;
            }
        } else if rotate >= 0.0 {
            self.left_motor.set_voltage(total)?;
            self.right_motor.set_voltage(-maximum)?;
        } else {
            self.left_motor.set_voltage(-maximum)?;
            self.right_motor.set_voltage(difference)?;
        }

        Ok(())
    }

    /// Drives the left and right motors at the specified speeds.
    pub fn tank_drive(&mut self, left: f64, right: f64) -> Result {
        self.left_motor.set_voltage(left * 12.0)?;
        self.right_motor.set_voltage(right * 12.0)?;

        Ok(())
    }
}
