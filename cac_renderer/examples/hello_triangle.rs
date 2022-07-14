use std::time::Instant;

use cac_renderer::{
    mat2, vec2, vec3, vec4,
    AttributeSemantic::{Color, Position},
    Backend, BufferAttributes, BufferStorage, BufferUsage, ClearFlags, Color32, FrameTimer,
    LayoutStorage, MaterialProperty, Mesh, ProgramStorage, PropertyId, PropertyValue, Renderer,
    ShaderStorage, VertexAttribute,
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

            uniform vec4 color[3];
            uniform mat2 tint;

            in vec3 frag_color;
            void main() 
            {
                result = vec4(color[0].a, color[1].g, tint[1][1], 1.0);
            }"##;

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

    let mut renderer = Renderer::new(&window, (4, 5))?;
    log::info!("{}", renderer.context_description());

    let render_target = renderer.screen_target();
    render_target.set_clear_color(Color32::DARK_JUNGLE_GREEN);
    render_target.set_clear_flags(ClearFlags::COLOR);

    #[repr(C)]
    struct Vertex {
        position: (f32, f32, f32),
        color: (f32, f32, f32, f32),
    }

    impl Vertex {
        const fn new(x: f32, y: f32, z: f32) -> Self {
            Self {
                position: (x, y, z),
                color: (1.0, 1.0, 0.0, 0.0),
            }
        }
    }

    #[rustfmt::skip]
    const VERTICES: [Vertex; 4] = [
        Vertex::new( -0.5, -0.5, 0.0),  // bottom left
        Vertex::new( -0.5,  0.5, 0.0),  // top left
        Vertex::new(  0.5,  0.5, 0.0),  // top right
        Vertex::new(  0.5, -0.5, 0.0),   // bottom right
    ];

    const INDICES: [u8; 6] = [0, 1, 2, 3, 0, 1];

    // Can either pass in a float array or just convert a packed struct into raw data.
    // TODO: test with u8
    let vertex_data = unsafe { std::slice::from_raw_parts(VERTICES.as_ptr() as *const f32, 28) };

    let buffers = &mut renderer.buffers;
    let vertex_buffer = buffers.new_vertex(vertex_data, BufferUsage::StaticRead)?;
    let index_buffer = buffers.new_index(&INDICES, BufferUsage::StaticRead)?;

    let ibo = buffers.get(index_buffer).unwrap();

    let vertex_layout = renderer.layouts.new(&[
        BufferAttributes {
            buffer: buffers.get(vertex_buffer).unwrap(),
            attributes: &[
                VertexAttribute {
                    stride: std::mem::size_of::<Vertex>(),
                    semantic: Position,
                    normalized: false,
                    offset: 0,
                },
                VertexAttribute {
                    stride: std::mem::size_of::<Vertex>(),
                    semantic: Color(0),
                    normalized: false,
                    offset: std::mem::size_of::<f32>() * 3,
                },
            ],
            offset: 0,
        },
        BufferAttributes {
            buffer: ibo,
            attributes: &[],
            offset: 0,
        },
    ])?;
    let mut triangle_mesh = Mesh {
        vertex_layout,
        start_index: 0,
        count: 3,
        primitive: cac_renderer::Primitive::Triangles,
    };

    let vs = renderer.shaders.new_vertex(VS_SOURCE)?;
    let fs = renderer.shaders.new_fragment(FS_SOURCE)?;

    let vertex_shader = renderer.shaders.get(vs).unwrap();
    let fragment_shader = renderer.shaders.get(fs).unwrap();

    let program = renderer
        .programs
        .new_program(vertex_shader, fragment_shader)?;

    renderer.draw(triangle_mesh);

    let material = renderer.create_material(
        program,
        &[
            MaterialProperty::new(
                "color",
                &[vec4(0.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 0.0, 0.0)],
            ),
            MaterialProperty::new("tint", &[mat2(vec2(0.0, 1.0), vec2(1.0, 1.0))]),
        ],
    )?;

    renderer.use_material(material);

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
                    renderer.draw(triangle_mesh);
                }
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
