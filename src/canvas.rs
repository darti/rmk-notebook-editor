use piet::kurbo::{Line, Point, Rect, Shape};
use piet::{Color, RenderContext};
use piet_common::{Brush, WebRenderContext};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Component, Context, Html, NodeRef};

pub enum Msg {
    Render,
}

pub struct Canvas {
    canvas_ref: NodeRef,
}

impl Component for Canvas {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
           <canvas ref={self.canvas_ref.clone()}/>

        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let window = window().unwrap();

        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let mut piet_context = WebRenderContext::new(context, window);
        let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 100.0));
        piet_context.stroke(line, &Color::BLACK, 4.);

        piet_context.finish().unwrap();
    }
}
