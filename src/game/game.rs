use anyhow::Result;
use async_trait::async_trait;

use crate::engine::{engine::Game, renderer::Renderer};

pub struct Quacker {}

#[async_trait(?Send)]
impl Game for Quacker {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        Ok(Box::new(Quacker {}))
    }

    fn update(&mut self) {}

    fn draw(&self, renderer: &Renderer) {}
}
