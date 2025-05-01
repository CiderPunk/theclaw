use bevy::prelude::*;



pub struct DropshipPlugin;

impl Plugin for DropshipPlugin{
  fn build(&self, app: &mut App) {    
 //   app.add_systems(PreStartup, register_ai);  
  }
}

#[derive(Component)]
pub struct DropShip{

}

