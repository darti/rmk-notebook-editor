mod canvas;

use yew::{html, Component, Context, Html};

use gloo_file::File;

use crate::canvas::Canvas;

type Chunks = bool;

pub enum Msg {
    Loaded(String, String),
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>, Chunks),
    ToggleReadBytes,
}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {"toto"}
                <Canvas />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
