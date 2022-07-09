use cac_renderer::{Color32, Renderer};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> anyhow::Result<(), anyhow::Error> {
    pretty_env_logger::env_logger::init_from_env(
        pretty_env_logger::env_logger::Env::default()
            .default_filter_or("hello_triangle,cac_renderer=trace"),
    );

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Example: Hello Triangle")
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut renderer = Renderer::new(&window).unwrap();
    log::info!("{}", renderer.context_description());

    let render_target = renderer.screen_target();
    render_target.set_clear_color(Color32::DARK_JUNGLE_GREEN);

    //let [meshbuffer] = MeshBuffer::with_position_uv(data);
    //let [meshbuffer] = MeshBuffer::with_position_uv_and_color(data, data2);

    //let mesh_handle = renderer.create_mesh([MeshBuffer]);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                //render
                renderer.update();
            }
            Event::WindowEvent {
                window_id: _window,
                event: WindowEvent::CloseRequested,
            } => *control_flow = ControlFlow::Exit,
            _ => window.request_redraw(),
        }
    });
}
