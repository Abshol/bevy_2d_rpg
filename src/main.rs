#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::{camera::ScalingMode, render_resource::Texture}, reflect::erased_serde::__private::serde::__private::de};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod player;
mod ascii;
mod tilemap;
mod combat;
mod fadeout;
mod audio;
mod graphics;
mod start_menu;
mod npc;
mod debug;

use player::PlayerPlugin;
use tilemap::TileMapPlugin;
use ascii::AsciiPlugin;
use combat::CombatPlugin;
use fadeout::FadeoutPlugin;
use audio::GameAudioPlugin;
use graphics::GraphicsPlugin;
use start_menu::MainMenuPlugin;
use npc::NpcPlugin;
use debug::DebugPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    StartMenu,
    Overworld,
    Combat,
}

fn main() {
    App::new()
    .add_state(GameState::StartMenu)
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
        width : 1600.0,
        height: 900.0,
        title: "Bevy 2D RPG test".to_string(),
        vsync: true,
        resizable: false,
        ..Default::default()
    })
    .add_startup_system(spawn_camera)
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(AsciiPlugin)
    .add_plugin(GameAudioPlugin)
    .add_plugin(TileMapPlugin)
    .add_plugin(CombatPlugin)
    .add_plugin(FadeoutPlugin)
    .add_plugin(GraphicsPlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(NpcPlugin)
    .add_plugin(DebugPlugin)
    .run();
}

#[derive(Component)]
pub struct MainCamera;
fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera).insert(MainCamera);
}
