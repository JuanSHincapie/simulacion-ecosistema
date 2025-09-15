use super::prey::{Prey, SpeciesProfile};
use super::predator::Predator;
use std::collections::HashMap;

#[derive(Debug)]
pub struct World {
    pub species: Vec<SpeciesProfile>,
    pub species_map: HashMap<String, usize>,
    pub prey: Vec<Prey>,
    pub predator: Predator,
}

impl World {
    pub fn new(species: Vec<SpeciesProfile>,
        predator: Predator,
        spawn: &std::collections::HashMap<String, u32>)
        -> Self {
            let mut species_map = HashMap::new();
            for (i,s) in species.iter().enumerate() {
                species_map.insert(s.id.clone(), i);
            }

            let mut prey = Vec::new();
            for(id, count) in spawn.iter() {
                if let Some(&idx) = species_map.get(id) {
                    for _ in 0..*count {
                        prey.push(Prey {species_idx: idx, age_days: 0});
                    }
                }
            }
            Self {species, species_map, prey, predator}
    }
}