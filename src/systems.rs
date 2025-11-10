use bevy::{app::AppExit, prelude::*};
use std::collections::HashSet;

use crate::components::{GameOverButton, GameOverUI, Health, HealthBarFill, Player, Rock};
use crate::states::GameState;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player {
            colliding_rocks: HashSet::new(),
        },
        Health {
            current: 100.0,
            max: 100.0,
        },
        Sprite {
            image: asset_server.load("character.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::srgb(0.5, 0.8, 1.0),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let rock_positions = [
        Vec2::new(150.0, 100.0),
        Vec2::new(-200.0, -50.0),
        Vec2::new(100.0, -150.0),
        Vec2::new(-150.0, 120.0),
    ];

    for pos in rock_positions.iter() {
        commands.spawn((
            Rock,
            Sprite {
                color: Color::srgb(0.5, 0.3, 0.2),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::from(Color::WHITE),
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|parent| {
            parent.spawn((
                HealthBarFill,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
            ));
        });
}

pub fn character_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        let speed = 200.0;

        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += speed * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * time.delta_secs();
        }
    }
}

pub fn check_rock_collision(
    mut player_query: Query<(&Transform, &mut Health, &mut Player)>,
    rock_query: Query<(Entity, &Transform), With<Rock>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let collision_distance = 75.0;
    let damage_per_hit = 20.0;

    for (player_transform, mut health, mut player) in player_query.iter_mut() {
        let mut currently_colliding = HashSet::new();

        for (rock_entity, rock_transform) in rock_query.iter() {
            let distance = player_transform
                .translation
                .distance(rock_transform.translation);

            if distance < collision_distance {
                currently_colliding.insert(rock_entity);

                if !player.colliding_rocks.contains(&rock_entity) {
                    health.current -= damage_per_hit;
                    health.current = health.current.max(0.0);

                    if health.current <= 0.0 {
                        next_state.set(GameState::GameOver);
                    }
                }
            }
        }

        player.colliding_rocks = currently_colliding;
    }
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Node, With<HealthBarFill>>,
) {
    if let Ok(health) = player_query.single() {
        if let Ok(mut style) = health_bar_query.single_mut() {
            let health_percentage = (health.current / health.max * 100.0).max(0.0);
            style.width = Val::Percent(health_percentage);
        }
    }
}

pub fn show_game_over(mut commands: Commands) {
    commands
        .spawn((
            GameOverUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Game Over"),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.2, 0.2)),
                    ));

                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    GameOverButton::Restart,
                                    Button,
                                    Node {
                                        width: Val::Px(200.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Restart"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            parent
                                .spawn((
                                    GameOverButton::Exit,
                                    Button,
                                    Node {
                                        width: Val::Px(200.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Exit"),
                                        TextFont {
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        });
                });
        });
}

pub fn handle_game_over_buttons(
    mut interaction_query: Query<
        (&Interaction, &GameOverButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    ui_query: Query<Entity, With<GameOverUI>>,
    player_query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button {
                GameOverButton::Restart => {
                    for entity in ui_query.iter() {
                        commands.entity(entity).despawn();
                    }

                    for entity in player_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    commands.spawn((
                        Player {
                            colliding_rocks: HashSet::new(),
                        },
                        Health {
                            current: 100.0,
                            max: 100.0,
                        },
                        Sprite {
                            image: asset_server.load("character.png"),
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            color: Color::srgb(0.5, 0.8, 1.0),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));

                    next_state.set(GameState::Playing);
                }
                GameOverButton::Exit => {
                    exit.write(AppExit::Success);
                }
            },
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}
