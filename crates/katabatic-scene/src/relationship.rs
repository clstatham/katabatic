use std::{any::Any, fmt::Debug};

use crate::node::Node;

pub trait Relationship: Any + Debug {
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: Sized,
    {
        self
    }
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>
    where
        Self: Sized,
    {
        self
    }
}

#[derive(Debug)]
pub struct RelationshipConnection {
    pub from: Node,
    pub to: Node,
    pub weight: Box<dyn Relationship>,
}

impl RelationshipConnection {
    pub fn new<T: Relationship>(from: Node, to: Node, weight: T) -> Self {
        Self {
            from,
            to,
            weight: Box::new(weight),
        }
    }
}
