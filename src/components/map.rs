use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage}
};

pub struct Map {
    pub name: String,
}

impl Component for Map {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
