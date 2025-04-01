use bevy::prelude::*;

use crate::ai::AiRegister;

pub struct MinePlugin;

impl Plugin for MinePlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(PreStartup, register_ai);  

  }
}



fn register_ai(mut commands:Commands){
  commands.spawn( AiRegister::new("mine"));
}