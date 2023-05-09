use crate::GameAssets;
use crate::*;
use bevy::prelude::*;
use bevy::utils::FloatOrd;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub(crate) shooting_timer: Timer,
    pub(crate) bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    towers.for_each_mut(|(tower_entity, mut tower, transform)| {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            //let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                commands.entity(tower_entity).with_children(|commands| {
                    commands
                        .spawn(SceneBundle {
                            scene: bullet_assets.bullet_scene.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..Default::default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.5, TimerMode::Once),
                        })
                        .insert(Bullet {
                            direction,
                            speed: 2.5,
                        })
                        .insert(Name::new("Bullet"));
                });
            }
        }
    });
}
