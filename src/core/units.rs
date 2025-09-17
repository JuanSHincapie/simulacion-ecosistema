pub struct Day(pub f64);

pub struct Tick(pub u64);

pub struct Seconds(pub f64);

pub struct DtDay(pub f64);

pub struct Age(pub u32);

pub struct Weight(pub f32);

impl Weight {
    pub fn zero() -> Self {
        Weight(0.0)
    }
}