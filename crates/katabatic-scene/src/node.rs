use std::{fmt::Debug, ops::Deref};

use katabatic_util::lock::SharedLock;

use crate::{data::Data, id::Id, world::WorldHandle};

#[derive(Clone)]
#[repr(C)]
pub struct Node {
    data: SharedLock<Data>,
    id: Id,
    world: WorldHandle,
}

impl Node {
    pub fn data(&self) -> &SharedLock<Data> {
        &self.data
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn world(&self) -> &WorldHandle {
        &self.world
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("data", &self.data)
            .finish()
    }
}

impl Deref for Node {
    type Target = SharedLock<Data>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
