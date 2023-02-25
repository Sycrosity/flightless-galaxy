// #![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod actions;
pub mod components;
pub mod error;
pub mod helpers;
pub mod inputs;
pub mod settings;
pub mod state;
pub mod systems;

pub mod prelude {

    pub use crate::actions::*;
    pub use crate::components::*;
    pub use crate::error::*;
    pub use crate::helpers::prelude::*;
    pub use crate::inputs::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::state::*;

    pub use core::f32::consts::*;
}
