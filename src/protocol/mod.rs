//! Protocol layer for packet encoding and decoding

mod packet;
mod sequence;

/// Maximum command length in bytes
pub const MAX_CMD_LEN: usize = 64;
