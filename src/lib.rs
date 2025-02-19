pub mod player;

use libspa::prelude::*;
use pipewire::prelude::*;

pub use player::Sock;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pipewire error occurred")]
    PipewireError(#[from] pipewire::Error)
}