// // https://learnopengl.com/Getting-started/Camera

// #[macro_use]
// #[allow(unused_imports)]

// extern crate glm;

// extern crate glium;
// use glium::{glutin, Surface, uniform, implement_vertex};
// use glium::{texture::SrgbTexture2d, VertexBuffer, index::NoIndices};
// use std::io::Cursor;
// use std::time::Instant;

// fn main() {
//     const REFRESH_RATE: i64 = 1000 / 20;
//     const WINDOW_WIDTH: u32 = 1280;
//     const WINDOW_HEIGHT: u32 = 720;

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
    
//     let texture = get_texture(&display);
//     let blk = Block::new(&display);

//     let mut time_stamp = Instant::now();

//     event_loop.run(move |event, _, control_flow| {
//         let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
//         *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

//         match event {
//             glutin::event::Event::WindowEvent { event, .. } => {
//                 match event {
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
//                     time_stamp = Instant::now();
//                 }

//                draw(&display, &texture, &program, &blk.buffer, blk.indices);
//             },
//             _ => (),
//         }
//     });
// }

// fn draw(display: &glium::Display, texture: &SrgbTexture2d, program: &glium::Program, positions: &VertexBuffer<Vertex>, indices: NoIndices) {
//     let mut target = display.draw();
//     target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

//     let model = [
//         [1.0, 0.0, 0.0, 0.0],
//         [0.0, 1.0, 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0f32],
//     ];

//     let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.5]);

//     let (width, height) = target.get_dimensions();
//     let perspective = get_perspective(width, height);

//     let params = glium::DrawParameters {
//         depth: glium::Depth {
//             test: glium::draw_parameters::DepthTest::IfLess,
//             write: true,
//             .. Default::default()
//         },
//         .. Default::default()
//     };

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

//     let s = [up[1] * f[2] - up[2] * f[1],
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