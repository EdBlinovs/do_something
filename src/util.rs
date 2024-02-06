use bevy::prelude::*;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn image(position: Vec2, name: String, asset_server: &Res<AssetServer>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.8)),
        texture: asset_server.load(name),
        ..default()
    }
}

pub fn image_rot(position: Vec2, name: String, asset_server: &Res<AssetServer>, rot: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(position.x, position.y, 0.8),
            rotation: Quat:: from_axis_angle(Vec3::Z, rot),
            ..default()
        },
        texture: asset_server.load(name),
        ..default()
    }
}

pub fn image_lift(position: Vec2, name: String, asset_server: &Res<AssetServer>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.)),
        texture: asset_server.load(name),
        ..default()
    }
}

pub fn image_low(position: Vec2, name: String, asset_server: &Res<AssetServer>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.5)),
        texture: asset_server.load(name),
        ..default()
    }
}