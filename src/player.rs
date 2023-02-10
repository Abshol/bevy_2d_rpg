use bevy::{prelude::*, sprite::{SpriteSheetBundle, TextureAtlasSprite, collide_aabb::collide}};
use bevy_inspector_egui::Inspectable;
use crate::{TILE_SIZE, ascii::{spawn_ascii_sprite, AsciiSheet}, tilemap::{TileCollider, EncounterSpawner}, GameState};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}


impl Plugin for PlayerPlugin {
    fn build (&self, app:&mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(GameState::Overworld).with_system(show_player)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Overworld).with_system(hide_player)
        )
        .add_system_set(SystemSet::on_update(GameState::Overworld)
            .with_system(player_encounter_checking.after("movement"))
            .with_system(camera_follow.after("movement"))
            .with_system(player_movement.label("movement")),
        )
        .add_startup_system(spawn_player);
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis = false;

    if let Ok(children) = children
}

fn camera_follow (
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>,Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation)) 
    {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation)) 
    {
        transform.translation = target;
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_transform: Vec3
) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        wall_transform,
        Vec2::splat(TILE_SIZE)
    );
    return collision.is_some()
}

fn player_encounter_checking(
    player_query: Query<&Transform, With<Player>>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
) {
    let player_translation = player_query.single().translation;
    if encounter_query.iter().any(|&transform| wall_collision_check(player_translation, transform.translation)) {
        println!("Changing to combat");
        state.set(GameState::Combat).expect("Failed to change states");
    }    
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {

    /* Creates player */
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii, 
        1, 
        Color::rgb(0.3, 0.3, 0.9), 
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );
    
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 })
        .id();

    /* Creates background */
    let background = spawn_ascii_sprite(
        &mut commands, 
        &ascii, 
        0, 
        Color::rgb(0.5, 0.5, 0.5), 
        Vec3::new(0.0, 0.0, -1.0)
    );

    commands
        .entity(background)
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}