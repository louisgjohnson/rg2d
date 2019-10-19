use sdl2::event::Event;
use sdl2::EventPump;
use std::time::Duration;
use rg2d::context::Context;


pub struct GameLoop {

}

trait Game {
    fn update(&self, ctx: &mut Context);
    fn render(&self, ctx: &mut Context);
}

pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), String>;
    fn render(&mut self, _ctx: &mut Context) -> Result<(), String>;
}


impl GameLoop {
  pub fn run<S>(mut ctx: &mut Context, events: &mut EventPump, state: &mut S) -> Result<(), String> where S: EventHandler {

    'main: loop {
        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'main;
            };
        }
        ctx.input.set_keys(&events);
        state.update(&mut ctx);
        ctx.canvas.clear();
        state.render(&mut ctx);
        ctx.canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
  }
}