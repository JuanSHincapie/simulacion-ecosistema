pub trait SpeciesConfig {
    fn name(&self) -> &str;
    fn max_age(&self) -> u32;
    fn reproduction_rate(&self) -> f32;
    fn initial_health(&self) -> f32;
    fn grow(&mut self);
    fn reproduce(&mut self) -> Option<Self> where Self: Sized;
    fn consume(&mut self, resource: f32);
    fn move_towards(&mut self, target: (f32, f32));
    fn age(&mut self);
    fn update_health(&mut self, delta: f32);
    fn check_mortality(&self) -> bool;
}


// Configuración de los conejos （￣︶￣）↗　
pub struct RabbitConfig {
    pub age: u32,
    pub health: f32,
    pub position: (f32, f32),
}

impl RabbitConfig {
    pub fn new() -> Self {
        Self {
            age: 0,
            health: 100.0,
            position: (0.0, 0.0),
        }
    }
}

impl SpeciesConfig for RabbitConfig {
    fn name(&self) -> &str {
        "Conejo"
    }
    fn max_age(&self) -> u32 {
        5
    }
    fn reproduction_rate(&self) -> f32 {
        0.8
    }   
    fn initial_health(&self) -> f32 {
        100.0
    }
    fn grow(&mut self) {
        self.health += 1.0;
    }
    fn reproduce(&mut self) -> Option<Self> {
        if self.health > 50.0 && self.age > 1 {
            Some(RabbitConfig::new())
        } else {
            None
        }
    }
    fn consume(&mut self, resource: f32) {
        self.health += resource;
    }
    fn move_towards(&mut self, target: (f32, f32)) {
        self.position = target;
    }
    fn age(&mut self) {
        self.age += 1;
    }
    fn update_health(&mut self, delta: f32) {
        self.health += delta;
    }
    fn check_mortality(&self) -> bool {
        self.age >= self.max_age() || self.health <= 0.0
    }
}


// Configuración de las ovejas (ʘ‿ʘ)
pub struct SheepConfig {
    pub age: u32,
    pub health: f32,
    pub position: (f32, f32),
}

impl SheepConfig {
    pub fn new() -> Self {
        Self {
            age: 0,
            health: 120.0,
            position: (0.0, 0.0),
        }
    }
}

impl SpeciesConfig for SheepConfig {
    fn name(&self) -> &str {
        "Oveja"
    }
    fn max_age(&self) -> u32 {
        9
    }
    fn reproduction_rate(&self) -> f32 {
        0.4
    }
    fn initial_health(&self) -> f32 {
        120.0
    }
    fn grow(&mut self) {
        self.health += 1.5;
    }
    fn reproduce(&mut self) -> Option<Self> {
        if self.health > 60.0 && self.age > 2 {
            Some(SheepConfig::new())
        } else {
            None
        }
    }
    fn consume(&mut self, resource: f32) {
        self.health += resource;
    }
    fn move_towards(&mut self, target: (f32, f32)) {
        self.position = target;
    }
    fn age(&mut self) {
        self.age += 1;
    }
    fn update_health(&mut self, delta: f32) {
        self.health += delta;
    }
    fn check_mortality(&self) -> bool {
        self.age >= self.max_age() || self.health <= 0.0
    }

}


pub struct WolfConfig {
    pub age: u32,
    pub health: f32,
    pub position: (f32, f32),
}

impl SpeciesConfig for WolfConfig {
    fn name(&self) -> &str {
        "Lobo"
    }
    fn max_age(&self) -> u32 {
        12
    }
    fn reproduction_rate(&self) -> f32 {
        0.0
    }
    fn initial_health(&self) -> f32 {
        150.0
    }
    fn grow(&mut self) {
        self.health += 2.0;
    }
    fn reproduce(&mut self) -> Option<Self> {
        if self.health > 80.0 && self.age > 3 {
            Some(WolfConfig {
                age: 0,
                health: self.initial_health(),
                position: self.position,
            })
        } else {
            None
        }
    }
    fn consume(&mut self, resource: f32) {
        self.health += resource;
    }
    fn move_towards(&mut self, target: (f32, f32)) {
        self.position = target;
    }
    fn age(&mut self) {
        self.age += 1;
    }
    fn update_health(&mut self, delta: f32) {
        self.health += delta;
    }
    fn check_mortality(&self) -> bool {
        self.age >= self.max_age() || self.health <= 0.0
    }
}