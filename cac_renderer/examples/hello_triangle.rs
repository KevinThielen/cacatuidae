use cac_renderer::Renderer;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> anyhow::Result<(), anyhow::Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Example: Hello Triangle")
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut renderer = Renderer::new(&window).unwrap();

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
