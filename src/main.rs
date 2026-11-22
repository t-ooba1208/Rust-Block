use bevy::prelude::*;

#[derive(Component)]
struct Paddle;
const PADDLE_Y: f32 = -200.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0,20.0);
const COLOR_PADDLE: Color = Color::srgb(0.3,0.7,1.0);
const Z_BACKGROUNG: f32 = -1.0;
const Z_PADDLE:f32 = 1.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,setup)
    .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite{
            image: asset_server.load("images/sample.webp"),  //
            custom_size: Some(Vec2::new(1280.0,720.0)),
            ..default()
        },
        Transform{
            translation: Vec3::new(0.0,0.0,Z_BACKGROUNG),
            ..default()
        },
    ));
    commands.spawn((
        Sprite::from_color(
            COLOR_PADDLE,
            Vec2::ONE,
        ),
        Transform{
            translation: Vec3::new(0.0,PADDLE_Y,Z_PADDLE),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
    ));
}