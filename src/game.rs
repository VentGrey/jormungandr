use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle};


const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
