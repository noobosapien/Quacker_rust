use std::cell::RefMut;

pub trait Component<'a> {}
pub type ComponentPointer<'a> = RefMut<'a, dyn Component<'a>>;

pub struct BaseComponent {}
