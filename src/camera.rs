use glium::{glutin::{event::VirtualKeyCode, self, dpi::PhysicalPosition}, Surface, uniform};
use glm::{Vector3, cos, sin};

use crate::shader;

// pub mod shader;

extern crate glm;

pub struct Camera {
    pub camera_pos: Vector3<f32>,
    pub camera_front: Vector3<f32>,
    pub camera_up: Vector3<f32>,
    first_mouse: bool,
    yaw: f64,
    pitch: f64,
    last_x: f64,
    last_y: f64,
    camera_speed: Vector3<f32>
}

pub struct ShaderCalculation {
    pub model: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub perspective: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(window_width: u32, window_height: u32) -> Camera {
        let x = 0.05;

        return Camera {
            first_mouse: true,
            yaw: -90.0,	// yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
            pitch:  0.0,
            last_x:  window_width as f64 / 2.0,
            last_y: window_height as f64 / 2.0,
            camera_pos: glm::vec3(1.0, 0.0, 3.0),
            camera_front: glm::vec3(0.0, 0.0, -1.0),
            camera_up: glm::vec3(0.0, 1.0, 0.0),
            camera_speed: glm::vec3(x, x, x)
        };
    }

    pub fn process_keyboard(&mut self, virtual_keycode: Option<VirtualKeyCode>) 
    {
        let key = match virtual_keycode {
            Some(key) => key,
            None => return
        };
        match key {
            glutin::event::VirtualKeyCode::W => self.camera_pos = self.camera_pos + self.camera_speed * self.camera_front,
            glutin::event::VirtualKeyCode::S => self.camera_pos = self.camera_pos - self.camera_speed * self.camera_front,
            glutin::event::VirtualKeyCode::A => self.camera_pos = self.camera_pos - glm::normalize(glm::cross(self.camera_front, self.camera_up)) * self.camera_speed,
            glutin::event::VirtualKeyCode::D => self.camera_pos = self.camera_pos + glm::normalize(glm::cross(self.camera_front, self.camera_up)) * self.camera_speed,
            _ => ()
        }
    }

    pub fn process_mouse(&mut self, position: PhysicalPosition<f64>) {
        let xpos = position.x;
        let ypos = position.y;

        if self.first_mouse
        {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }
    
        let mut xoffset = xpos - self.last_x;
        let mut yoffset = self.last_y - ypos; // reversed since y-coordinates go from bottom to top
        self.last_x = xpos;
        self.last_y = ypos;
    
        let sensitivity = 0.1; // change this value to your liking
        xoffset *= sensitivity;
        yoffset *= sensitivity;
    
        self.yaw += xoffset;
        self.pitch += yoffset;
    
        // make sure that when pitch is out of bounds, screen doesn't get flipped
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let _x = (cos(glm::radians(self.yaw)) * cos(glm::radians(self.pitch))) as f32;
        let _y = sin(glm::radians(self.pitch)) as f32;
        let _z = (sin(glm::radians(self.yaw)) * cos(glm::radians(self.pitch))) as f32;
    
        let front = glm::vec3(_x, _y, _z);
        self.camera_front = glm::normalize(front);
    }

    pub fn get_calculation(&self) -> ShaderCalculation{
        // let mut target = shader.display.draw();
        // target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
    
        // [[f32; 4]; 4]
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
   
        // let params = glium::DrawParameters {
        //     depth: glium::Depth {
        //         test: glium::draw_parameters::DepthTest::IfLess,
        //         write: true,
        //         .. Default::default()
        //     },
        //     .. Default::default()
        // };

        let view_temp = glm::ext::look_at(self.camera_pos, self.camera_pos + self.camera_front, self.camera_up);

        let view = [
            [view_temp.c0.x, view_temp.c0.y, view_temp.c0.z, view_temp.c0.w],
            [view_temp.c1.x, view_temp.c1.y, view_temp.c1.z, view_temp.c1.w],
            [view_temp.c2.x, view_temp.c2.y, view_temp.c2.z, view_temp.c2.w],
            [view_temp.c3.x, view_temp.c3.y, view_temp.c3.z, view_temp.c3.w]
        ];

        let fov = 45.0;

        let projection_temp = glm::ext::perspective(glm::radians(fov), 1024.0 / 720.0, 0.1, 100.0);

        let perspective = [
            [projection_temp.c0.x as f32, projection_temp.c0.y as f32, projection_temp.c0.z as f32, projection_temp.c0.w as f32],
            [projection_temp.c1.x as f32, projection_temp.c1.y as f32, projection_temp.c1.z as f32, projection_temp.c1.w as f32],
            [projection_temp.c2.x as f32, projection_temp.c2.y as f32, projection_temp.c2.z as f32, projection_temp.c2.w as f32],
            [projection_temp.c3.x as f32, projection_temp.c3.y as f32, projection_temp.c3.z as f32, projection_temp.c3.w as f32]
        ];

        ShaderCalculation {
            view: view,
            model: model,
            perspective: perspective
        }

        // let uniforms = uniform!{ 
        //     model: model, 
        //     view: view, 
        //     perspective: perspective, 
        //     tex: &texture,
        // };
    }

}