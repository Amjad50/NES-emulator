mod apu2a03;
mod apu2a03_registers;
mod channels;
mod envelope;
mod length_counter;
mod sequencer;
mod tone_source;

pub use apu2a03::APU2A03;

// for performance
pub const SAMPLE_RATE: u32 = 22050;
