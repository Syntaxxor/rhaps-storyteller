use macroquad::{prelude::*, ui::root_ui};
use crate::phases::*;

#[derive(Copy, Clone)]
enum MenuState {
    Main,
    Options,
    StorySelector,
}

pub struct Menu {

}

impl PhaseBehavior for Menu {
    fn setup() -> Self {
        Self {

        }
    }

    fn update(&mut self) -> Option<Phase> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(Phase::Quit);
        }

        None
    }

    fn render(&self) {
        clear_background(Color::new(1.0, 1.0, 1.0, 1.0));

        root_ui().label(vec2(160.0, 90.0), "Storyteller");
    }
}