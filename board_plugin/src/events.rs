use crate::components::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct BoardCompletedEvent;

#[derive(Debug, Copy, Clone)]
pub struct BoardExplosionEvent;

#[derive(Debug, Copy, Clone)]
pub struct TileMarkEvent(pub Coordinates);
