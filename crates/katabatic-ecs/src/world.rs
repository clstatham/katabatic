use std::sync::atomic::{AtomicU32, Ordering};

use katabatic_util::lock::{Lock, MapRead, MapWrite};

use crate::{component::Component, entity::Entity, query::Query, storage::Storage};

#[derive(Default)]
pub struct World {
    next_entity: AtomicU32,
    free_entities: Lock<Vec<Entity>>,
    storage: Storage,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut Storage {
        &mut self.storage
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(entity) = self.free_entities.write().pop() {
            Entity::new(entity.id(), entity.generation() + 1)
        } else {
            let id = self.next_entity.fetch_add(1, Ordering::Relaxed);
            Entity::new(id, 0)
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.storage.remove_entity(entity);
        self.free_entities.write().push(entity);
    }

    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.storage.insert_component(entity, component)
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> Option<T> {
        self.storage.remove_component::<T>(entity)
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<MapRead<'_, T>> {
        self.storage.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: Component>(&self, entity: Entity) -> Option<MapWrite<'_, T>> {
        self.storage.get_component_mut::<T>(entity)
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        self.storage.has_component::<T>(entity)
    }

    pub fn query<T: Component>(&self) -> Query<T> {
        Query::new(self)
    }
}
