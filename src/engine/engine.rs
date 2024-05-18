use crate::engine::browser;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::{cell::RefCell, error::Error, rc::Rc};

use super::{browser::LoopClosure, renderer::Renderer};

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&self, renderer: &Renderer);
}

pub struct GameLoop {
    last_frame: f64,
    accumulated_time: f64,
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

const FRAME_SIZE: f64 = 1.0 / 60.0 * 1000.0;

impl GameLoop {
    pub async fn start(game: impl Game) -> Result<()> {
        let mut game = game.initialize().await?;

        let mut game_loop = GameLoop {
            last_frame: browser::Browser::now()?,
            accumulated_time: 0.0,
        };

        let renderer = Renderer {
            context: browser::Browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(browser::Browser::create_raf_closure(move |perf: f64| {
            let frame_time: f64 = perf - game_loop.last_frame;

            game_loop.accumulated_time += frame_time;

            while game_loop.accumulated_time > FRAME_SIZE {
                game.update();
                game_loop.accumulated_time -= FRAME_SIZE;
            }

            game_loop.last_frame = perf;
            game.draw(&renderer);

            browser::Browser::request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
        }));

        browser::Browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;

        Ok(())
    }
}
