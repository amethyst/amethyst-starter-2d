use amethyst::{
    core::{SystemDesc, bundle::SystemBundle, Transform},
    derive::SystemDesc,
    ecs::{Write, World, Read, System, SystemData, DispatcherBuilder, WriteStorage, Entity, Entities, Join},
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
    Result, 
};

use crate::{
    components::{Map}
};

pub struct MapSystem;

impl<'s> System<'s> for MapSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Map>,
    );
    
    /// Should ONLY be called in a re-draw event of the map
    /// Resource room should be updated with the newest room
    fn run(&mut self, (mut transforms, mut sprite_renders, mut maps): Self::SystemData) {
        for (tr, spr, map) in (&mut transforms, &mut sprite_renders, &mut maps).join() {
            tr.move_down(1.0);
        }
    }

}
