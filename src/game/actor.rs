use std::cell::RefMut;

use crate::{
    engine::engine::Game,
    math::{Mat4::mat4, Vec3::vec3},
};

use super::component::{Component, ComponentPointer};

pub enum State {
    EActive,
    EPaused,
    EDead,
}

pub trait Actor<'a, 'b> {
    fn initialize(&self, game: &'a dyn Game);
    fn update(&mut self, delta: f32);
    fn updateComponents(&mut self, delta: f32);
    fn updateActor(&mut self, delta: f32);

    fn processInput(&self); //implement a key state

    fn setPosition(&self, position: vec3);
    fn getPosition(&self) -> vec3;

    fn setScale(&self, scale: f32);
    fn getScale(&self) -> f32;

    fn setRotation(&self, scale: f32);
    fn getRotation(&self) -> f32;

    fn computeWorldTransform(&self);
    fn getWorldTransform(&self) -> mat4;

    fn getForward(&self) -> vec3;

    fn getState(&self) -> State;
    fn setState(&self, state: State);

    fn getGame(&self) -> &'a dyn Game;

    fn addComponent(&self, component: ComponentPointer<'a, 'b>);
    fn removeComponent(&self, component: ComponentPointer<'a, 'b>);

    fn writePacket(&self) {}
}

pub type ActorPointer<'a, 'b> = RefMut<'a, dyn Actor<'a, 'b>>;

pub struct BaseActor<'a, 'b> {
    mState: State,
    mPosition: vec3,
    mWorldTransform: mat4,
    mScale: f32,
    mRotation: f32,
    mRecomputeWorldTransform: bool,
    mComponents: Vec<ComponentPointer<'a, 'b>>,
    mGame: &'b dyn Game,
}
