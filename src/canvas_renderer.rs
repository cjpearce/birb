use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::tracer::Tracer;

pub struct CanvasRenderer {
    width: usize,
    height: usize,
    canvas: HtmlCanvasElement,
    tick_ms: f64
}

impl CanvasRenderer {
    pub fn new(canvas_id: &str) -> CanvasRenderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let width = canvas.client_width() as usize;
        let height = canvas.client_height() as usize;
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        CanvasRenderer {
            width,
            height,
            canvas,
            tick_ms: 50.0
        }
    }

    pub fn start(&self, mut tracer: Tracer) {
        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let mut data = vec![0u8; self.width * self.height * 4];

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            self.width as u32,
            self.height as u32,
        )
        .unwrap();

        let performance = window()
            .performance()
            .expect("performance should be available");
        
        let tick_ms = self.tick_ms;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let start = performance.now();
            let end = start + tick_ms;

            loop {
                tracer.update(&mut data);
                if performance.now() > end {
                    break;
                }
            }
            context
                .put_image_data(&image_data, 0.0, 0.0)
                .expect("should have a value");

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
