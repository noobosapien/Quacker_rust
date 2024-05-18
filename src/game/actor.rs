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
    fn initialize(game: &'a dyn Game);
    fn update(delta: f32);
    fn updateComponents(delta: f32);
    fn updateActor(delta: f32);

    fn processInput(); //implement a key state

    fn setPosition(position: vec3);
    fn getPosition(&self) -> vec3;

    fn setScale(scale: f32);
    fn getScale(&self) -> f32;

    fn setRotation(scale: f32);
    fn getRotation(&self) -> f32;

    fn computeWorldTransform();
    fn getWorldTransform() -> mat4;

    fn getForward() -> vec3;

    fn getState() -> State;
    fn setState(state: State);

    fn getGame() -> &'a dyn Game;

    fn addComponent(component: ComponentPointer<'b>);
    fn removeComponent(component: ComponentPointer<'b>);

    fn writePacket() {}
}

pub struct BaseActor<'a, 'b> {
    mState: State,
    mPosition: vec3,
    mWorldTransform: mat4,
    mScale: f32,
    mRotation: f32,
    mRecomputeWorldTransform: bool,
    mComponents: Vec<ComponentPointer<'a>>,
    mGame: &'b dyn Game,
}
