use ggez::conf;
use ggez::event::{self, EventHandler};
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
}

pub fn main() -> GameResult {
    let start = Instant::now();
    let cb = ggez::ContextBuilder::new("Gamepad Start Lag MVE", "Tubbles")
        .window_setup(conf::WindowSetup::default());
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(start)?;
    event::run(ctx, event_loop, state)
}
