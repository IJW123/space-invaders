use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
            title: "Rust Invaders!".into(),
            resolution: (598., 676.).into(),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_startup_system(setup_system)
    .run();
}
  
fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}