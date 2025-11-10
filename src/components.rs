use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Player {
    pub colliding_rocks: HashSet<Entity>,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Rock;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub enum GameOverButton {
    Restart,
    Exit,
}
