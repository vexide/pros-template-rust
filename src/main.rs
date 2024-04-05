#![no_main]
#![no_std]

use core::time::Duration;

use pros::{devices::Controller, prelude::*};

use crate::drivetrain::Drivetrain;

mod drivetrain;

/// Store robot state and subsystems that will be used throughout the program.
struct VexRobot {
    drivetrain: Drivetrain,
}

impl VexRobot {
    /// Create subsystems and set up anything that will be used throughout the program.
    /// This function is run by PROS as soon as the robot program is selected.
    pub fn new(peripherals: Peripherals) -> Self {
        Self {
            drivetrain: Drivetrain::new(
                Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward).unwrap(),
                Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward).unwrap(),
            ),
        }
    }
}

impl AsyncRobot for VexRobot {
    /// Runs when the robot is enabled in autonomous mode.
    async fn auto(&mut self) -> Result {
        self.drivetrain.tank_drive(1.0, 1.0)?;
        sleep(Duration::from_secs(2)).await;
        self.drivetrain.tank_drive(0.0, 0.0)?;
        Ok(())
    }

    /// Runs when the robot is enabled in driver control mode.
    async fn opcontrol(&mut self) -> Result {
        loop {
            let joysticks = Controller::Master.state().unwrap().joysticks;

            self.drivetrain
                .arcade_drive(joysticks.left.y as f64, joysticks.right.x as f64)?;

            sleep(Duration::from_millis(2)).await;
        }
    }
}

// Register the robot with PROS so that its methods will be called.
async_robot!(VexRobot, VexRobot::new(Peripherals::take().unwrap()));
