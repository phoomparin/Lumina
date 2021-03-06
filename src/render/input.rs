use super::game::Game;

const MOVE_FORWARD: u32 = 13;
const MOVE_LEFT: u32 = 0;
const MOVE_BACKWARD: u32 = 1;
const MOVE_RIGHT: u32 = 2;
const JUMP: u32 = 49;
const CROUCH: u32 = 56;

const ARROW_UP: u32 = 126;
const ARROW_DOWN: u32 = 125;
const ARROW_LEFT: u32 = 123;
const ARROW_RIGHT: u32 = 124;

// +; 24 => {}
// -; 27 => {}
// J; 38 => {}
// K; 40 => {}

pub trait Input {
  fn handle_keyboard(&mut self, scancode: u32);
}

impl<'a> Input for Game<'a> {
  fn handle_keyboard(&mut self, scancode: u32) {
    let camera = &mut self.camera;

    match scancode {
      MOVE_FORWARD => {
        camera.add_z(-0.1)
      }
      MOVE_LEFT => {
       camera.add_x(0.1)
      }
      MOVE_BACKWARD => {
        camera.add_z(0.1)
      }
      MOVE_RIGHT => {
        camera.add_x(-0.1)
      }
      JUMP => {
        camera.add_y(0.1)
      }
      CROUCH => {
        camera.add_y(-0.1)
      }
      ARROW_UP => {
        camera.add_pitch(0.1)
      }
      ARROW_DOWN => {
        camera.add_pitch(-0.1)
      }
      ARROW_LEFT => {
        camera.add_yaw(-0.1)
      }
      ARROW_RIGHT => {
        camera.add_yaw(0.1)
      }
      _ => println!("Unmapped Key: {}", &scancode)
    }
  }
}

