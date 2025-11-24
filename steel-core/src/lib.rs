//! # Steel Core
//!
//! The core library for the Steel Minecraft server. Handles everything related to the PLAY state.

use crate::chunk::chunk_map::ChunkMap;

pub mod chunk;
pub mod config;
pub mod player;
pub mod server;
pub mod world;

/// The root of all worlds.
pub struct Level {
    /// A map of all the chunks in the level.
    pub chunks: ChunkMap,
}

impl Level {
    /// Creates a new level.
    #[must_use]
    pub fn create() -> Self {
        Self {
            chunks: ChunkMap::new(),
        }
    }
}
