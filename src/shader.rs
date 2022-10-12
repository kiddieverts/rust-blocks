use glium::glutin::dpi::{Position, LogicalPosition};
use glium::glutin::window::CursorGrabMode;
use glium::glutin;

pub struct Display
{
    pub program: glium::Program,
    pub display: glium::Display
}

impl Display
{
    pub fn new(WINDOW_WIDTH: u32, WINDOW_HEIGHT: u32, event_loop: &glutin::event_loop::EventLoop<()>) -> Display {
        let wb = glutin::window::WindowBuilder::new()
        .with_title(format!("Minecraft RS"))
        .with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, event_loop).unwrap();

        display.gl_window().window().set_cursor_grab(CursorGrabMode::Locked).expect("Could not lock cursor");
        display.gl_window().window().set_cursor_visible(false);

        let position = Position::Logical(LogicalPosition::new(WINDOW_WIDTH as f64 / 2.0, WINDOW_HEIGHT as f64  / 2.0));
        display.gl_window().window().set_cursor_position(position).expect("Set cursor position fialed");

        let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                mat4 modelview = view * model;
                v_tex_coords = tex_coords;
                gl_Position = perspective * modelview * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 150

            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Display {
            program: program,
            display: display
        }
    }
}