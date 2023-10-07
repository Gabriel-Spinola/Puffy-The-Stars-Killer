mod enemy;
mod player;
mod systems;

use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("fab.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

/*

fn main()  {
  const a: i32 = 2;
  let b: u32 = 10;

  println!("test {} {}", a, b);


  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .add_plugins(PlayerPlugin)
    .add_systems(Update, exit_game)
    .run();
}

fn test_system(test: &str) {

  println!("Test: {}", test.to_string());
}



*/
