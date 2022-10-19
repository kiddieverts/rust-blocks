extern crate glm;
extern crate glium;
extern crate stopwatch;

use glium::{glutin::{self, dpi::{Position, LogicalPosition}}};

use constants::{WINDOW_WIDTH, WINDOW_HEIGHT, TIME_BETWEEN_FRAMES};
pub mod block;
pub mod camera;
pub mod chunk;
pub mod constants;
pub mod shader;
pub mod tex;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let shader = shader::Shader::new(WINDOW_WIDTH, WINDOW_HEIGHT, &event_loop);
    let mut camera = camera::Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut stopwatch = stopwatch::Stopwatch::new();

    // We only have one chunk for now
    let chunk = chunk::Chunk::new();

    // We only have one texture for now
    let texture = tex::get_texture(&shader.display);

    let mut time_passed = 0.0; // <--

    let mut calc = camera.get_calculation();

    stopwatch.start();

    let vertices = chunk.get_vertices();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => camera.process_keyboard(input.virtual_keycode),
                    glutin::event::WindowEvent::CursorMoved { position, .. } => camera.process_mouse(position.x, position.y),
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
                // Main program

                time_passed += 0.02; // <--
                camera.delta_time = time_passed.clone() - camera.last_frame;
                camera.last_frame = time_passed.clone();

                shader.render_block(&calc, &texture, &vertices);
               
                // Check if more than ~16 ms. has passed
                if (stopwatch.elapsed_ms() as u64) >= TIME_BETWEEN_FRAMES {
                    stopwatch.restart();
                    calc = camera.get_calculation();

                    // Reset mouse cursor to the center of the screen
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

#[derive(PartialEq)]
pub enum BlockId {
    Air,
    Plank,
}

impl BlockId {
    pub fn is_visible(&self) -> bool { self != &BlockId::Air }
    pub fn is_transparent(&self) -> bool { self == &BlockId::Air }
}

pub struct Sides {
    top: bool,
    bottom: bool,
    front: bool,
    back: bool,
    left: bool,
    right: bool
}