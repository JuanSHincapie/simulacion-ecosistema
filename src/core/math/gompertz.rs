
#[derive(Debug, Clone, Copy)]
pub struct Gompertz {
    pub a: f32,
    pub b: f32,
    pub t0: f32,
}

impl Gompertz {
   pub fn weight_at_days(&self, days: u32) -> f32 {
    let t = days as f32;
    self.a * (-(-self.b * (t - self.t0)).exp()).exp()
   }
}