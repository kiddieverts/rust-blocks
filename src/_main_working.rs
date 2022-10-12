// // https://learnopengl.com/Getting-started/Camera

// #[macro_use]
// #[allow(unused_imports)]

// extern crate glm;

// extern crate glium;
// use glium::{glutin, Surface, uniform, implement_vertex};
// use glium::{texture::SrgbTexture2d, VertexBuffer, index::NoIndices};
// use std::io::Cursor;
// use std::time::Instant;
// use glutin::dpi::{LogicalPosition, Position};

// fn normalize(arr1: [f32; 3]) -> [f32; 3]{
//     // Make unit vector 

//     let mut result: [f32; 3] = [0.0,0.0,0.0];

//     let magnitude = (f32::powi(arr1[0], 2) + f32::powi(arr1[1], 2) + f32::powi(arr1[2], 2)).sqrt();

//     result[0] = arr1[0]/magnitude;
//     result[1] = arr1[1]/magnitude;
//     result[2] = arr1[2]/magnitude;

//     result
// }

// fn sub(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
//     // Subtract two vectors
//     let mut result: [f32; 3] = [0.0,0.0,0.0];

//     result[0] = arr1[0] - arr2[0];
//     result[1] = arr1[1] - arr2[1];
//     result[2] = arr1[2] - arr2[2];

//     result
// }

// fn add(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
//     // Add two vectors
//     let mut result: [f32; 3] = [0.0,0.0,0.0];

//     result[0] = arr1[0] + arr2[0];
//     result[1] = arr1[1] + arr2[1];
//     result[2] = arr1[2] + arr2[2];

//     result
// }

// fn cross(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3]{
//     let mut result: [f32; 3] = [0.0, 0.0, 0.0];

//     let i = arr1[1] * arr2[2] - arr1[2] * arr2[1];
//     let j = arr1[2] * arr2[0] - arr1[0] * arr2[2];
//     let k = arr1[0] * arr2[1] - arr1[1] * arr2[0];

//     result[0] = i;
//     result[1] = (-1.0) * j;
//     result[2] = k;

//     result
// }


// fn main() {
//     const REFRESH_RATE: i64 = 1000 / 20;
//     const WINDOW_WIDTH: u32 = 1280;
//     const WINDOW_HEIGHT: u32 = 720;

//     let mut pitch = 0.0;
//     let mut yaw = 0.0;

//     // let mut camera_pos = [0.0, 0.0, 0.0];

//     // let mut camera_target = [0.0, 0.0, 0.0];

//     // let camera_direction = normalize(sub(camera_pos, camera_target));

//     // let up = [0.0, 1.0, 0.0];

//     // let camera_right = normalize(cross(up, camera_direction));

//     // let camera_up = cross(camera_direction, camera_right);

//     // let center = camera_pos + camera_fro

//     // let view_temp = glm::ext::look_at(
//     //     glm::vec3(camera_pos[0], camera_pos[1], camera_pos[2]), 
//     //     glm::vec3(center[0], center[1], center[2]), 
//     //     // glm::vec3(self.camera_up[0], self.camera_up[1], self.camera_up[2])
//     // );


//     // view = glm::lookAt(cameraPos, cameraPos + cameraFront, cameraUp);

   

//     let event_loop = glutin::event_loop::EventLoop::new();
//     let wb = glutin::window::WindowBuilder::new()
//         .with_title(format!("Minecraft RS"))
//         .with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
//     let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
//     let display = glium::Display::new(wb, cb, &event_loop).unwrap();

//     let vertex_shader_src = r#"
//         #version 150

//         in vec3 position;
//         in vec2 tex_coords;
//         out vec2 v_tex_coords;

//         uniform mat4 perspective;
//         uniform mat4 view;
//         uniform mat4 model;

//         void main() {
//             mat4 modelview = view * model;
//             v_tex_coords = tex_coords;
//             gl_Position = perspective * modelview * vec4(position, 1.0);
//         }
//     "#;

//     let fragment_shader_src = r#"
//         #version 150

//         in vec2 v_tex_coords;
//         out vec4 color;

//         uniform sampler2D tex;

//         void main() {
//             color = texture(tex, v_tex_coords);
//         }
//     "#;

//     let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
//     let mut a: f32 = 2.0;
//     let mut b: f32 = -1.0;
//     let mut c: f32 = 1.0;

//     let mut d: f32 = -2.0;
//     let mut e: f32 = 1.0;
//     let mut f: f32 = 1.0;

//     let mut g: f32 = 0.0;
//     let mut h: f32 = 1.0;
//     let mut j: f32 = 0.5;

//     let texture = get_texture(&display);
//     let cb = Block::new(&display);

//     let mut time_stamp = Instant::now();

//     let nb = 0.08;

//     event_loop.run(move |event, _, control_flow| {
//         let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
//         *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

//         match event {
//             glutin::event::Event::WindowEvent { event, .. } => {
//                 match event {
//                     glutin::event::WindowEvent::KeyboardInput { input, .. } => {
//                         let key = match input.virtual_keycode {
//                             Some(key) => key,
//                             None => return
//                         };
//                         match key {
//                             glutin::event::VirtualKeyCode::Q => a += nb,
//                             glutin::event::VirtualKeyCode::A => a -= nb,
//                             glutin::event::VirtualKeyCode::W => b += nb,
//                             glutin::event::VirtualKeyCode::S => b -= nb,
//                             glutin::event::VirtualKeyCode::E => c += nb,
//                             glutin::event::VirtualKeyCode::D => c -= nb,

//                             glutin::event::VirtualKeyCode::R => d += nb,
//                             glutin::event::VirtualKeyCode::F => d -= nb,
//                             glutin::event::VirtualKeyCode::T => e += nb,
//                             glutin::event::VirtualKeyCode::G => e -= nb,
//                             glutin::event::VirtualKeyCode::Y => f += nb,
//                             glutin::event::VirtualKeyCode::H => f -= nb,

//                             glutin::event::VirtualKeyCode::U => g += nb,
//                             glutin::event::VirtualKeyCode::J => g -= nb,
//                             glutin::event::VirtualKeyCode::I => h += nb,
//                             glutin::event::VirtualKeyCode::K => h -= nb,
//                             glutin::event::VirtualKeyCode::O => j += nb,
//                             glutin::event::VirtualKeyCode::L => j -= nb,
                        
//                             _ => ()
//                         }
//                     }
//                     glutin::event::WindowEvent::CursorMoved { position, .. } => {
//                         let dev_x = WINDOW_WIDTH as f64/2.0 - position.x;
//                         let dev_y = WINDOW_HEIGHT as f64/2.0 - position.y;

//                         yaw += dev_x/10.0;
//                         pitch += dev_y/10.0;

//                         // int dev_x,dev_y;
//                         // dev_x = (width/2)-x;
//                         // dev_y = (height/2)-y;
                    
//                         // /* apply the changes to pitch and yaw*/
//                         // yaw+=(float)dev_x/10.0;
//                         // pitch+=(float)dev_y/10.0;
//                     },
//                     glutin::event::WindowEvent::CloseRequested => {
//                         *control_flow = glutin::event_loop::ControlFlow::Exit;
//                         return;
//                     },
//                 _ => (),
//             }},
//             glutin::event::Event::NewEvents(cause) => match cause {
//                 glutin::event::StartCause::ResumeTimeReached { .. } => (),
//                 glutin::event::StartCause::Init => (),
//                 _ => return,
//             },


//             glutin::event::Event::MainEventsCleared => {
//                 if elapsed_ms(time_stamp) >= REFRESH_RATE {
//                     let position = Position::Logical(LogicalPosition::new(WINDOW_WIDTH as f64  / 2.0, WINDOW_HEIGHT as f64  / 2.0));
//                     display.gl_window().window().set_cursor_position(position).expect("set cursor failed");
//                     time_stamp = Instant::now();
//                 }


//                draw(a, b, c, d, e, f, g, h, j, &display, &texture, &program, &cb.buffer, cb.indices);
//             },
//             _ => (),
//         }
//     });
// }

// fn draw(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, j: f32, display: &glium::Display, texture: &SrgbTexture2d, program: &glium::Program, positions: &VertexBuffer<Vertex>, indices: NoIndices) {
//     let mut target = display.draw();
//     target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

//     let model = [
//         // [1.0, 0.0, 0.0, 0.0],
//         // [0.0, 1.0, 0.0, 0.0],
//         // [0.0, 0.0, 1.0, 0.0],
//         // [0.0, 0.0, 0.0, 1.0f32]
//         [ a.cos(), a.sin(), 0.0, 0.0],
//         [-a.sin(), a.cos(), 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0f32],
//     ];

//     let view = view_matrix(&[a, b, c], &[d, e, f], &[g, h, j]);

//     let (width, height) = target.get_dimensions();
//     let perspective = get_perspective(width, height);

//     let params = glium::DrawParameters {
//         depth: glium::Depth {
//             test: glium::draw_parameters::DepthTest::IfLess,
//             write: true,
//             .. Default::default()
//         },
//         // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
//         .. Default::default()
//     };

//     let camera_pos   = [0.0, 0.0,  3.0];
//     let camera_front = [0.0, 0.0, -1.0];
//     let camera_up    = [0.0, 1.0,  0.0];

//     let center = add(camera_pos, camera_front);

//     let view_temp = glm::ext::look_at(
//         glm::vec3(camera_pos[0], camera_pos[1], camera_pos[2]), 
//         glm::vec3(center[0], center[1], center[2]), 
//         glm::vec3(camera_up[0], camera_up[1], camera_up[2])
//     );

//     let final_view =  [
//         [view_temp.c0.x, view_temp.c0.y, view_temp.c0.z, view_temp.c0.w],
//         [view_temp.c1.x, view_temp.c1.y, view_temp.c1.z, view_temp.c1.w],
//         [view_temp.c2.x, view_temp.c2.y, view_temp.c2.z, view_temp.c2.w],
//         [view_temp.c3.x, view_temp.c3.y, view_temp.c3.z, view_temp.c3.w]
//     ];

//     let uniforms = uniform!{ 
//         model: model, 
//         view: view, 
//         perspective: perspective, 
//         tex: texture,
//     };

//     target.draw(positions, &indices, program, &uniforms, &params).unwrap();
//     target.finish().unwrap();
// }

// fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
//     let f = {
//         let f = direction;
//         let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
//         let len = len.sqrt();
//         [f[0] / len, f[1] / len, f[2] / len]
//     };

//     let s = [
//         up[1] * f[2] - up[2] * f[1],
//         up[2] * f[0] - up[0] * f[2],
//         up[0] * f[1] - up[1] * f[0]];

//     let s_norm = {
//         let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
//         let len = len.sqrt();
//         [s[0] / len, s[1] / len, s[2] / len]
//     };

//     let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
//         f[2] * s_norm[0] - f[0] * s_norm[2],
//         f[0] * s_norm[1] - f[1] * s_norm[0]];

//     let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
//         -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
//         -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

//     [
//         [s_norm[0], u[0], f[0], 0.0],
//         [s_norm[1], u[1], f[1], 0.0],
//         [s_norm[2], u[2], f[2], 0.0],
//         [p[0], p[1], p[2], 1.0],
//     ]
// }

// fn get_perspective(width: u32, height: u32) -> [[f32; 4]; 4]{
//     let aspect_ratio = height as f32 / width as f32;

//     let fov: f32 = 3.141592 / 3.0;
//     let zfar = 1024.0;
//     let znear = 0.1;

//     let f = 1.0 / (fov / 2.0).tan();

//     [
//         [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
//         [         0.0         ,     f ,              0.0              ,   0.0],
//         [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
//         [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
//     ]
// }

// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 3],
//     tex_coords: [f32; 2],      
// }

// struct Block {
//     indices: glium::index::NoIndices,
//     buffer: VertexBuffer<Vertex>,
// }
// impl Block {
//     pub fn new(display: &glium::Display) -> Block {
//         let cb = get_cube(0.0);
//         let buffer = glium::VertexBuffer::new(display, &cb).unwrap();
//         let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

//         Block {
//             indices: indices,
//             buffer: buffer
//         }
//     }
// }

// implement_vertex!(Vertex, position, tex_coords);

// fn get_cube(i: f32) -> Vec<Vertex> {
//     let n = 1.0;

//     let _a = [-n, -n+i, n];
//     let _b = [n, -n+i, n];
//     let _c = [n, n+i, n];
//     let _d = [-n, n+i, n];

//     let _e = [-n, -n+i, -n];
//     let _f = [n, -n+i, -n];
//     let _g = [n, n+i, -n];
//     let _h = [-n, n+i, -n];

//     let _tex_a = [0.0, 0.0];
//     let _tex_b = [1.0, 0.0];
//     let _tex_c = [0.0, 1.0];
//     let _tex_e = [1.0, 1.0];

//     return vec![
//         // Front
//         Vertex { position: _a, tex_coords: _tex_a },
//         Vertex { position: _b, tex_coords: _tex_b },
//         Vertex { position: _d, tex_coords: _tex_c },

//         Vertex { position: _d, tex_coords: _tex_c },
//         Vertex { position: _c, tex_coords: _tex_e },
//         Vertex { position: _b, tex_coords: _tex_b },

//         // Right
//         Vertex { position: _b, tex_coords: _tex_a },
//         Vertex { position: _f, tex_coords: _tex_b },
//         Vertex { position: _c, tex_coords: _tex_c },
//         Vertex { position: _c, tex_coords: _tex_c },
//         Vertex { position: _g, tex_coords: _tex_e },
//         Vertex { position: _f, tex_coords: _tex_b },

//         // Back
//         Vertex { position: _e, tex_coords: _tex_a },
//         Vertex { position: _f, tex_coords: _tex_b },
//         Vertex { position: _h, tex_coords: _tex_c },
//         Vertex { position: _h, tex_coords: _tex_c },
//         Vertex { position: _g, tex_coords: _tex_e },
//         Vertex { position: _f, tex_coords: _tex_b },

//         // Left
//         Vertex { position: _a, tex_coords: _tex_a },
//         Vertex { position: _e, tex_coords: _tex_b },
//         Vertex { position: _d, tex_coords: _tex_c },
//         Vertex { position: _d, tex_coords: _tex_c },
//         Vertex { position: _h, tex_coords: _tex_e },
//         Vertex { position: _e, tex_coords: _tex_b },

//         // Top
//         Vertex { position: _d, tex_coords: _tex_a },
//         Vertex { position: _c, tex_coords: _tex_b },
//         Vertex { position: _h, tex_coords: _tex_c },
//         Vertex { position: _h, tex_coords: _tex_c },
//         Vertex { position: _g, tex_coords: _tex_e },
//         Vertex { position: _c, tex_coords: _tex_b },

//         // Bottom
//         Vertex { position: _a, tex_coords: _tex_a },
//         Vertex { position: _b, tex_coords: _tex_b },
//         Vertex { position: _e, tex_coords: _tex_c },
//         Vertex { position: _e, tex_coords: _tex_c },
//         Vertex { position: _f, tex_coords: _tex_e },
//         Vertex { position: _b, tex_coords: _tex_b },
//     ];
// }

// fn get_texture(display: &glium::Display) -> SrgbTexture2d{ 
//     let image = image::load(Cursor::new(&include_bytes!("../assets/black_hole.png")), image::ImageFormat::Png).unwrap().to_rgba8();
//     let image_dimensions = image.dimensions();
//     let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

//     return glium::texture::SrgbTexture2d::new(display, image).unwrap();
// }

// fn elapsed_ms(x: Instant) -> i64 {
//     // let dur = self.elapsed();
//     let dur = x.elapsed();

//     return (dur.as_secs() * 1000 + (dur.subsec_nanos() / 1000000) as u64) as i64;
// }