use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct WorldCfg{
    pub ticks_per_day: u32,
    pub days_per_second: f64,
    pub max_steps_per_frame: u32,
    pub time_scale: f64,
    pub seed: u64,
    pub spawn: std::collections::HashMap<String, u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PredatorCfg {
    pub reserve_min: f32,
    pub reserve_opt: f32,
    pub efficiency: f32,
    pub prey_min_slaughter_age: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpeciesGrompertz {
    pub a: f32,
    pub b: f32,
    pub t0: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpeciesReproduction {
    pub p_daily: f32,
    pub sex_ratio: f32,
    pub offspring_min: u32,
    pub offspring_max: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpeciesHealth {
    pub p_sick: f32,
    pub p_die_if_sick: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpeciesMaturity {
    pub age_days: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpeciesCfg {
    pub id: String,
    pub display_name: String,
    pub gompertz: SpeciesGrompertz,
    pub reproduction: SpeciesReproduction,
    pub health: SpeciesHealth,
    pub maturity: SpeciesMaturity,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AllConfig {
    pub world: WorldCfg,
    pub predator: PredatorCfg,
    pub species: Vec<SpeciesCfg>,
}


pub fn load_all() -> anyhow::Result<AllConfig> {
    let world = fs::read_to_string("config/world.toml")?;
    let predator = fs::read_to_string("config/predator.toml")?;

    let world: WorldCfg = toml::from_str(&world)?;
    let predator: PredatorCfg = toml::from_str(&predator)?;

    let mut species = Vec::new();
    for entry in fs::read_dir("config/species")? {
       let entry = entry?;
       if entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
            let txt = fs::read_to_string(entry.path())?;
            let cfg: SpeciesCfg = toml::from_str(&txt)?;
            species.push(cfg);
        }
    }
    Ok(AllConfig { world, predator, species })
}