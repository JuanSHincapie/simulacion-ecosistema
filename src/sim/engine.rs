use crate::domain::predator::Predator;
use crate::domain::prey::SpeciesProfile;
use crate::domain::world::World;
use crate::systems::aging::AgingSystem;
use crate::io::config::{AllConfig, load_all};


pub struct Engine {
    pub world: World,
    pub cfg: AllConfig,
}

impl Engine {
    pub fn new () -> anyhow::Result<Self> {
        let cfg = load_all()?;
        let species: Vec<SpeciesProfile> = cfg.species.iter().map(|s| {SpeciesProfile {
            id: s.id.clone(),
            display_name: s.display_name.clone(),
            maturity_days: s.maturity.age_days,
        }}).collect();

        let predator = Predator {
            reserve: cfg.predator.reserve_opt,
            reserve_min: cfg.predator.reserve_min,
            reserve_opt: cfg.predator.reserve_opt,
        };

        let world = World::new(species, predator, &cfg.world.spawn);
        Ok(Self {world, cfg})
    }

    pub fn run_steps(&mut self, steps: u32) {
        let _ = steps;
    }

    pub fn on_end_of_day(&mut self) {
        AgingSystem::apply_one_day(&mut self.world);
    }
}

