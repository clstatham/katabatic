use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use generational_arena::{Arena, Index};
use katabatic_util::lock::SharedLock;

use crate::{node::Node, scene::Scene};

#[derive(Clone)]
pub struct World {
    nodes: SharedLock<Arena<SharedLock<Node>>>,
    root: OnceLock<Index>,
}

impl World {
    pub fn new() -> Arc<Self> {
        #[allow(clippy::arc_with_non_send_sync)]
        let this = Arc::new(Self {
            nodes: SharedLock::new(Arena::new()),
            root: OnceLock::new(),
        });

        let root = this
            .nodes
            .write()
            .insert(SharedLock::new(Node::Scene(Scene::new(this.clone()))));

        this.root.get_or_init(|| root);

        this
    }

    pub fn root(&self) -> Index {
        *self.root.get().unwrap()
    }

    pub fn insert_node(&self, node: Node) -> Index {
        self.nodes.write().insert(SharedLock::new(node))
    }

    pub fn remove_node(&self, id: Index) -> Option<SharedLock<Node>> {
        self.nodes.write().remove(id)
    }

    pub fn with_root<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Scene) -> R,
    {
        let root = self.nodes.read()[self.root()].clone();
        let root = root.read();
        match &*root {
            Node::Scene(root) => f(root),
            _ => unreachable!("WorldHandle::with_root(): Root node is not a Scene"),
        }
    }

    pub fn with_root_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Scene) -> R,
    {
        let root = self.nodes.read()[self.root()].clone();
        let mut root = root.write();
        match &mut *root {
            Node::Scene(root) => f(root),
            _ => unreachable!("WorldHandle::with_root_mut(): Root node is not a Scene"),
        }
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WorldHandle")
    }
}
