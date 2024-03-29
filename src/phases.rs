use macroquad::prelude::*;

#[derive(Clone)]

pub enum Phase {
    Quit,
    Menu,
    Story(String),
}

pub trait PhaseBehavior<T>: Sized {
    async fn setup(setup_data: T) -> Self;
    async fn update(&mut self) -> Option<Phase>;
}

pub trait PhaseLoop<T> {
    async fn run(render_target: &RenderTarget, render_camera: &Camera2D, setup_data: T) -> Phase;

    /// Returns the size and offset the screen texture will be applied to.
    fn get_screen_size_and_offset(&self) -> (Vec2, Vec2);
}

impl <T, P: PhaseBehavior<T>> PhaseLoop<T> for P {
    async fn run(render_target: &RenderTarget, render_camera: &Camera2D, setup_data: T) -> Phase {
        let mut phase = Self::setup(setup_data).await;

        loop {
            set_camera(render_camera);

            let next_phase = phase.update().await;
    
            set_default_camera();
    
            clear_background(Color::new(0.0, 0.0, 0.0, 1.0));

            let (dest_size, dest_offset) = phase.get_screen_size_and_offset();

            let draw_params = DrawTextureParams {
                rotation: 0.0,
                dest_size: Some(dest_size),
                source: None,
                flip_x: false,
                flip_y: false,
                pivot: None,
            };
    
            draw_texture_ex(
                &render_target.texture, 
                dest_offset.x,
                dest_offset.y,
                Color::new(1.0, 1.0, 1.0, 1.0), 
                draw_params);
    
            next_frame().await;

            if let Some(next_phase) = next_phase {
                return next_phase;
            }
        }
    }

    fn get_screen_size_and_offset(&self) -> (Vec2, Vec2) {
        let screen_scale_x = screen_width() / 320.0;
        let screen_scale_y = screen_height() / 180.0;
        let screen_scale = screen_scale_x.min(screen_scale_y);

        let dest_size = vec2(320.0, 180.0) * screen_scale;

        let dest_offset = vec2(
            screen_width() / 2.0 - dest_size.x / 2.0, 
            screen_height() / 2.0 - dest_size.y / 2.0
        );

        (dest_size, dest_offset)
    }
}
