use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GameInputs {
    Primary,
    Secondary,
    MoveLeft,
    MoveRight,
    Jump,
}
