use macroquad::prelude::*;

mod core {
    pub mod units;
    pub mod time;
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

async fn main() {
    let ticks_per_day = 2400;
    let days_per_second = 0.25;
    let max_steps_per_frame = 32;

    let mut timer = Timer::new(ticks_per_day, days_per_second, max_steps_per_frame);
    let mut engine = Engine::new();

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
            // TODO: Este bloque con los actividades de final de día ╰（‵□′）╯
        }

        clear_background(BLACK);
        ui::hud::draw_hud(&timer, (1.0 / real_dt.0) as f32).await;

        next_frame().await;
    }
}