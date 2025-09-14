use macroquad::prelude::*;
use crate::core::time::Timer;

pub async fn draw_hud(timer: &Timer, fps: f32) {
    let y0 = 20.0;
    let lh = 22.0;

    draw_text("Eco Sim (Macroquad + Timer Core)", 20.0, y0, 26.0, WHITE);
    draw_text(&format!("FPS: {:.0}", fps), 20.0, y0 + lh, 22.0, WHITE);
    draw_text(&format!("Day: {:.3}", timer.current_day.0), 20.0, y0 + 2.0*lh, 22.0, WHITE);
    draw_text(&format!("Tick: {}", timer.current_tick.0), 20.0, y0 + 3.0*lh, 22.0, WHITE);
    draw_text(&format!("Scale: {:.2}x {}", timer.time_scale, if timer.paused { "(PAUSED)" } else { "" }), 20.0, y0 + 4.0*lh, 22.0, WHITE);
    draw_text("Controles: SPACE pausa | Z/X -/+ velocidad | F fast-forward 8x", 20.0, y0 + 6.0*lh, 20.0, GRAY);

}