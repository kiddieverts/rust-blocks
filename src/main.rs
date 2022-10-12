// https://learnopengl.com/Getting-started/Camera
// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp

#[macro_use]
#[allow(unused_imports)]

extern crate glm;
extern crate glium;

use glium::{glutin, Surface, uniform};
use glium::{texture::SrgbTexture2d};
use std::io::Cursor;

pub mod block;
pub mod shader;
pub mod camera;

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;

    let event_loop = glutin::event_loop::EventLoop::new();
    let shader = shader::Shader::new(WINDOW_WIDTH, WINDOW_HEIGHT, &event_loop);

    // let texture = get_texture(&shader.display);
    // let blk = block::Block::new(&shader.display);

    let mut cam = camera::Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => cam.process_keyboard(input.virtual_keycode),
                    glutin::event::WindowEvent::CursorMoved { position, .. } => cam.process_mouse(position),
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                _ => (),
            }},
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },

            glutin::event::Event::MainEventsCleared => {
                // let mut target = shader.display.draw();
                // target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            
                // let params = glium::DrawParameters {
                //     depth: glium::Depth {
                //         test: glium::draw_parameters::DepthTest::IfLess,
                //         write: true,
                //         .. Default::default()
                //     },
                //     .. Default::default()
                // };

                // let calc = cam.get_calculation();

                // let uniforms = uniform!{ 
                //     model: calc.model, 
                //     view: calc.view, 
                //     perspective: calc.perspective, 
                //     tex: &texture,
                // };
           
                // target.draw(&blk.buffer, &blk.indices, &shader.program, &uniforms, &params).unwrap();
                // target.finish().unwrap();

                shader.draw(&cam.get_calculation());
            },
            _ => (),
        }
    });
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],      
}


fn get_texture(display: &glium::Display) -> SrgbTexture2d{ 
    let image = image::load(Cursor::new(&include_bytes!("../assets/dirt.png")), image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    return glium::texture::SrgbTexture2d::new(display, image).unwrap();
}