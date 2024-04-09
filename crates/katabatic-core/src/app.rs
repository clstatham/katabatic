use katabatic_scene::world::{World, WorldHandle};
use katabatic_util::error::KResult;

use crate::plugin::Plugin;

pub struct App {
    world: WorldHandle,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: WorldHandle::new(World::new()),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn world(&self) -> &WorldHandle {
        &self.world
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) -> KResult<&mut Self> {
        plugin.build(self)?;

        Ok(self)
    }

    pub fn run(self) -> KResult<()> {
        Ok(())
    }
}
