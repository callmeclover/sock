pub const DEFAULT_RATE: u32 = 44100;
pub const DEFAULT_CHANNELS: u32 = 2;
pub const DEFAULT_VOLUME: f64 = 0.7;
pub const PI_2: f64 = std::f64::consts::PI + std::f64::consts::PI;
pub const CHAN_SIZE: usize = std::mem::size_of::<i16>();

pub struct Player {

}

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}

impl Player {
    pub fn new() -> Self {
        todo!()
    }
}