use macroquad::prelude::*;

mod core {
    pub mod units;
    pub mod time;
}

mod io {
    pub mod config;
}

mod domain {
    pub mod traits;
    pub mod prey;
    pub mod predator;
    pub mod world;
}

mod systems {
    pub mod aging;
}

mod sim {
    pub mod engine;
}

mod ui {
    pub mod hud;
}

use core::time::Timer;
use core::units::Seconds;
use sim::engine::Engine;

#[macroquad::main("Ecosim")]
async fn main() -> anyhow::Result<()> {

  let mut engine = Engine::new()?;
  let wcfg = &engine.cfg.world;
  let mut timer = Timer::new(
    wcfg.ticks_per_day,
    wcfg.days_per_second,
    wcfg.max_steps_per_frame,
  );
  timer.time_scale = wcfg.time_scale;
  let mut fast_forward = false;

    loop {
       if is_key_pressed(KeyCode::Space) {
            timer.paused = !timer.paused;
        }
        if is_key_pressed(KeyCode::Z) { 
            timer.time_scale = (timer.time_scale * 0.5).max(0.125);
        }
        if is_key_pressed(KeyCode::X) { 
            timer.time_scale = (timer.time_scale * 2.0).min(32.0);
        }
        if is_key_pressed(KeyCode::F) { 
            fast_forward = !fast_forward;
            timer.time_scale = if fast_forward { 8.0 } else { 1.0 };
        }

        let real_dt = Seconds(get_frame_time() as f64);
        let step = timer.update(&real_dt);


        engine.run_steps(step.steps_to_run);

        if step.end_of_day {
           engine.on_end_of_day();
        }

        clear_background(BLACK);
        ui::hud::draw_hud(&timer, (1.0 / real_dt.0) as f32).await;

        next_frame().await;
    }
}