#![no_main]
#![no_std]

use core::time::Duration;

use pros::prelude::*;

/// Store robot state that will be used throughout the program.
struct VexRobot {
    drive_motor: Motor,
}

impl VexRobot {
    /// Create subsystems and set up anything that will be used throughout the program.
    /// This function is run by PROS as soon as the robot program is selected.
    pub fn new() -> Self {
        Self {
            drive_motor: Motor::new(1, BrakeMode::Brake).unwrap(),
        }
    }
}

impl SyncRobot for VexRobot {
    /// Runs when the robot is enabled in autonomous mode.
    fn auto(&mut self) -> pros::Result {
        self.drive_motor.set_output(1.0)?;
        sleep(Duration::from_secs(2));
        self.drive_motor.set_output(0.0)?;
        Ok(())
    }

    /// Runs when the robot is enabled in driver control mode.
    fn opcontrol(&mut self) -> pros::Result {
        loop {
            let joysticks = Controller::Master.state().joysticks;

            println!("Speed %: {:?}", joysticks.left.y);
            self.drive_motor.set_output(joysticks.left.y)?;

            sleep(Duration::from_millis(20));
        }
    }
}

// Register the robot with PROS so that its methods will be called.
sync_robot!(VexRobot, VexRobot::new());
