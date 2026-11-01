use bevy::prelude::*;
use crate::core::GameState;
use crate::player::Player;
use crate::zombies::Zombie;

pub fn handle_fire(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut zombie_query: Query<(Entity, &mut Zombie, &Transform)>,
    mut state: ResMut<GameState>,
) {
    let Ok((mut player, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    player.fire_cooldown = (player.fire_cooldown - time.delta_secs()).max(0.0);

    if keys.just_pressed(KeyCode::KeyR) {
        player.ammo = 12;
    }

    let fire_triggered = mouse.just_pressed(MouseButton::Left) || keys.just_pressed(KeyCode::Space);
    if !fire_triggered || player.fire_cooldown > 0.0 || player.ammo <= 0 {
        return;
    }

    player.ammo -= 1;
    player.fire_cooldown = 0.25;
    let direction = (player_transform.rotation * Vec3::NEG_Z).normalize();
    let mut target: Option<(Entity, f32)> = None;

    for (entity, zombie, transform) in zombie_query.iter() {
        let to_target = transform.translation - player_transform.translation;
        let dist = to_target.length();
        if dist < 30.0 && direction.dot(to_target.normalize_or_zero()) > 0.85 {
            if let Some((_, closest)) = target {
                if dist < closest {
                    target = Some((entity, dist));
                }
            } else {
                target = Some((entity, dist));
            }
        }
    }

    if let Some((entity, _)) = target {
        let mut dead = Vec::new();
        for (zombie_entity, mut zombie, transform) in zombie_query.iter_mut() {
            if zombie_entity == entity {
                zombie.hp -= 55.0;
                if zombie.hp <= 0.0 {
                    dead.push(zombie_entity);
                    state.kills += 1;
                    state.score += 100;
                }
                break;
            }
        }
        for entity_to_remove in dead {
            commands.entity(entity_to_remove).despawn();
        }
    }
}
