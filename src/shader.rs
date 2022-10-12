use std::io::Cursor;

use glium::glutin::dpi::{Position, LogicalPosition};
use glium::glutin::window::CursorGrabMode;
use glium::texture::SrgbTexture2d;
use glium::{glutin, Surface, uniform};

use crate::camera::{Camera, ShaderCalculation};
use crate::{get_texture, block};

pub struct Shader
{
    pub program: glium::Program,
    pub display: glium::Display
}

impl Shader
{
    pub fn new(window_width: u32, window_height: u32, event_loop: &glutin::event_loop::EventLoop<()>) -> Shader {
        let wb = glutin::window::WindowBuilder::new()
            .with_title(format!("Minecraft RS"))
            .with_inner_size(glutin::dpi::LogicalSize::new(window_width, window_height));
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, event_loop).unwrap();

        display.gl_window().window().set_cursor_grab(CursorGrabMode::Locked).expect("Could not lock cursor");
        display.gl_window().window().set_cursor_visible(false);

        let position = Position::Logical(LogicalPosition::new(window_width as f64 / 2.0, window_height as f64  / 2.0));
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

        Shader {
            program: program,
            display: display
        }
    }

    pub fn draw(&self, calc: &ShaderCalculation) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
    
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let texture = self.get_texture();
        let blk = block::Block::new(&self.display);

        let uniforms = uniform!{ 
            model: calc.model, 
            view: calc.view, 
            perspective: calc.perspective, 
            tex: &texture,
        };
   
        target.draw(&blk.buffer, &blk.indices, &self.program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    }

    fn get_texture(&self) -> SrgbTexture2d{ 
        let image = image::load(Cursor::new(&include_bytes!("../assets/dirt.png")), image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    
        return glium::texture::SrgbTexture2d::new(&self.display, image).unwrap();
    }
}