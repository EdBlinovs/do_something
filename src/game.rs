use std::ops::{Add, Mul};

use bevy::prelude::*;
use rand::Rng;
use crate::app_state::{self, AppState};
use crate::generic_ui::{spawn_button, ButtonActionType};
use crate::{startup, util as util};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Score(0, "".into()))
        .insert_resource(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(OnEnter(AppState::Game), (setup_game, spawn_baby_fire).chain())
        .add_systems(OnExit(AppState::Game), util::cleanup_system::<GameEntity>)
        .add_systems(Update, (
            update_good_thing, 
            swing_mallet,
            spin_carousel,
            good_thing_does_not_touch_bad_thing, score_ticker
        )
            .chain()
            .run_if(in_state(AppState::Game))
        );

    }
}

#[derive(Component)]
struct GameEntity;


#[derive(Component)]
pub struct GoodThing{
    pub situation: Situation
}

fn get_message(situation: &Situation) -> String {
    match situation {
        Situation::Baby => return "your negligent inaction led to withered death".to_string(),
        Situation::Kitten => return "their soft paws were torn to shreds by rusty nails. weeping, they wondered why their protector had abandoned them".to_string(),
        Situation::Sloth => return "the baby sloth just wanted to eat some leaves - it did not choose to be put in the hands of an incompetent carer".to_string(),
    }
}

#[derive(Component)]
struct BadThing;

#[derive(Component)]
struct CircleCollider{
    radius: f32
}

#[derive(Resource)]
pub struct Score(pub i32, pub String);


#[derive(Resource)]
pub struct Difficulty{
    baby: f32,
    sloth: f32,
    kitten: f32
}

fn setup_game(mut commands: Commands, mut background_colour: ResMut<ClearColor>) {
    background_colour.0 = startup::CLEAR_COLOUR;

    commands.insert_resource(Score(0, "".to_string()));

    commands.insert_resource(Difficulty {
        baby: 1.,
        sloth: 0.,
        kitten: 0.
    });
}



pub enum Situation {
    Baby,
    Kitten,
    Sloth,
}

fn spawn_baby_fire(mut commands: Commands, asset_server: Res<AssetServer>){
    let node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    commands
        .spawn((node, GameEntity))
        .with_children(|parent| {
            spawn_button(parent, &asset_server, "end baby", ButtonActionType::EndGame("the baby explored, naive and innocent, its newfound territory. you actively steered creation to its death".to_string()));

            spawn_button(parent, &asset_server, "pull baby", ButtonActionType::ProlongInevitable(Situation::Baby));
        });

    commands.spawn((
        util::image(Vec2::new(-450., 220.), "baby.png".into(), &asset_server),
        GameEntity,
        GoodThing { situation: Situation::Baby },
        CircleCollider{ radius:100. },
        AudioBundle {
            source: asset_server.load("baby.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            }
        }
    ));

    commands.spawn((
        util::image_lift(Vec2::new(350., 220.), "fire.png".into(), &asset_server),
        GameEntity,
        BadThing,
        CircleCollider{ radius:100. },
        AudioBundle {
            source: asset_server.load("fire.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            }
        }
    ));
}

#[derive(Component)]
pub struct Carousel(pub f32);

#[derive(Component)]
pub struct Nail;

fn spin_carousel
(
    mut query: Query<
    (&mut Transform, &Carousel),
    Without<Nail>
    >,
    mut query2: Query<
    (&mut Transform, &Carousel),
    With<Nail>
    >,
    time: Res<Time>,
    difficulty: Res<Difficulty>,
)
{
    for (mut transform, carousel) in &mut query {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2 + (carousel.0 * 0.1 * difficulty.kitten * time.delta_seconds());
        
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }

    for (mut transform, carousel) in &mut query2 {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2 + (carousel.0 * 0.1 * difficulty.kitten * time.delta_seconds());
        
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }
}

fn spawn_kitten_nail(commands: &mut Commands, asset_server: &Res<AssetServer>){
    commands.spawn((
        util::image_low(Vec2::new(316., -180.), "carousel.png".into(), &asset_server),
        GameEntity,
        Carousel(1.)
    )).with_children(|parent| {
        parent.spawn((
            util::image(Vec2::new(0., 125.), "bridge.png".into(), &asset_server),
        ));

        parent.spawn((
            util::image_rot(Vec2::new(-91., -72.), "bridge.png".into(), &asset_server, 2.3),
        ));

        parent.spawn((
            util::image_rot(Vec2::new(125./1.414, -125./1.414), "nails.png".into(), &asset_server, 2. * 6.28/3.),
            BadThing,
            Nail,
            CircleCollider { radius:50. }
        ));
    });

    commands.spawn((
        util::image(Vec2::new(316., 30.), "kitten.png".into(), &asset_server),
        GameEntity,
        GoodThing { situation: Situation::Kitten },
        CircleCollider{ radius:50. },
        AudioBundle {
            source: asset_server.load("kitten.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            }
        }
    ));

    let node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        },
        ..default()
    };

    commands
    .spawn((node, GameEntity))
    .with_children(|parent| {
        spawn_button(parent, &asset_server, "reverse world", ButtonActionType::ProlongInevitable(Situation::Kitten));
    });
}

fn spawn_sloth(commands: &mut Commands, asset_server: &Res<AssetServer>){
    commands.spawn((
        util::image(Vec2::new(-370., 420.), "sloth.png".into(), &asset_server),
        GameEntity,
        GoodThing { situation: Situation::Sloth },
        CircleCollider{ radius:50. },
        AudioBundle {
            source: asset_server.load("sloth.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            }
        }
    ));
}

#[derive(Component)]
struct Mallet;

fn spawn_sloth_mallet(mut commands: &mut Commands, asset_server: &Res<AssetServer>){

    let node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::End,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    spawn_sloth(&mut commands, &asset_server);

    commands
    .spawn((node, GameEntity))
    .with_children(|parent| {
        spawn_button(parent, &asset_server, "strangle sloth", ButtonActionType::EndGame("the baby sloth, choking for air, sheds a tear and squeaks out cries for the safety of its mother. it does not come.".to_string()));

        spawn_button(parent, &asset_server, "push sloth", ButtonActionType::ProlongInevitable(Situation::Sloth));
    });

    commands.spawn((
        util::image_low(Vec2::new(-387.2, -283.5), "hit_area.png".into(), &asset_server),
        GameEntity,
    ));

    commands.spawn((
        util::image_lift(Vec2::new(-320., -240.), "mallet.png".into(), &asset_server),
        GameEntity,
        Mallet
    ));
}

#[derive(Resource)]
struct ScoreTimer(Timer);

fn score_ticker(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut timer: ResMut<ScoreTimer>,
    time: Res<Time>,
    mut difficulty: ResMut<Difficulty>,
    asset_server: Res<AssetServer>
){
    if timer.0.tick(time.delta()).just_finished() {
        score.0 += 1;
        let num = rand::thread_rng().gen_range(0..4);

        if num == 0 {
            difficulty.baby += 0.1;
        }

        if num == 1 {
            difficulty.sloth += 0.1;

            if difficulty.sloth == 0.5 {
                spawn_sloth_mallet(&mut commands, &asset_server);
            }

            if difficulty.sloth == 1.5 {
                spawn_sloth(&mut commands, &asset_server);
            }
        }

        if num == 2 {
            difficulty.kitten += 0.1;

            if difficulty.kitten == 2.0 {
                spawn_kitten_nail(&mut commands, &asset_server);
            }
        }
    }
}

fn swing_mallet(
    mut query: Query<
    &mut Transform,
    With<Mallet>
    >,
    mut commands: Commands,
    mut mallet_down: Query<
    Entity,
    (With<Mallet>, With<CircleCollider>, With<BadThing>)
    >,
    time: Res<Time>,
    asset_server: Res<AssetServer>
){
    let angle = (1.7)*(1. - (time.elapsed_seconds() * 2.094).sin().abs());
    for mut transform in &mut query {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);

        if angle > 1.2 {
            if mallet_down.is_empty() == true {
                commands.spawn((
                    Transform {
                        translation: Vec3::new(-387.2, -283.5, 0.),
                        ..default()
                    },
                    GameEntity,
                    BadThing,
                    Mallet,
                    CircleCollider { radius:50. },
                    AudioBundle {
                        source: asset_server.load("mallet.wav"),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            ..default()
                        }
                    }
                ));
            }
        }
        else {
            for hit in &mut mallet_down {
                commands.entity(hit).despawn_recursive();
            }
        }

    }
}

fn update_good_thing(
    mut query: Query<
    (&GoodThing, &mut Transform)
    >, 
    time: Res<Time>,
    difficulty: Res<Difficulty>,
){
    for (good_thing, mut transform) in &mut query {
        match good_thing.situation {
            Situation::Baby => {
                transform.translation.x += difficulty.baby * 10. * time.delta_seconds();
            },
            Situation::Kitten => {

            },
            Situation::Sloth => {
                transform.translation.y -= 7. * time.delta_seconds();

                if transform.translation.y < -420. {
                    transform.translation.y = 420.;
                }
            },
        }
    }
}

fn good_thing_does_not_touch_bad_thing(
    mut good_things: Query<
    (&GoodThing, &CircleCollider, &Transform)
    >, 
    mut bad_things: Query<
    (&BadThing, &CircleCollider, &Transform, Option<&Nail>)
    >,
    mut score: ResMut<Score>,
    mut current_game_state: ResMut<NextState<app_state::AppState>>,
    world_query: Query<(&Transform, &Carousel)>
){
    for (good_thing, good_circle, good_transform) in &mut good_things {
        for (_, bad_circle, bad_transform, maybe_nail) in &mut bad_things {
            let good_pos = good_transform.translation.xy();
            let mut bad_pos = bad_transform.translation.xy();

            match maybe_nail {
                Some(_) => {
                    for (transform, _) in &world_query {
                        let angle = transform.rotation;

                        let direction_vector = angle.mul_vec3(bad_transform.translation).xy();

                        bad_pos = Vec2::new(316., -180.).add(direction_vector);
                    }
                }
                None => {}
            }

            let distance2 = good_pos.distance_squared(bad_pos);
        
            if distance2 <= (good_circle.radius + bad_circle.radius)*(good_circle.radius + bad_circle.radius) {
                current_game_state.set(AppState::GameOver);
                score.1 = get_message(&good_thing.situation);
            }
        }
    }


}