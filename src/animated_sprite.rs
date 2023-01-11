use raylib::prelude::*;

use crate::window::Window;

pub struct AnimatedSprite {
    spritesheet: Texture2D,
    fps: u64,
    frames_qtd: u64,
    looping: bool,
    frames_counter: u64,
    current_frame: u64,
    frame_rec: Rectangle
}

impl AnimatedSprite {
    pub fn from_window(window: &mut Window, file: &str, fps: u64, frames_qtd: u64, looping: bool) -> Self {
        let spritesheet = window.rl.load_texture(&window.thread, file)
            .expect("File not found");

        Self {
            frames_counter: 0,
            current_frame: 0,
            frame_rec: Rectangle { 
                x: 0.0, 
                y: 0.0, 
                width: spritesheet.width as f32 / frames_qtd as f32, 
                height: spritesheet.height as f32
            },
            spritesheet,
            fps,
            frames_qtd,
            looping
        }
    }

    pub fn draw(&mut self, handler: &mut RaylibDrawHandle) {
        self.frames_counter += 1;

        if self.frames_counter >= (60 / self.fps) {
            self.frames_counter = 0;
            self.current_frame += 1;

            if self.current_frame >= self.frames_qtd {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames_qtd - 1;
                }
            }

            self.frame_rec.x = (self.current_frame * self.spritesheet.width as u64 / self.frames_qtd) as f32;
        }

        handler.draw_texture_pro(
            &self.spritesheet,
            self.frame_rec,
            Rectangle::new(0.0, 0.0, self.frame_rec.width * 2.0, self.frame_rec.height * 2.0),
            Vector2::new(0.0, 0.0), 
            0.0,
            Color::WHITE
        );
    }
}