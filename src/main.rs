use ggez::conf;
use ggez::event::{self, Button, EventHandler, GamepadId, KeyCode, KeyMods};
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

struct MainState {
    start: Instant,
    first_loop: bool,
}

impl MainState {
    pub fn new(start: Instant) -> GameResult<MainState> {
        let this = MainState {
            start,
            first_loop: true,
        };
        Ok(this)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let now = Instant::now();
        if self.first_loop {
            println!(
                "Time to first loop: {} s",
                now.duration_since(self.start).as_secs_f32()
            );
            self.first_loop = false;
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Q && keymods.contains(KeyMods::CTRL) {
            ggez::event::quit(ctx);
        }
    }
    fn gamepad_button_down_event(&mut self, ctx: &mut Context, button: Button, _id: GamepadId) {
        if button == Button::Mode {
            ggez::event::quit(ctx);
        }
    }
}

pub fn main() -> GameResult {
    let start = Instant::now();
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_setup(
        conf::WindowSetup::default()
            .title("super_simple")
            .vsync(true),
    );
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(start)?;
    event::run(ctx, event_loop, state)
}
