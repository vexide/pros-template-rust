#![no_main]
#![no_std]

use core::time::Duration;

use pros::prelude::*;

/// Store robot state that will be used throughout the program.
struct VexRobot {
    controller: Controller,
}

impl VexRobot {
    /// Create subsystems and set up anything that will be used throughout the program.
    /// This function is run by PROS as soon as the robot program is selected.
    pub fn new() -> Self {
        Self {
            controller: Controller::Master,
        }
    }
}

impl Robot for VexRobot {
    /// Runs when the robot is enabled.
    fn opcontrol(&mut self) -> pros::Result {
        loop {
            let input = self.controller.state();
            let Joysticks { left, right } = input.joysticks;

            println!("Speed %: {:?}, Turn %: {:?}", left.y, right.x);

            sleep(Duration::from_millis(20));
        }
    }
}

// Register the robot with PROS so that its methods will be called.
robot!(VexRobot, VexRobot::new());
