use std::{any::TypeId, collections::HashMap};

use katabatic_scene::world::WorldHandle;
use katabatic_util::error::KResult;

use crate::plugin::Plugin;

pub struct App {
    world: WorldHandle,
    plugins: HashMap<TypeId, Box<dyn Plugin>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: WorldHandle::new(),
            plugins: HashMap::new(),
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

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> KResult<&mut Self> {
        plugin.build(self)?;

        self.plugins.insert(TypeId::of::<T>(), Box::new(plugin));

        Ok(self)
    }

    pub fn get_plugin<T: Plugin>(&self) -> Option<&T> {
        if let Some(plugin) = self.plugins.get(&TypeId::of::<T>()) {
            (**plugin).downcast_ref()
        } else {
            None
        }
    }

    pub fn run(&mut self) -> KResult<()> {
        let plugins = std::mem::take(&mut self.plugins);

        for plugin in plugins.values() {
            plugin.finish(self)?;
        }

        for plugin in plugins.values() {
            plugin.cleanup(self)?;
        }

        Ok(())
    }
}
