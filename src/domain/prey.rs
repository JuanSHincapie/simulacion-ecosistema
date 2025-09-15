#[derive(Debug, Clone)]
pub struct SpeciesProfile {
    pub id: String,
    pub display_name: String,
    pub maturity_days: u32,
}

#[derive(Debug, Clone)]
pub struct Prey {
    pub species_idx: usize,
    pub age_days: u32
}

impl Prey {
    pub fn age_one_day(&mut self) {
        self.age_days = self.age_days.saturating_add(1);
    }
}