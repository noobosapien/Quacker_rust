use glam::{Mat4, Vec3};
use std::cell::RefMut;

use crate::engine::engine::Game;

use super::component::{Component, ComponentPointer};

pub enum State {
    EActive,
    EPaused,
    EDead,
}

pub trait Actor<'a, 'b> {
    fn initialize(&mut self, game: &'a mut dyn Game);
    fn update(&mut self, delta: f32);
    fn updateComponents(&mut self, delta: f32);
    fn updateActor(&mut self, delta: f32);

    fn processInput(&mut self); //implement a key state

    fn setPosition(&mut self, position: Vec3);
    fn getPosition(&self) -> &Vec3;

    fn setScale(&mut self, scale: f32);
    fn getScale(&self) -> f32;

    fn setRotation(&mut self, scale: f32);
    fn getRotation(&self) -> f32;

    fn computeWorldTransform(&mut self);
    fn getWorldTransform(&self) -> &Mat4;

    fn getForward(&self) -> Vec3;

    fn getState(&self) -> &State;
    fn setState(&mut self, state: State);

    fn getGame(&mut self) -> &mut dyn Game;

    fn addComponent(&self, component: ComponentPointer<'a, 'b>);
    fn removeComponent(&self, component: ComponentPointer<'a, 'b>);

    fn writePacket(&self) {}
}

pub type ActorPointer<'a, 'b> = RefMut<'a, dyn Actor<'a, 'b>>;

pub struct BaseActor<'a, 'b> {
    mState: State,
    mPosition: Vec3,
    mWorldTransform: Mat4,
    mScale: f32,
    mRotation: f32,
    mRecomputeWorldTransform: bool,
    mComponents: Vec<ComponentPointer<'a, 'b>>,
    mGame: &'b mut dyn Game,
}

impl<'a, 'b> Actor<'a, 'b> for BaseActor<'a, 'b>
where
    'a: 'b,
{
    fn initialize(&mut self, game: &'a mut dyn Game) {
        self.mGame = game;
        self.mState = State::EActive;
        self.mPosition = Vec3::new(0.0, 0.0, 0.0);
        self.mWorldTransform = Mat4::identity();
        self.mScale = 1.0;
        self.mRotation = 0.0;
        self.mRecomputeWorldTransform = true;
    }

    fn update(&mut self, delta: f32) {
        match self.mState {
            State::EActive => {
                self.computeWorldTransform();
                self.updateComponents(delta);
                self.updateActor(delta);
                self.computeWorldTransform();
            }
            _ => {}
        }
    }

    fn updateComponents(&mut self, delta: f32) {
        for component in self.mComponents.iter_mut() {
            component.update(delta);
        }
    }
    fn updateActor(&mut self, delta: f32) {}

    fn processInput(&mut self) {} //implement a key state

    fn setPosition(&mut self, position: Vec3) {}
    fn getPosition(&self) -> &Vec3 {
        &self.mPosition
    }

    fn setScale(&mut self, scale: f32) {}
    fn getScale(&self) -> f32 {
        self.mScale
    }

    fn setRotation(&mut self, scale: f32) {}
    fn getRotation(&self) -> f32 {
        self.mRotation
    }

    fn computeWorldTransform(&mut self) {}
    fn getWorldTransform(&self) -> &Mat4 {
        &self.mWorldTransform
    }

    fn getForward(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn getState(&self) -> &State {
        &self.mState
    }
    fn setState(&mut self, state: State) {}

    fn getGame(&mut self) -> &mut dyn Game {
        self.mGame
    }

    fn addComponent(&self, component: ComponentPointer<'a, 'b>) {}
    fn removeComponent(&self, component: ComponentPointer<'a, 'b>) {}

    fn writePacket(&self) {}
}
