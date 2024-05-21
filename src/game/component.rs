use std::cell::RefMut;

use super::actor::ActorPointer;

pub trait Component<'a, 'b> {
    fn initialize(&mut self, actor: ActorPointer<'a, 'b>, updateOrder: i32);
    fn update(&mut self, delta: f32);
    fn processInput(&mut self); // create a key state
    fn onUpdateWorldTransform(&mut self);

    fn getUpdateOrder(&self) -> i32;
}
pub type ComponentPointer<'a, 'b> = RefMut<'a, dyn Component<'a, 'b>>;

pub struct BaseComponent<'a, 'b> {
    mOwner: ActorPointer<'a, 'b>,
    mUpdateOrder: i32,
}
