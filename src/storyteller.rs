use macroquad::prelude::*;
use crate::phases::{Phase, PhaseLoop};
use crate::menu::Menu;
use crate::story::Story;

pub struct Storyteller {
    render_target: RenderTarget,
    render_camera: Camera2D,
    phase: Phase,
}

impl Storyteller {
    pub fn new(width: u32, height: u32) -> Self {
        let render_target = render_target(width, height);
        render_target.texture.set_filter(FilterMode::Nearest);

        let render_camera = Camera2D{
            render_target: Some(render_target.clone()),
            target: vec2(160.0, 90.0),
            zoom: vec2(2.0 / 320.0, 2.0 / 180.0),
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
                    self.phase = Menu::run(&self.render_target, &self.render_camera, ()).await;
                },
                Phase::Story(path) => {
                    self.phase = Story::run(&self.render_target, &self.render_camera, path).await;
                }
            }
        }
    }
}