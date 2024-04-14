use std::{any::Any, fmt::Debug};

use generational_arena::{Arena, Index};
use katabatic_util::lock::{Read, SharedLock, Write};

use crate::{data::Data, id::Id, node::Node, scene::Scene};

pub struct World {
    nodes: Arena<Node>,
    root: Index,
}

impl Default for World {
    fn default() -> Self {
        Self {
            nodes: Arena::new(),
            root: Index::from_raw_parts(usize::MAX, u64::MAX),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::ops::Index<Id> for World {
    type Output = Node;

    fn index(&self, index: Id) -> &Self::Output {
        &self.nodes[index.node_id]
    }
}

impl std::ops::IndexMut<Id> for World {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        &mut self.nodes[index.node_id]
    }
}

#[derive(Clone)]
pub struct WorldHandle {
    world: SharedLock<World>,
}

impl Default for WorldHandle {
    fn default() -> Self {
        Self::empty()
    }
}

impl WorldHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        let this = Self {
            world: SharedLock::new(World::new()),
        };

        let root_id = this.create_scene();

        this.write().root = root_id;

        this
    }

    pub fn read(&self) -> Read<World> {
        self.world.read()
    }

    pub fn write(&self) -> Write<World> {
        self.world.write()
    }

    pub fn insert_data<T: Any>(&self, data: T) -> Index {
        self.write().nodes.insert(Node::Data(Data::new(data)))
    }

    pub fn create_scene(&self) -> Index {
        let scene = Scene::new(self.clone());
        self.write().nodes.insert(Node::Scene(scene))
    }

    pub fn with_root<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Scene) -> R,
    {
        let root_id = self.read().root;
        let root = &self.read().nodes[root_id];
        match root {
            Node::Scene(root) => f(root),
            _ => unreachable!("WorldHandle::with_root(): Root node is not a Scene"),
        }
    }

    pub fn with_root_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Scene) -> R,
    {
        let root_id = self.read().root;
        let root = &mut self.write().nodes[root_id];
        match root {
            Node::Scene(root) => f(root),
            _ => unreachable!("WorldHandle::with_root_mut(): Root node is not a Scene"),
        }
    }
}

impl Debug for WorldHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WorldHandle")
    }
}
