use std::any::TypeId;

use katabatic_util::lock::{MapRead, MapWrite};

use crate::{component::Component, entity::Entity, world::World};

pub struct Query<'a, T: Component> {
    pub(crate) world: &'a World,
    pub(crate) entities: Vec<Entity>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(world: &'a World) -> Self {
        // gather the entities that contain the component
        let storage = world.storage();
        let entities = storage
            .entity_iter()
            .filter(|entity| storage.has_component_by_type_id(*entity, TypeId::of::<T>()))
            .collect::<Vec<_>>();

        Self {
            world,
            entities,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn entity_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = MapRead<'_, T>> + '_ {
        self.entities
            .iter()
            .filter_map(move |entity| self.world.get_component::<T>(*entity))
    }

    pub fn iter_mut(&self) -> impl Iterator<Item = MapWrite<'_, T>> + '_ {
        self.entities
            .iter()
            .filter_map(move |entity| self.world.get_component_mut::<T>(*entity))
    }

    pub fn get(&self, entity: Entity) -> Option<MapRead<'_, T>> {
        if self.entities.contains(&entity) {
            self.world.get_component::<T>(entity)
        } else {
            None
        }
    }

    pub fn get_mut(&self, entity: Entity) -> Option<MapWrite<'_, T>> {
        if self.entities.contains(&entity) {
            self.world.get_component_mut::<T>(entity)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[test]
    fn query() {
        let mut world = World::new();
        let entity1 = world.create_entity();
        let entity2 = world.create_entity();
        let _entity3 = world.create_entity();

        world.insert_component(entity1, Position { x: 1.0, y: 2.0 });
        world.insert_component(entity2, Position { x: 3.0, y: 4.0 });
        world.insert_component(entity2, Velocity { dx: 5.0, dy: 6.0 });

        let query = world.query::<Position>();

        let positions: Vec<_> = query.iter().collect();
        assert_eq!(positions.len(), 2);
        assert_eq!(query.get(entity1).unwrap().x, 1.0);
        assert_eq!(query.get(entity1).unwrap().y, 2.0);
        assert_eq!(query.get(entity2).unwrap().x, 3.0);
        assert_eq!(query.get(entity2).unwrap().y, 4.0);

        let query = world.query::<Velocity>();

        let velocities: Vec<_> = query.iter().collect();
        assert_eq!(velocities.len(), 1);
        assert_eq!(query.get(entity2).unwrap().dx, 5.0);
        assert_eq!(query.get(entity2).unwrap().dy, 6.0);
    }
}
