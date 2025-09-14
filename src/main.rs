use macroquad::prelude::*;

#[macroquad::main("predator")]
async fn main() {


    loop {
        clear_background(RED);       
        next_frame().await;
    }
}
