use macroquad::prelude::*;
use crate::phases::{Phase, PhaseRender};
use crate::menu::Menu;

pub struct Storyteller {
    render_target: RenderTarget,
    render_camera: Camera2D,
    phase: Phase,
}

impl Storyteller {
    pub fn new(width: u32, height: u32) -> Self {
        let render_target = render_target(width, height);

        let render_camera = Camera2D{
            render_target: Some(render_target.clone()),
            ..Default::default()
        };

        let phase = Phase::Menu;

        Self {render_target, render_camera, phase}
    }

    pub async fn run(&mut self) {
        loop {
            match self.phase.clone() {
                Phase::Quit => break,
                Phase::Menu => {
                    self.phase = Menu::run(&self.render_target, &self.render_camera).await;
                },
                Phase::Story(path) => {

                }
            }
        }
    }
}