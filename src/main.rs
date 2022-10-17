// https://learnopengl.com/Getting-started/Camera
// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp

#[macro_use]
#[allow(unused_imports)]

extern crate glm;
extern crate glium;
extern crate stopwatch;

use std::{collections::{HashMap, HashSet}};
use glium::{glutin::{self, dpi::{Position, LogicalPosition}}, Surface};

pub mod block;
pub mod shader;
pub mod camera;
pub mod tex;

#[derive(PartialEq)]
enum BlockId {
    Air,
    Plank,
}

impl BlockId {
    pub fn is_visible(&self) -> bool { self != &BlockId::Air }
    pub fn is_transparent(&self) -> bool { self == &BlockId::Air }
}

type Chunk = HashMap<i32, BlockId>;

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
    const CHUNK_HEIGHT: i32 = 8;
    const CHUNK_WIDTH: i32 = 4;

    let mut time_increment: f32 = 0.0;

    let event_loop = glutin::event_loop::EventLoop::new();
    
    let shader = shader::Shader::new(WINDOW_WIDTH, WINDOW_HEIGHT, &event_loop);
    let mut camera = camera::Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let texture = tex::get_texture(&shader.display);

    let mut stopwatch = stopwatch::Stopwatch::new();

    stopwatch.reset();
    stopwatch.start();

    let chunk = get_chunk(CHUNK_WIDTH, CHUNK_HEIGHT);
    let vertexes = get_vertexes(CHUNK_WIDTH, CHUNK_HEIGHT, chunk);

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

fn get_chunk(chunk_width: i32, chunk_height: i32) -> HashMap<i32, BlockId> {
    let mut chunk: HashMap<i32, BlockId> = HashMap::new();

    for i in 0..chunk_width*chunk_width*chunk_height {
        if i % 3 == 0 {
            chunk.insert(i, BlockId::Plank);
        }
        else {
            chunk.insert(i, BlockId::Air);
        }
    }

    return chunk
}

fn get_vertexes(chunk_weight: i32, chunk_height: i32, chunk: Chunk) -> Vec<Vertex> {
    let mut v: Vec<Vertex> = vec![];
    let mut i = 0;

    for y in 0..chunk_height {
        for z in 0..chunk_weight {
            for x in 0..chunk_weight {
                let sides = get_sides(i, &chunk, chunk_weight, chunk_height);
                let block = block::Block::get_cube(x, y, -z, sides);
                for item in 0..block.len() {
                    if chunk[&i].is_visible() {
                        v.push(block[item]);
                    }
                }
                i = i +1;
            }
        }
    }
    return v;
}

fn get_sides(i: i32, chunk: &Chunk, chunk_width: i32, chunk_height: i32 ) -> Sides {
    let max = chunk_width * chunk_width * chunk_height;

    let get_left = || {
        if i % chunk_width == 0 {
            return true;
        }
        return chunk[&(i -1)].is_transparent();
    };

    let get_right = || {
        if (i+1) % chunk_width == 0 {
            return true;
        }
        return chunk[&(i +1)].is_transparent();
    };

    let get_front = || {
        let res = match i % (chunk_width * chunk_width) {
            0 => true,
            1 => true,
            2 => true,
            3 => true,
            _ => false
        };
        if res == true {
            return res;
        }
        return chunk[&(i - chunk_width)].is_transparent();
    };

    let get_back = || {
        let mut cnt = 0;
        let mut val: HashSet<i32> = HashSet::new();

        while cnt < max {
            val.insert(cnt + (chunk_width * chunk_width) - chunk_width);
            val.insert(cnt + (chunk_width * chunk_width) - chunk_width + 1);
            val.insert(cnt + (chunk_width * chunk_width) - chunk_width + 2);
            val.insert(cnt + (chunk_width * chunk_width) - chunk_width + 3);
            cnt += chunk_width * chunk_width;
        }

        if val.contains(&i) {
            return true;
        }

        if i + chunk_width >= max { 
            return false;
        }

        return chunk[&(i + chunk_width)].is_transparent();
    };

    let get_top = || {
        if i > (max - chunk_width * chunk_width - 1) {
            return true;
        }

        if (i + chunk_width * chunk_width) >= max { 
            return false;
        }
     
        return chunk[&(i + chunk_width * chunk_width)].is_transparent();
    };

    let get_bottom = || {
        if i > max - chunk_width*chunk_width {
            return true;
        }

        if i - chunk_width * chunk_width < 0 {
            return false;
        }

        return chunk[&(i - chunk_width * chunk_width)].is_transparent();
    };

    return Sides { top: get_top(), bottom: get_bottom(), front: get_front(), back: get_back(), left: get_left(), right: get_right() };
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],   
    brightness: f32 
}
 