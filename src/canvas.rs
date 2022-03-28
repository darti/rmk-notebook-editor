use pathfinder_canvas::{vec2f, Canvas as PathfinderCanvas, CanvasFontContext, Path2D, RectF};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::vec2i;
use pathfinder_renderer::{
    concurrent::executor::SequentialExecutor,
    gpu::{
        options::{DestFramebuffer, RendererMode, RendererOptions},
        renderer::Renderer,
    },
    options::BuildOptions,
};
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use pathfinder_webgl::WebGlDevice;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
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
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        let framebuffer_size = vec2i(canvas.width() as i32, canvas.height() as i32);

        let pathfinder_device = WebGlDevice::new(context);

        let mode = RendererMode::default_for_device(&pathfinder_device);

        let options = RendererOptions {
            dest: DestFramebuffer::full_window(framebuffer_size),
            background_color: Some(ColorF::white()),
            ..RendererOptions::default()
        };

        let resource_loader = EmbeddedResourceLoader::new();
        let mut renderer = Renderer::new(pathfinder_device, &resource_loader, mode, options);

        let font_context = CanvasFontContext::from_system_source();
        let mut canvas =
            PathfinderCanvas::new(framebuffer_size.to_f32()).get_context_2d(font_context);

        // Set line width.
        canvas.set_line_width(10.0);

        // Draw walls.
        canvas.stroke_rect(RectF::new(vec2f(75.0, 140.0), vec2f(150.0, 110.0)));

        // Draw door.
        canvas.fill_rect(RectF::new(vec2f(130.0, 190.0), vec2f(40.0, 60.0)));

        // Draw roof.
        let mut path = Path2D::new();
        path.move_to(vec2f(50.0, 140.0));
        path.line_to(vec2f(150.0, 60.0));
        path.line_to(vec2f(250.0, 140.0));
        path.close_path();
        canvas.stroke_path(path);

        // Render the canvas to screen.
        let mut scene = canvas.into_canvas().into_scene();

        scene.build_and_render(&mut renderer, BuildOptions::default(), SequentialExecutor);
    }
}
