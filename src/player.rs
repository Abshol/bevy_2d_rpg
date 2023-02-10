use bevy::{prelude::*, sprite::{SpriteSheetBundle, TextureAtlasSprite}};
use crate::AsciiSheet;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;


impl Plugin for PlayerPlugin {
    fn build (&self, app:&mut App) {
        app
        .add_startup_system(spawn_player);
    }
}

fn player_movement(
    player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>
) {

}
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {

    /* Creates player */
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    let player = commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default() 
        },
        ..Default::default()
    }).insert(Name::new("Player"))
    .insert(Player)
    .id(); 
    
    /* Creates background */
    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(1.0));

    let background = commands.spawn_bundle(SpriteSheetBundle {
        sprite: background_sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, -1.0),
            ..Default::default() 
        },
        ..Default::default()
    }).insert(Name::new("Background")).id();

    commands.entity(player).push_children(&[background]);
}