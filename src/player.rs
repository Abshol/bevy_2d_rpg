use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    combat::CombatStats,
    fadeout::create_fadeout,
    graphics::{CharacterSheet, FacingDirection, FrameAnimation, PlayerGraphics},
    tilemap::{EncounterSpawner, TileCollider},
    GameState, MainCamera, TILE_SIZE,
};
use bevy::{
    prelude::*,
    render::camera::Camera2d,
    sprite::{collide_aabb::collide, SpriteSheetBundle, TextureAtlasSprite},
};
use bevy_inspector_egui::Inspectable;
use rand::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    pub active: bool,
    just_moved: bool,
    pub exp: usize,
}

impl Player {
    pub fn give_exp(&mut self, exp: usize, stats: &mut CombatStats) -> bool {
        self.exp += exp;
        if self.exp >= 50 {
            stats.health += 2;
            stats.max_health += 2;
            stats.attack += 2;
            stats.defense += 2;
            stats.health += 2;
            self.exp -= 50;
            return true;
        }
        false
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(show_player))
            .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(hide_player))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(player_encounter_checking.after(player_movement))
                    .with_system(camera_follow.after(player_movement))
                    .with_system(player_movement),
            )
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(spawn_player));
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_player(
    mut player_query: Query<(&mut Player, &mut Visibility)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let (mut player, mut player_vis) = player_query.single_mut();
    player.active = true;
    player_vis.is_visible = true;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform, &mut PlayerGraphics)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform, mut graphics) = player_query.single_mut();
    player.just_moved = false;

    if !player.active {
        return;
    }

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

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if y_delta != 0.0 {
            player.just_moved = true;
            if y_delta > 0.0 {
                graphics.facing = FacingDirection::Up;
            } else {
                graphics.facing = FacingDirection::Down;
            }
        }
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if x_delta != 0.0 {
            player.just_moved = true;
            if x_delta > 0.0 {
                graphics.facing = FacingDirection::Right;
            } else {
                graphics.facing = FacingDirection::Left;
            }
        }
        transform.translation = target;
    }
}

fn wall_collision_check(target_player_pos: Vec3, wall_transform: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        wall_transform,
        Vec2::splat(TILE_SIZE),
    );
    return collision.is_some();
}

fn player_encounter_checking(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
    ascii: Res<AsciiSheet>,
    mut time: Res<Time>,
) {
    let (mut player, mut encounter_tracker, player_transform) = player_query.single_mut();
    let player_translation = player_transform.translation;
    if player.just_moved
        && encounter_query
            .iter()
            .any(|&transform| wall_collision_check(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());
        if encounter_tracker.timer.just_finished() {
            player.active = false;
            encounter_tracker.timer = Timer::from_seconds(thread_rng().gen_range(1.0..=6.0), true);
            create_fadeout(&mut commands, Some(GameState::Combat), &ascii);
        }
    }
}

fn spawn_player(mut commands: Commands, characters: Res<CharacterSheet>) {
    // /* Creates player ascii sprite */
    // let player = spawn_ascii_sprite(
    //     &mut commands,
    //     &ascii,
    //     1,
    //     Color::rgb(0.3, 0.3, 0.9),
    //     Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    //     Vec3::splat(1.0)
    // );

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: characters.player_down[0],
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
            texture_atlas: characters.handle.clone(),
            ..default()
        })
        .insert(FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.player_down.to_vec(),
            current_frame: 0,
        })
        .insert(PlayerGraphics {
            facing: FacingDirection::Down,
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 3.0,
            active: true,
            just_moved: false,
            exp: 0,
        })
        .insert(CombatStats {
            health: 10,
            max_health: 10,
            attack: 2,
            defense: 1,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(thread_rng().gen_range(0.0..=6.0), true),
        });

    // /* Creates background ascii OLD */
    // let background = spawn_ascii_sprite(
    //     &mut commands,
    //     &ascii,
    //     0,
    //     Color::rgb(0.5, 0.5, 0.5),
    //     Vec3::new(0.0, 0.0, -1.0),
    //     Vec3::splat(1.0)
    // );
    // commands
    //     .entity(background)
    //     .insert(Name::new("Background"))
    //     .id();
    // commands.entity(player).push_children(&[background]);
}
