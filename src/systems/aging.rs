use crate::domain::world::World;

pub struct AgingSystem;

impl AgingSystem{
    pub fn apply_one_day(world: &mut World) {
        for p in &mut world.prey {
            p.age_one_day();
        }
    }
}