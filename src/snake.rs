use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// Import local mods
use draw::draw_block;

const SNAKE_COLOUR: Color = [0.00, 0.80, 0.00, 1.0];

// Hora de los objetos

// Enumerado dirección
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Métodos para dirección
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

