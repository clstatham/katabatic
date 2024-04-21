use std::{any::TypeId, collections::HashMap};

use katabatic_ecs::world::World;
use katabatic_scene::scene::Scene;
use katabatic_util::{error::KResult, lock::SharedLock};

use crate::{
    plugin::Plugin,
    runner::{DefaultRunner, Runner},
};

pub struct App {
    world: SharedLock<World>,
    root_scene: SharedLock<Scene>,
    plugins: HashMap<TypeId, Box<dyn Plugin>>,
    runner: Option<Box<dyn Runner>>,
}

impl Default for App {
    fn default() -> Self {
        let world = SharedLock::new(World::new());
        let root_scene = Scene::new(world.clone());
        Self {
            world,
            root_scene: SharedLock::new(root_scene),
            plugins: HashMap::new(),
            runner: Some(Box::<DefaultRunner>::default()),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn world(&self) -> &SharedLock<World> {
        &self.world
    }

    pub fn root_scene(&self) -> &SharedLock<Scene> {
        &self.root_scene
    }

    pub fn add_plugin<T: Plugin>(&mut self, mut plugin: T) -> KResult<&mut Self> {
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

    pub fn set_runner<T>(&mut self, runner: T)
    where
        T: Runner + 'static,
    {
        self.runner = Some(Box::new(runner));
    }

    pub fn run(&mut self) -> KResult<()> {
        let mut runner = self
            .runner
            .take()
            .expect("App:run(): Runner not initialized");

        runner.run(self)?;

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
