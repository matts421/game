use bevy::prelude::*;

pub struct PrintPlugin;
impl Plugin for PrintPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Update, print_position);
    }
}



fn print_position(query: Query<(Entity, &Transform)>){
    for (entity, transform) in query.iter(){
        info!("Entity {:?} is at position (x, y, z) {:?}", entity, transform.translation);
    }
}