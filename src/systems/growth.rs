use crate::domain::world::World;
use crate::core::units::Weight;

pub struct GrowthSystem;

impl GrowthSystem {
    pub fn apply_one_day(world: &mut World) {
        for prey in &mut world.prey {
            let species = &world.species[prey.species_idx];
            let weight = &species.gompertz.weight_at_age(prey.age_days.0 as f64);
            prey.weight = Weight(weight.max(prey.weight.0));        
            
        }
    }
}