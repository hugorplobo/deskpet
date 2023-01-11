use std::time::SystemTime;
use raylib::prelude::*;
use rand::prelude::*;

use crate::{window::Window, animated_sprite::AnimatedSprite};

enum CatState {
    Idle,
    Walking,
    Sleeping,
}

pub struct Cat {
    x: f64,
    state: CatState,
    speed: f64,
    timer: SystemTime,
    wait_secs: u64,
    sprite: AnimatedSprite
}

impl Cat {
    pub fn from_window(window: &mut Window) -> Self {
        Self {
            x: 0.0,
            state: CatState::Idle,
            speed: 0.25,
            timer: SystemTime::now(),
            wait_secs: 20,
            sprite: AnimatedSprite::from_window(window, "sprites/idle_right.png", 1, 1, true)
        }
    }

    pub fn choose_movement(&mut self, window: &mut Window) {
        if self.timer.elapsed().unwrap().as_secs() > self.wait_secs {
            self.timer = SystemTime::now();
            let n = thread_rng().gen_range(1..=3);

            match n {
                1 => {
                    self.change_to_idle(window);
                },
                2 => {
                    self.change_direction(window);
                },
                3 => {
                    self.change_to_sleep(window);
                },
                _ => {
                    unreachable!();
                }
            }
        } else {
            match self.state {
                CatState::Walking => {
                    self.walk(window);
                },
                _ => {

                }
            }
        }

        let mut handler = window.rl.begin_drawing(&window.thread);
        handler.clear_background(Color::new(0, 0, 0, 0));
        self.sprite.draw(&mut handler);
    }

    fn walk(&mut self, window: &mut Window) {
        self.x += self.speed;
        window.set_x(self.x as i32);

        let width = window.get_width();

        if self.x >= width as f64 {
            self.change_direction(window);
        } else if self.speed < 0.0 && self.x <= 0.0 {
            self.change_direction(window);
        }
    }

    fn change_direction(&mut self, window: &mut Window) {
        self.wait_secs = 20;

        if thread_rng().gen_bool(0.2) {
            self.speed *= -1.0;
            self.state = CatState::Idle;
        }

        if !matches!(self.state, CatState::Walking) {
            self.state = CatState::Walking;
            
            if self.speed.signum() == -1.0 {
                self.sprite = AnimatedSprite::from_window(window, "sprites/walk_left.png", 3, 4, true);
            } else {
                self.sprite = AnimatedSprite::from_window(window, "sprites/walk_right.png", 3, 4, true);
            }
        }
    }

    fn change_to_idle(&mut self, window: &mut Window) {
        self.state = CatState::Idle;
        self.wait_secs = 5;

        if self.speed.signum() == -1.0 {
            self.sprite = AnimatedSprite::from_window(window, "sprites/idle_left.png", 1, 1, true);
        } else {
            self.sprite = AnimatedSprite::from_window(window, "sprites/idle_right.png", 1, 1, true);
        }
    }

    fn change_to_sleep(&mut self, window: &mut Window) {
        self.wait_secs = 15;

        if !matches!(self.state, CatState::Sleeping) {
            self.state = CatState::Sleeping;
            self.sprite = AnimatedSprite::from_window(window, "sprites/sleep.png", 3, 9, false);
        }
    }
}
