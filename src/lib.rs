#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pipewire error occurred")]
    PipewireError(#[from] pipewire::Error)
}