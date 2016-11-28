
use rov::Rov;
use mock::MockRov;
use control_state::{ControlState, ThrustMode, SamplerReleaseMode};
use screen::{Engine, Screen, Trans};
use time::{PreciseTime, Duration};
use vecmath;
use sdl2::pixels::Color;
use util::draw_text;

pub struct RovControl {
    control_state: ControlState,
    prev_control_state: ControlState,
    last_write_time: PreciseTime,
    rov: Rov,
    mock_rov: MockRov,
}

impl RovControl {
    pub fn new(rov: Rov) -> RovControl {
        RovControl {
            control_state: ControlState::new(),
            prev_control_state: ControlState::new(),
            last_write_time: PreciseTime::now(),
            rov: rov,
            mock_rov: MockRov::new(),
        }
    }
}

impl Screen for RovControl {
    fn update(&mut self, engine: &mut Engine) -> Trans {
        for event in engine.event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            use sdl2::controller::Axis;
            use sdl2::controller::Button;

            match event {
                Event::ControllerAxisMotion { axis: Axis::LeftY, value: val, .. } => {
                    self.control_state.forward_thrust = val as f64 / 32768.0
                }
                Event::ControllerAxisMotion { axis: Axis::LeftX, value: val, .. } => {
                    self.control_state.sideways_thrust = val as f64 / 32768.0
                }
                Event::ControllerAxisMotion { axis: Axis::RightX, value: val, .. } => {
                    self.control_state.rotational_thrust = val as f64 / 32768.0
                }
                Event::ControllerAxisMotion { axis: Axis::TriggerLeft, value: val, .. } => {
                    self.control_state.ascent_thrust = val as f64 / 32768.0;
                }
                Event::ControllerAxisMotion { axis: Axis::TriggerRight, value: val, .. } => {
                    self.control_state.descent_thrust = val as f64 / 32768.0;
                }
                Event::ControllerButtonDown { button: Button::Y, .. } => {
                    self.control_state.power_lights = !self.control_state.power_lights
                }
                Event::ControllerButtonUp { button: Button::RightShoulder, .. } => {
                    self.control_state.thrust_mode = ThrustMode::Normal
                }
                Event::ControllerButtonDown { button: Button::RightShoulder, .. } => {
                    self.control_state.thrust_mode = ThrustMode::Emergency
                }
                Event::ControllerButtonDown { button: Button::Start, .. } => {
                    self.control_state.power_master = !self.control_state.power_master
                }
                Event::ControllerButtonDown { button: Button::B, .. } => {
                    self.control_state.sampler_release = true
                }
                Event::ControllerButtonUp { button: Button::LeftShoulder, .. } => {
                    self.control_state.sampler_release_mode = SamplerReleaseMode::One
                }
                Event::ControllerButtonDown { button: Button::LeftShoulder, .. } => {
                    self.control_state.sampler_release_mode = SamplerReleaseMode::All
                }
                Event::Quit { .. } |
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => return Trans::Quit,
                _ => (),
            }
        }

        let now = PreciseTime::now();
        if self.last_write_time.to(now) >= Duration::milliseconds(5) {
            let mut buffer = vec![];
            self.control_state.generate_commands_diff(&self.prev_control_state, &mut buffer);
            self.mock_rov.apply_commands(&buffer);
            for command in buffer.iter() {
                self.rov.send_command(command.clone()).expect("Failed to update rov");
            }

            self.prev_control_state = self.control_state.clone();
            self.control_state.sampler_release = false;
            self.last_write_time = now;
        }
        self.mock_rov.update();

        for response in self.rov.responses().iter() {
            pintln!([response]);
        }

        engine.renderer.set_draw_color(Color::RGB(255, 128, 128));
        engine.renderer.clear();

        engine.renderer.set_draw_color(Color::RGB(255, 255, 255));

        let rect = (30, 450, 70, 70).into();
        if self.mock_rov.robot_is_on {
            engine.renderer.fill_rect(rect).unwrap()
        } else {
            engine.renderer.draw_rect(rect).unwrap()
        }
        draw_text(&mut engine.renderer, &engine.font, "Master", [30, 510]);

        let rect = (120, 450, 50, 50).into();
        if self.mock_rov.light_relay {
            engine.renderer.fill_rect(rect).unwrap()
        } else {
            engine.renderer.draw_rect(rect).unwrap()
        }

        let rect = (180, 450, 50, 50).into();
        if self.mock_rov.sampler_relay {
            engine.renderer.fill_rect(rect).unwrap()
        } else {
            engine.renderer.draw_rect(rect).unwrap()
        }

        {
            use control_state::{MOTOR_1, MOTOR_2, MOTOR_3, MOTOR_4, MOTOR_5, MOTOR_6};
            // Render mock rov
            let motor_1_start = [430.0, 260.0];
            let motor_2_start = [370.0, 260.0];
            let motor_3_start = [430.0, 340.0];
            let motor_4_start = [370.0, 340.0];
            let motor_5_start = [500.0, 300.0];
            let motor_6_start = [560.0, 300.0];

            let multiplier = 60.0 / i16::max_value() as f64;
            let motor_1_vector = vecmath::vec2_scale([-0.5, -0.5],
                                                     self.mock_rov.motors[MOTOR_1 as usize] as f64 *
                                                     multiplier);
            let motor_2_vector = vecmath::vec2_scale([0.5, -0.5],
                                                     self.mock_rov.motors[MOTOR_2 as usize] as f64 *
                                                     multiplier);
            let motor_3_vector = vecmath::vec2_scale([-0.5, 0.5],
                                                     self.mock_rov.motors[MOTOR_3 as usize] as f64 *
                                                     multiplier);
            let motor_4_vector = vecmath::vec2_scale([0.5, 0.5],
                                                     self.mock_rov.motors[MOTOR_4 as usize] as f64 *
                                                     multiplier);
            let motor_5_vector = vecmath::vec2_scale([0.0, 1.0],
                                                     self.mock_rov.motors[MOTOR_5 as usize] as f64 *
                                                     multiplier);
            let motor_6_vector = vecmath::vec2_scale([0.0, 1.0],
                                                     self.mock_rov.motors[MOTOR_6 as usize] as f64 *
                                                     multiplier);
            let motor_1_end = vecmath::vec2_add(motor_1_start, motor_1_vector);
            let motor_2_end = vecmath::vec2_add(motor_2_start, motor_2_vector);
            let motor_3_end = vecmath::vec2_add(motor_3_start, motor_3_vector);
            let motor_4_end = vecmath::vec2_add(motor_4_start, motor_4_vector);
            let motor_5_end = vecmath::vec2_add(motor_5_start, motor_5_vector);
            let motor_6_end = vecmath::vec2_add(motor_6_start, motor_6_vector);

            engine.renderer
                .draw_line((motor_1_start[0] as i32, motor_1_start[1] as i32).into(),
                           (motor_1_end[0] as i32, motor_1_end[1] as i32).into())
                .unwrap();
            engine.renderer
                .draw_line((motor_2_start[0] as i32, motor_2_start[1] as i32).into(),
                           (motor_2_end[0] as i32, motor_2_end[1] as i32).into())
                .unwrap();
            engine.renderer
                .draw_line((motor_3_start[0] as i32, motor_3_start[1] as i32).into(),
                           (motor_3_end[0] as i32, motor_3_end[1] as i32).into())
                .unwrap();
            engine.renderer
                .draw_line((motor_4_start[0] as i32, motor_4_start[1] as i32).into(),
                           (motor_4_end[0] as i32, motor_4_end[1] as i32).into())
                .unwrap();
            engine.renderer
                .draw_line((motor_5_start[0] as i32, motor_5_start[1] as i32).into(),
                           (motor_5_end[0] as i32, motor_5_end[1] as i32).into())
                .unwrap();
            engine.renderer
                .draw_line((motor_6_start[0] as i32, motor_6_start[1] as i32).into(),
                           (motor_6_end[0] as i32, motor_6_end[1] as i32).into())
                .unwrap();
        }

        engine.renderer.present();

        Trans::None
    }
}
