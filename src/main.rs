// https://learnopengl.com/Getting-started/Camera
// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp

#[macro_use]
#[allow(unused_imports)]

extern crate glm;
extern crate glium;
extern crate stopwatch;

use glium::{glutin::{self, dpi::{Position, LogicalPosition}}, Surface};

pub mod block;
pub mod shader;
pub mod camera;
pub mod tex;

pub mod chunk;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum BlockId {
    Air,
    Plank,
}

impl BlockId {
    pub fn is_visible(&self) -> bool { self != &BlockId::Air }
    pub fn is_transparent(&self) -> bool { self == &BlockId::Air }
}

#[derive(Debug)]
pub struct Sides {
    top: bool,
    bottom: bool,
    front: bool,
    back: bool,
    left: bool,
    right: bool
}

fn main() {
    const WINDOW_WIDTH: u32 = 1280;  
    const WINDOW_HEIGHT: u32 = 720;  
    const TIME_BETWEEN_FRAMES: u64 = 1000 / 60;

    let mut time_increment: f32 = 0.0;

    let event_loop = glutin::event_loop::EventLoop::new();
    
    let shader = shader::Shader::new(WINDOW_WIDTH, WINDOW_HEIGHT, &event_loop);
    let mut camera = camera::Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let texture = tex::get_texture(&shader.display);

    let mut stopwatch = stopwatch::Stopwatch::new();

    stopwatch.reset();
    stopwatch.start();

    let chunk = chunk::Chunk::new();

    let vertexes = chunk::Chunk::get_vertices(chunk);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => camera.process_keyboard(input.virtual_keycode),
                    glutin::event::WindowEvent::CursorMoved { position, .. } => camera.process_mouse(position.x / 2.0, position.y / 2.0),
                    glutin::event::WindowEvent::CloseRequested => {  *control_flow = glutin::event_loop::ControlFlow::Exit; return; },
                _ => (),
            }},
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },

            glutin::event::Event::MainEventsCleared => {
                // Main program

                camera.delta_time = time_increment.clone() - camera.last_frame;
                camera.last_frame = time_increment.clone();

                let mut target = shader.display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            
                time_increment += 0.02;
                
                let buffer = glium::VertexBuffer::new(&shader.display, &vertexes).unwrap();
                target = shader.render_block(&camera, &texture, target, &buffer);  

                target.finish().unwrap();

                if (stopwatch.elapsed_ms() as u64) >= TIME_BETWEEN_FRAMES {
                    stopwatch.reset();
                    stopwatch.start();

                    let position = Position::Logical(LogicalPosition::new(camera.window_width as f64  / 2.0, camera.window_height as f64  / 2.0));
                    shader.display.gl_window().window().set_cursor_position(position).expect("set cursor position faild");
                }
            },
            _ => (),
        }
    });
}
 
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],   
    brightness: f32 
}