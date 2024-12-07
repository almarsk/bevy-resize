use crate::Player;
use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display);
        app.add_systems(Update, update_coordinate_display);
    }
}

#[derive(Component)]
pub struct CoordinateDisplay;

pub fn spawn_coordinate_display(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Player XY: [-, -]"),
        TextLayout::new_with_justify(JustifyText::Left),
        Transform::from_translation(Vec3::new(10., -10., 0.)),
        bevy::sprite::Anchor::TopLeft,
        CoordinateDisplay {},
    ));
}

pub fn update_coordinate_display(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<(&mut Text2d, &CoordinateDisplay)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut text, _)) = text_query.get_single_mut() {
            text.0 = format!(
                "Player XY: [{:.0}, {:.0}]",
                player_transform.translation.x, player_transform.translation.y
            );
        }
    }
}
