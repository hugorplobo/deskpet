use crate::{window::Window, cat::Cat};

pub struct Application {
    window: Window,
    cat: Cat
}

impl Application {
    pub fn new() -> Self {
        let mut window = Window::new();

        Self {
            cat: Cat::from_window(&mut window),
            window
        }
    }

    pub fn run(&mut self) {
        while !self.window.rl.window_should_close() {
            self.cat.choose_movement(&mut self.window);
        }
    }
}
