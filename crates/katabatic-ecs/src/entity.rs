#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Entity {
    id: u32,
    generation: u32,
}

impl Entity {
    pub const fn new(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }

    pub const fn id(&self) -> u32 {
        self.id
    }

    pub const fn generation(&self) -> u32 {
        self.generation
    }
}
