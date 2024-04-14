#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub node_id: generational_arena::Index,
    pub scene_index: petgraph::graph::NodeIndex,
}

impl Default for Id {
    fn default() -> Self {
        Self::placeholder()
    }
}

impl Id {
    pub fn placeholder() -> Self {
        Id {
            node_id: generational_arena::Index::from_raw_parts(usize::MAX, u64::MAX),
            scene_index: Default::default(),
        }
    }
}
