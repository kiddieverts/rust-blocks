// https://learnopengl.com/Getting-started/Camera
// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp

#[macro_use]
#[allow(unused_imports)]

extern crate glm;
extern crate glium;

use glium::{glutin, Surface, uniform};
use glium::{texture::SrgbTexture2d};
use glm::{cos, sin};
use std::io::Cursor;
pub mod block;
pub mod shader;
pub mod camera;

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;

    // let mut first_mouse = true;
    // let mut yaw   = -90.0;	// yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
    // let mut pitch =  0.0;
    // let mut last_x =  WINDOW_WIDTH as f64 / 2.0;
    // let mut last_y = WINDOW_HEIGHT as f64 / 2.0;

    let event_loop = glutin::event_loop::EventLoop::new();

    let shader = shader::Display::new(WINDOW_WIDTH, WINDOW_HEIGHT, &event_loop);

    let texture = get_texture(&shader.display);
    let blk = block::Block::new(&shader.display);

    // let mut camera_pos = glm::vec3(1.0, 0.0, 3.0);
    // let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
    // let camera_up = glm::vec3(0.0, 1.0, 0.0);


    let mut cam = camera::Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    // let x = 0.05;
    // let camera_speed = glm::vec3(x, x, x);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        // let key = match input.virtual_keycode {
                        //     Some(key) => key,
                        //     None => return
                        // };
                        // match key {
                        //     glutin::event::VirtualKeyCode::W => camera_pos = camera_pos + camera_speed * camera_front,
                        //     glutin::event::VirtualKeyCode::S => camera_pos = camera_pos - camera_speed * camera_front,
                        //     glutin::event::VirtualKeyCode::A => camera_pos = camera_pos - glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed,
                        //     glutin::event::VirtualKeyCode::D => camera_pos = camera_pos + glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed,
                        //     _ => ()
                        // }
                        cam.process_keyboard(input.virtual_keycode);
                    },
                    glutin::event::WindowEvent::CursorMoved { position, .. } => {
                        cam.process_mouse(position);
                        // let xpos = position.x;
                        // let ypos = position.y;

                        // if first_mouse
                        // {
                        //     last_x = xpos;
                        //     last_y = ypos;
                        //     first_mouse = false;
                        // }
                    
                        // let mut xoffset = xpos - last_x;
                        // let mut yoffset = last_y - ypos; // reversed since y-coordinates go from bottom to top
                        // last_x = xpos;
                        // last_y = ypos;
                    
                        // let sensitivity = 0.1; // change this value to your liking
                        // xoffset *= sensitivity;
                        // yoffset *= sensitivity;
                    
                        // yaw += xoffset;
                        // pitch += yoffset;
                    
                        // // make sure that when pitch is out of bounds, screen doesn't get flipped
                        // if pitch > 89.0 {
                        //     pitch = 89.0;
                        // }
                        // if pitch < -89.0 {
                        //     pitch = -89.0;
                        // }

                        // let _x = (cos(glm::radians(yaw)) * cos(glm::radians(pitch))) as f32;
                        // let _y = sin(glm::radians(pitch)) as f32;
                        // let _z = (sin(glm::radians(yaw)) * cos(glm::radians(pitch))) as f32;
                    
                        // let front = glm::vec3(_x, _y, _z);
                        // camera_front = glm::normalize(front);
                    }

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
                let mut target = shader.display.draw();
                target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            
                // let model = [
                //     [1.0, 0.0, 0.0, 0.0],
                //     [0.0, 1.0, 0.0, 0.0],
                //     [0.0, 0.0, 1.0, 0.0],
                //     [0.0, 0.0, 0.0, 1.0f32],
                // ];
           
                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                };

                // let view_temp = glm::ext::look_at(cam.camera_pos, cam.camera_pos + cam.camera_front, cam.camera_up);

                // let view = [
                //     [view_temp.c0.x, view_temp.c0.y, view_temp.c0.z, view_temp.c0.w],
                //     [view_temp.c1.x, view_temp.c1.y, view_temp.c1.z, view_temp.c1.w],
                //     [view_temp.c2.x, view_temp.c2.y, view_temp.c2.z, view_temp.c2.w],
                //     [view_temp.c3.x, view_temp.c3.y, view_temp.c3.z, view_temp.c3.w]
                // ];

                // let fov = 45.0;

                // let projection_temp = glm::ext::perspective(glm::radians(fov), 1024.0 / 720.0, 0.1, 100.0);

                // let perspective = [
                //     [projection_temp.c0.x as f32, projection_temp.c0.y as f32, projection_temp.c0.z as f32, projection_temp.c0.w as f32],
                //     [projection_temp.c1.x as f32, projection_temp.c1.y as f32, projection_temp.c1.z as f32, projection_temp.c1.w as f32],
                //     [projection_temp.c2.x as f32, projection_temp.c2.y as f32, projection_temp.c2.z as f32, projection_temp.c2.w as f32],
                //     [projection_temp.c3.x as f32, projection_temp.c3.y as f32, projection_temp.c3.z as f32, projection_temp.c3.w as f32]
                // ];

                let calc = cam.get_calculation();

                let uniforms = uniform!{ 
                    model: calc.model, 
                    view: calc.view, 
                    perspective: calc.perspective, 
                    tex: &texture,
                };
           
                target.draw(&blk.buffer, &blk.indices, &shader.program, &uniforms, &params).unwrap();
                target.finish().unwrap();
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