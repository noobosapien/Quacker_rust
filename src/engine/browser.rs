use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use std::future::Future;

use wasm_bindgen::{
    closure::WasmClosure, closure::WasmClosureFnOnce, prelude::Closure, JsCast, JsValue,
};

use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, HtmlImageElement,
    Response, Window,
};

macro_rules! log{
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!( $( $t)*).into());
    }
}

macro_rules! error {
    ($ ($t:tt)*) => {
        web_sys::console::error_1(&format!($($t)*).into());
    }
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub struct Browser {}

impl Browser {
    pub fn window() -> Result<Window> {
        web_sys::window().ok_or_else(|| anyhow!("NO WINDOW FOUND"))
    }

    pub fn document() -> Result<Document> {
        Browser::window()?
            .document()
            .ok_or_else(|| anyhow!("No document found"))
    }

    pub fn canvas() -> Result<HtmlCanvasElement> {
        Browser::document()?
            .get_element_by_id("canvas")
            .ok_or_else(|| anyhow!("No canvas element found with id 'canvas'"))?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
    }

    pub fn context() -> Result<web_sys::WebGl2RenderingContext> {
        Browser::canvas()?
            .get_context("webgl")
            .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
            .ok_or_else(|| anyhow!("No 2d Context found"))?
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .map_err(|element| anyhow!("Error converting {:#?} to WebGl2RenderingContext", element))
    }

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future);
    }

    pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
        JsFuture::from(Browser::window()?.fetch_with_str(resource))
            .await
            .map_err(|err| anyhow!("error fetching {:#?}", err))
    }

    pub async fn fetch_response(resource: &str) -> Result<Response> {
        Browser::fetch_with_str(resource)
            .await?
            .dyn_into()
            .map_err(|err| anyhow!("error converting fetch to Response {:#?}", err))
    }

    pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
        let resp = Browser::fetch_response(json_path).await?;

        JsFuture::from(
            resp.json()
                .map_err(|err| anyhow!("Could not get JSON from response {:#?}", err))?,
        )
        .await
        .map_err(|err| anyhow!("error fetching JSON {:#?}", err))
    }

    pub async fn fetch_array_buffer(resource: &str) -> Result<ArrayBuffer> {
        let array_buffer = Browser::fetch_response(resource)
            .await?
            .array_buffer()
            .map_err(|err| anyhow!("Error loading array buffer {:#?}", err))?;

        JsFuture::from(array_buffer)
            .await
            .map_err(|err| anyhow!("Error converting array buffer into future {:#?}", err))?
            .dyn_into()
            .map_err(|err| anyhow!("Error converting raw JSVALUE to ARRAYBUFFER {:#?}", err))
    }

    pub fn new_image() -> Result<HtmlImageElement> {
        HtmlImageElement::new()
            .map_err(|err| anyhow!("Could not create HTMLImageElement : {:#?}", err))
    }

    pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
        closure_wrap(Box::new(f))
    }

    pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
        Browser::window()?
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
    }

    pub fn now() -> Result<f64> {
        Ok(Browser::window()?
            .performance()
            .ok_or_else(|| anyhow!("Performance object not found"))?
            .now())
    }
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}

pub fn draw_ui(html: &str) -> Result<()> {
    find_ui()?
        .insert_adjacent_html("afterbegin", html)
        .map_err(|err| anyhow!("Could not insert html {:#?}", err))
}

pub fn hide_ui() -> Result<()> {
    let ui = find_ui()?;

    if let Some(child) = ui.first_child() {
        ui.remove_child(&child)
            .map(|_removec_child| ())
            .map_err(|err| anyhow!("Failed to remove child {:#?}", err))
            .and_then(|_unit| {
                Browser::canvas()?
                    .focus()
                    .map_err(|err| anyhow!("Could not set focus to canvas {:#?} ", err))
            })
    } else {
        Ok(())
    }
}

fn find_ui() -> Result<Element> {
    Browser::document().and_then(|doc| {
        doc.get_element_by_id("ui")
            .ok_or_else(|| anyhow!("UI element not found"))
    })
}

pub fn find_html_element_by_id(id: &str) -> Result<HtmlElement> {
    Browser::document()
        .and_then(|doc: Document| {
            doc.get_element_by_id(id)
                .ok_or_else(|| anyhow!("Element with id {} not found", id))
        })
        .and_then(|element| {
            element
                .dyn_into::<HtmlElement>()
                .map_err(|err| anyhow!("Could not cast into HtmlElement {:#?}", err))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_error_loading_json() {
        let json = Browser::fetch_json("notthere.json").await;
        assert_eq!(json.is_err(), true);
    }
}
