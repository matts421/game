use bevy::prelude::*;


pub struct MovementPlugin;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}


impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Update, update_position);
    }
}


pub fn update_position(mut query: Query<(&Velocity, &mut Transform)>){
    for (velocity, mut transform) in query.iter_mut(){
        transform.translation.x += velocity.value.x; 
        transform.translation.y += velocity.value.y;
        transform.translation.z += velocity.value.z;
    }
}

