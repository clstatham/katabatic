use std::sync::Arc;

use generational_arena::Index;
use petgraph::prelude::*;

use crate::{id::Id, node::Node, world::World};

#[derive(Debug)]
pub struct Scene {
    pub(crate) world: Arc<World>,
    pub(crate) graph: StableDiGraph<Index, ()>,
}

impl Scene {
    pub fn new(world: Arc<World>) -> Self {
        Self {
            world,
            graph: StableGraph::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Id {
        let node_id = self.world.insert_node(node);

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
