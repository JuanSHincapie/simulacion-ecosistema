use crate::core::time::{Timer, TimerStep};
use crate::core::units::Seconds;

fn main_loop() {
    let mut timer = Timer::new(1440, 0.25, 32);

    loop {
        let real_dt = Seconds(0.016);
        let step = timer.update(real_dt);

        for _ in 0..step.steps_to_run {
            //TODO: aqui iría la parte de hacer cosas bonitas, no olvidar
        }

        if step.end_of_day {
            println!("Fin del día {:?}", timer.current_day)
        }
    }
}