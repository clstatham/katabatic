use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{data::Data, id::Id, world::WorldHandle};

#[repr(C)]
pub struct Node {
    data: Data,
    id: Id,
    world: WorldHandle,
    parent: Option<Id>,
    children: Vec<Id>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("data", &self.data)
            .finish()
    }
}

impl Deref for Node {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
