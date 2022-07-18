use cac_renderer::{
    math::*,
    AttributeSemantic::{Color, Position},
    Backend, Buffer, BufferAttributes, BufferUsage, ClearFlags, Color32, FrameTimer,
    MaterialProperty, Mesh, Renderer, Shader, ShaderProgram, VertexLayout,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const VS_SOURCE: &str = r##"
            #version 450
            layout(location = 0) in vec3 pos;
            layout(location = 5)  in vec4 color;


            out vec3 frag_color;
            void main() 
            {
                frag_color = color.xyz;
                gl_Position = vec4(pos, 1.0);
            }"##;

const FS_SOURCE: &str = r##"
            #version 450
            out vec4 result;

            uniform vec4 color;
            uniform mat2 tint;

            in vec3 frag_color;
            void main() 
            {
                result = vec4(frag_color, 1.0) * vec4(color.a, color.g, tint[1][1], 1.0);
            }"##;

#[repr(C)]
struct Vertex {
    position: Vec3,
    color: Color32,
}

impl Vertex {
    const fn new(position: Vec3, color: Color32) -> Self {
        Self { position, color }
    }
}

#[rustfmt::skip]
    const VERTICES: [Vertex; 4] = [
        Vertex::new(vec3(-0.5, -0.5, 0.0), Color32::RED),  // bottom left
        Vertex::new(vec3( -0.5,  0.5, 0.0), Color32::GAINSBORO),  // top left
        Vertex::new(vec3(  0.5,  0.5, 0.0), Color32::UNITY_YELLOW), // top right
        Vertex::new(vec3(  0.5, -0.5, 0.0), Color32::PERSIAN_INDIGO),
    ];

const INDICES: [u8; 6] = [0, 1, 2, 3, 0, 1];

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

    let mut ctx = Renderer::new(&window, (4, 5))?;
    log::info!("{}", ctx.context_description());

    let render_target = ctx.screen_target();
    render_target.set_clear_color(Color32::DARK_JUNGLE_GREEN);
    render_target.set_clear_flags(ClearFlags::COLOR);

    // Can either pass in a float array or just convert a packed struct into raw data.
    let vertex_data = unsafe { std::slice::from_raw_parts(VERTICES.as_ptr() as *const f32, 28) };

    let vertex_buffer = Buffer::with_vertex(&mut ctx, vertex_data, BufferUsage::StaticRead)?;
    let index_buffer = Buffer::with_index(&mut ctx, &INDICES, BufferUsage::StaticRead)?;

    let vertex_layout = VertexLayout::new(
        &mut ctx,
        &[
            BufferAttributes::with_semantics(vertex_buffer, 0, &[Position, Color(0)]),
            BufferAttributes::with_index(index_buffer, 0),
        ],
    )?;

    let mut triangle_mesh = Mesh {
        vertex_layout,
        start_index: 0,
        count: 3,
        primitive: cac_renderer::Primitive::Triangles,
    };

    let vertex_shader = Shader::with_vertex(&mut ctx, VS_SOURCE)?;
    let fragment_shader = Shader::with_fragment(&mut ctx, FS_SOURCE)?;

    let program = ShaderProgram::new(&mut ctx, vertex_shader, fragment_shader)?;

    let material = {
        ctx.create_material(
            program,
            &[
                MaterialProperty::new("color", &[vec4(1.0, 1.0, 1.0, 1.0)]),
                MaterialProperty::new("tint", &[mat2(vec2(1.0, 1.0), vec2(1.0, 1.0))]),
            ],
        )?
    };

    let mut timer = FrameTimer::with_repeated(0.5);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                //render
                if timer.tick_done() {
                    triangle_mesh.start_index += 1;
                    if triangle_mesh.start_index >= 4 {
                        triangle_mesh.start_index = 0;
                    }
                }

                ctx.draw(triangle_mesh, material, &[]);

                ctx.update();
            }
            Event::WindowEvent {
                window_id: _window,
                event: WindowEvent::CloseRequested,
            } => *control_flow = ControlFlow::Exit,
            _ => window.request_redraw(),
        }
    });
}
