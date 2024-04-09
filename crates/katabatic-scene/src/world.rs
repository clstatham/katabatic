use std::ops::{Index, IndexMut};

use generational_arena::Arena;
use katabatic_util::lock::SharedLock;

use crate::{id::Id, node::Node};

pub type WorldHandle = SharedLock<World>;

pub struct World {
    nodes: Arena<Node>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            nodes: Arena::new(),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Index<Id> for World {
    type Output = Node;

    fn index(&self, index: Id) -> &Self::Output {
        &self.nodes[index.0]
    }
}

impl IndexMut<Id> for World {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        &mut self.nodes[index.0]
    }
}
