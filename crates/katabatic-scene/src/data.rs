use std::{
    any::{type_name, Any, TypeId},
    borrow::Cow,
    mem::{align_of, size_of},
};

/// Type-erased pointer to a chunk of memory, alongside various type-related information.
#[derive(Debug)]
#[repr(C)]
pub struct Data {
    item: Box<dyn Any + Send + Sync>,
    type_id: TypeId,
    type_name: Cow<'static, str>,
    size: usize,
    align: usize,
}

impl Data {
    pub fn new<T: Any + Send + Sync>(item: T) -> Data {
        Data {
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>().into(),
            size: size_of::<T>(),
            align: align_of::<T>(),
            item: Box::new(item),
        }
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn align(&self) -> usize {
        self.align
    }

    pub fn item(&self) -> &(dyn Any + Send + Sync) {
        &*self.item
    }

    pub fn item_mut(&mut self) -> &mut (dyn Any + Send + Sync) {
        &mut *self.item
    }

    pub fn is<T: Any>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (*self.item).downcast_ref()
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (*self.item).downcast_mut()
    }
}
