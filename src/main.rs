mod phases;
mod storyteller;
mod menu;
mod story;
mod tilemap;

use storyteller::Storyteller;
use macroquad::prelude::*;


const RENDER_WIDTH: u32 = 320;
const RENDER_HEIGHT: u32 = 180;


fn window_conf() -> Conf {
    Conf {
        window_title: "Storyteller".to_owned(),
        fullscreen: false,
        window_width: (RENDER_WIDTH * 2) as i32,
        window_height: (RENDER_HEIGHT * 2) as i32,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    // This engine is set up to render at 320x180 at all times.
    let mut storyteller = Storyteller::new(RENDER_WIDTH, RENDER_HEIGHT);

    storyteller.run().await;
}
