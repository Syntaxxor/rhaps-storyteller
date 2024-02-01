use macroquad::prelude::*;

#[derive(Clone)]

pub enum Phase {
    Quit,
    Menu,
    Story(String),
}

pub trait PhaseBehavior {
    fn setup() -> Self;
    fn update(&mut self) -> Option<Phase>;
    fn render(&self);
}

pub trait PhaseRender {
    async fn run(render_target: &RenderTarget, render_camera: &Camera2D) -> Phase;
}

impl <T: PhaseBehavior> PhaseRender for T {
    async fn run(render_target: &RenderTarget, render_camera: &Camera2D) -> Phase {
        let mut phase = Self::setup();

        loop {
            let next_phase = phase.update();

            if let Some(next_phase) = next_phase {
                return next_phase;
            }

            set_camera(render_camera);

            phase.render();
    
            set_default_camera();
    
            clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
    
            draw_texture_ex(&render_target.texture, 0.0, 0.0, Color::new(1.0, 1.0, 1.0, 1.0), DrawTextureParams::default());
    
            next_frame().await;
        }
    }
}
