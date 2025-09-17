use crate::core::units::{Age, Weight};
use crate::core::math::gompertz::Gompertz;

#[derive(Debug, Clone)]
pub struct SpeciesProfile {
    pub id: String,
    pub display_name: String,
    pub maturity_days: u32,
    pub gompertz: Gompertz,
}

#[derive(Debug, Clone)]
pub struct Prey {
    pub species_idx: usize,
    pub age_days: Age,  
    pub weight: Weight,
}

impl Prey {
    pub fn age_one_day(&mut self) {
        self.age_days = self.age_days.saturating_add(1);
    }
}