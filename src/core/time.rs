use crate::core::units::{Day, DtDay, Seconds, Tick};

pub struct Timer {
    pub current_day: Day,
    pub current_tick: Tick,
    accumulator_day: DtDay,
    dt_day: DtDay,
    days_per_second: f64,
    max_steps_per_frame: u32,
    pub time_scale: f64,
    pub paused: bool,
}

impl Timer {
    pub fn new(ticks_per_day: u32, days_per_second: f64, max_steps_per_frame: u32) -> Self {
        let dt_day = 1.0/ticks_per_day as f64;
        Self {
            current_day: Day(0.0),
            current_tick: Tick(0),
            accumulator_day: DtDay(0.0),
            dt_day: DtDay(dt_day),
            days_per_second,
            max_steps_per_frame,
            time_scale: 1.0,
            paused: false,
        }
    }
}

pub struct TimerStep{
    pub steps_to_run: u32,
    pub end_of_day: bool,
}


impl Timer {
    pub fn update(&mut self, real_dt: &Seconds) -> TimerStep {
        if self.paused {
            return TimerStep{ steps_to_run: 0, end_of_day: false };
        }

        let delta_day_effective = real_dt.0 as f64 * self.days_per_second * self.time_scale;
        self.accumulator_day.0 += delta_day_effective;

        let mut steps_to_run = (self.accumulator_day.0 / self.dt_day.0).floor() as u32;
        if steps_to_run > self.max_steps_per_frame {
            steps_to_run = self.max_steps_per_frame;
        }
        
        let mut end_of_day = false;
        if steps_to_run > 0 {
           self.accumulator_day.0 -= steps_to_run as f64 * self.dt_day.0;
              self.current_tick.0 += steps_to_run as u64;
              let prev_day = self.current_day.0;
              self.current_day.0 += steps_to_run as f64 * self.dt_day.0;

              if prev_day.floor() < self.current_day.0.floor() {
                end_of_day = true;
              }
        }
        TimerStep{ steps_to_run, end_of_day }
    }
}