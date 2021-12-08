use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

struct MainState {
    start: Instant,
    builder: Instant,
    context: Instant,
    first_loop: bool,
}

impl MainState {
    pub fn new(start: Instant, builder: Instant, context: Instant) -> GameResult<MainState> {
        let this = MainState {
            start,
            builder,
            context,
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
                "Time at create builder: {} s",
                self.builder.duration_since(self.start).as_secs_f32()
            );
            println!(
                "Time at build context: {} s",
                self.context.duration_since(self.start).as_secs_f32()
            );
            println!(
                "Time at first loop: {} s",
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
    let builder = Instant::now();
    let (ctx, event_loop) = cb.build()?;
    let context = Instant::now();
    let state = MainState::new(start, builder, context)?;
    event::run(ctx, event_loop, state)
}
