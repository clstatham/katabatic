use std::any::Any;

use generational_arena::Index;
use petgraph::prelude::*;

use crate::{id::Id, world::WorldHandle};

#[derive(Debug)]
pub struct Scene {
    pub(crate) world: WorldHandle,
    pub(crate) graph: StableDiGraph<Index, ()>,
}

impl Scene {
    pub fn new(world: WorldHandle) -> Self {
        Self {
            world,
            graph: StableDiGraph::new(),
        }
    }

    pub fn add_data<T: Any>(&mut self, data: T) -> Id {
        let node_id = self.world.insert_data(data);

        let scene_index = self.graph.add_node(node_id);

        Id {
            node_id,
            scene_index,
        }
    }

    pub fn add_scene(&mut self) -> Id {
        let node_id = self.world.create_scene();

        let scene_index = self.graph.add_node(node_id);

        Id {
            node_id,
            scene_index,
        }
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(a, b, ())
    }
}
