use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Actionlike, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub enum GameAction {
    Primary,
    Secondary,
    Up,
    Down,
    Left,
    Right,
    Jump,
}

// impl GameAction {
//     // Lists like this can be very useful for quickly matching subsets of actions
//     pub const DIRECTIONS: [Self; 4] = [
//         Self::Up,
//         Self::Down,
//         Self::Left,
//         Self::Right,
//     ];

//     pub fn direction(self) -> Option<Direction> {
//         match self {
//             Self::Up => Some(Direction::NORTH),
//             Self::Down => Some(Direction::SOUTH),
//             Self::Left => Some(Direction::EAST),
//             Self::Right => Some(Direction::WEST),
//             _ => None,
//         }
//     }
// }
