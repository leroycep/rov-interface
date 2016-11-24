
/// A mock ROV that reflects the state of the ROV.

struct MockRov {
    pub motors: [i16; 6],
    pub light_relay: bool,
}

impl MockRov {
    pub fn new() -> MockRov {
        MockRov {
            motors: [0; 6],
            light_relay: false,
        }
    }

    pub fn apply_commands(&mut self, commands: &Vec<RovCommand>) {
        for command in commands.iter() {
            self.apply_command(command);
        }
    }

    pub fn apply_command(&mut self, command: &RovCommand) {
        match *command {
            RovCommand::ControlMotor { id, throttle } => {
                if id < motors.len() {
                    self.motors[id] = throttle;
                }
            }
            RovCommand::CollectSamples { amount } => {
                unimplemented!();
            }
            RovCommand::LightsOn => self.light_relay = true,
            RovCommand::LightsOff => self.light_relay = false,
        }
    }
}