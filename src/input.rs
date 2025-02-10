use bevy::prelude::*;

use crate::scheduling::GameSchedule;


#[derive(PartialEq)]
pub enum InputEventType{
  Pressed,
  Released,
}


#[derive(PartialEq)]
pub enum InputEventAction{
  Shoot,
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_event::<InputMovementEvent>()
      .add_event::<InputTriggerEvent>()
      .add_systems(Update, read_keys.in_set(GameSchedule::UserInput));
  }
}


#[derive(Event)]
pub struct InputMovementEvent{
  pub direction:Vec2,
}

impl InputMovementEvent{
  pub fn new(direction:Vec2)-> Self {
    Self { direction }
  }
}

#[derive(Event)]
pub struct InputTriggerEvent{
  pub action: InputEventAction,
  pub input_type:InputEventType,
}

impl InputTriggerEvent{
  pub fn new(action:InputEventAction, input_type:InputEventType)-> Self {
    Self { action, input_type }
  }
}




fn read_keys(
  keyboard_input: Res<ButtonInput<KeyCode>>, 
  mut ev_movement_event:EventWriter<InputMovementEvent>,
  mut ev_trigger_event:EventWriter<InputTriggerEvent>,
){
  let mut dir:Vec2 = Vec2::ZERO;
  if keyboard_input.pressed(KeyCode::KeyD) {
    dir.x -= 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyA) {
    dir.x += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyW) {
    dir.y += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyS) {
    dir.y -= 1.;
  }
  if (dir!= Vec2::ZERO){
    ev_movement_event.send(InputMovementEvent::new(dir));
  }

  if keyboard_input.just_pressed(KeyCode::Space){
    ev_trigger_event.send(InputTriggerEvent::new(InputEventAction::Shoot, InputEventType::Pressed));
  }

  if keyboard_input.just_released(KeyCode::Space){
    ev_trigger_event.send(InputTriggerEvent::new(InputEventAction::Shoot, InputEventType::Released));
  }
}



