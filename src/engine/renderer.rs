use web_sys::WebGl2RenderingContext;

use super::browser;

pub struct Renderer {
    pub context: WebGl2RenderingContext,
}

impl Renderer {
    pub fn initialize(&self) {
        let gl = &self.context;

        gl.viewport(0, 0, 600, 600)
    }
    pub fn clear(&self) {}
}
