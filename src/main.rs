use bevy::prelude::*;
//component
#[derive(Component, Debug)]
struct Displacement {
    x: f32,
    y: f32
}
//component
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32
}

fn main() {
    //app
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (update_position, print_components))
        .add_systems(Startup, start_up)
        .run();

}
//system
fn start_up(mut commands: Commands) {

    commands.spawn((
        Velocity { x: 1f32, y: 0f32 },
        Displacement { x: 0f32, y: 0f32 }
    ));
}
//system
fn update_position(mut query: Query<(&Velocity, &mut Displacement)>) {
    for (velocity, mut displacement) in query.iter_mut() {
        displacement.x += velocity.x;
        displacement.y += velocity.y;
    }
}
//system
fn print_components(query: Query<(Entity, &Velocity, &Displacement)>) {
    for (entity, velocity, displacement) in query.iter() {
        info!{"Entity {:?} is at position {:?}, and is moving at a velocity {:?}", entity, displacement, velocity}
    }
}