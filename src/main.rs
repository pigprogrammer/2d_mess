extern crate ggez;


use ggez::{Context,event,graphics,GameResult};
use ggez::event::Keycode;

use std::time::{Duration, Instant};

pub struct Player {
    pub pos: [f32;2],
    pub spawned: bool,
}

struct Entity {
    position: [f32;2],
}

enum Entities {
    Player(Player),
    Other(Entity)
}

trait Update {
    fn update(&mut self,ctx: &mut Context);

    fn draw(&mut self,ctx: &mut Context) {}

    fn key_down_event(&mut self,ctx: &mut Context,keycode: Keycode,_keymod: event::Mod,_repeat: bool) {}

    fn key_up_event(&mut self,ctx: &mut Context,keycode: Keycode,_keymod: event::Mod,_repeat: bool) {}
}

impl Update for Player {
    fn update(&mut self, _ctx: &mut Context) {
        
        if !self.spawned {
            self.pos[0] = 800.0 / 2.0;
            self.pos[1] = 600.0 / 2.0;
            self.spawned = true;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        let rect = graphics::Rect::new(self.pos[0],self.pos[1],20.0,10.0);
        
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).unwrap();

    }

    fn key_down_event(&mut self,ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => {
                self.pos[1] -= 4.0;
            }
            Keycode::Down => {
                self.pos[1] += 4.0;
            }

            Keycode::Right => {
                self.pos[0] += 4.0;
            }
            Keycode::Left => {
                self.pos[0] -= 4.0;
            }

            _ => (),
        }
    }
}

struct Game {
    entities: Vec<Entities>,
    last_update: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            entities: Vec::new(),
            last_update: Instant::now()
        }
    }
}

const UPDATES_PER_SECOND: f32 = 8.0;
// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            for e in self.entities.iter_mut() {
                match e {
                    Entities::Player(p) => p.update(ctx),
                    _ => (),
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        for e in self.entities.iter_mut() {
            match e {
                Entities::Player(p) => p.draw(ctx),
                _ => ()

            }
        }
        graphics::present(ctx);
        ggez::timer::yield_now();
        Ok(())
    }
    
    fn key_down_event(&mut self,ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {
        for e in self.entities.iter_mut() {
            match e {
                Entities::Player(p) => p.key_down_event(ctx,keycode,_keymod,_repeat),
                _ => ()
            }
        }
    }
}

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("Testing", "User")
        .window_setup(ggez::conf::WindowSetup::default().title("TEST GAME"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800, 600))
        .build().expect("Failed to build ggez context");

        graphics::set_background_color(ctx, [0.0, 1.0, 0.0, 1.0].into());
    let mut state =&mut Game::new();

    state.entities.push(Entities::Player(Player {
        pos: [0.0,0.0],
        spawned: false,
    }));

    match event::run(ctx,state) {
                // If we encounter an error, we print it before exiting
        Err(e) => println!("Error encountered running game: {}", e),
        // And if not, we print a message saying we ran cleanly. Hooray!
        Ok(_) => println!("Game exited cleanly!"),
    }
}
