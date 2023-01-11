use raylib::prelude::*;

pub struct Window {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
}

impl Window {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(64, 64)
            .title("meow")
            .undecorated()
            .transparent()
            .build();

        let window_state = WindowState::default()
            .set_window_topmost(true)
            .set_window_always_run(true);

        rl.set_window_state(window_state);
        rl.set_target_fps(60);

        let curr_monitor = window::get_current_monitor();
        let height = window::get_monitor_height(curr_monitor);
        rl.set_window_position(0, height);

        Self { rl, thread }
    }

    pub fn set_x(&mut self, x: i32) {
        let curr_monitor = window::get_current_monitor();
        let height = window::get_monitor_height(curr_monitor);
        self.rl.set_window_position(x, height);
    }

    pub fn get_width(&self) -> i32 {
        let curr_monitor = window::get_current_monitor();
        window::get_monitor_width(curr_monitor)
    }
}
