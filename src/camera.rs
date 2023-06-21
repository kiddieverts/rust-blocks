use glium::{glutin::{event::VirtualKeyCode, self}};
use glm::{Vector3, cos, sin, Matrix4};

extern crate glm;
pub struct Camera {
    pub position: Vector3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub window_width: u32,
    pub window_height: u32,
    pub delta_time: f32,
    pub last_frame: f32,
    yaw: f64,
    pitch: f64,
    fov: f64,
}

pub struct CameraCalculation {
    pub model: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub perspective: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(window_width: u32, window_height: u32) -> Camera {
        return Camera {
            yaw: -90.0,	// Yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
            pitch:  0.0,
            position: glm::vec3(1.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            fov: 45.0, // Field of view
            window_width: window_width,
            window_height: window_height,
            last_frame: 0.0,
            delta_time: 0.0
        };
    }

    pub fn process_keyboard(&mut self, virtual_keycode: Option<VirtualKeyCode>) {
        // TODO: The player should be able to use more than one button at the time. 

        let sensitivity = 7.0; // Change this value to your liking
        let speed = self.delta_time * sensitivity;
        let camera_speed = glm::vec3(speed, speed, speed); 

        let key = match virtual_keycode {
            Some(key) => key,
            None => return
        };
        match key {
            glutin::event::VirtualKeyCode::W => self.position = self.position + camera_speed * self.front,
            glutin::event::VirtualKeyCode::S => self.position = self.position - camera_speed * self.front,
            glutin::event::VirtualKeyCode::A => self.position = self.position - glm::normalize(glm::cross(self.front, self.up)) * camera_speed,
            glutin::event::VirtualKeyCode::D => self.position = self.position + glm::normalize(glm::cross(self.front, self.up)) * camera_speed,
            glutin::event::VirtualKeyCode::Space =>  self.position = glm::vec3(self.position.x, self.position.y + camera_speed.y / 2.0, self.position.z),
            glutin::event::VirtualKeyCode::LShift => self.position = glm::vec3(self.position.x, self.position.y - camera_speed.y / 2.0, self.position.z),
            _ => ()
        }
    }

    pub fn process_mouse(&mut self, x: f64, y: f64) {
        let xpos = x / 2.0;
        let ypos = y / 2.0;

        let mut xoffset = xpos - self.window_width as f64 / 2.0;
        let mut yoffset = self.window_height as f64 / 2.0 - ypos;

        let sensitivity = 0.1; // Change this value to your liking
        xoffset *= sensitivity;
        yoffset *= sensitivity;
    
        self.yaw += xoffset;
        self.pitch += yoffset;
    
        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let yaw_radians = glm::radians(self.yaw);
        let pitch_radians = glm::radians(self.pitch);
        let cos_pitch_radians = cos(pitch_radians);
    
        let front = glm::vec3(
            (cos(yaw_radians) * cos_pitch_radians) as f32,
            sin(pitch_radians) as f32,
            (sin(yaw_radians) * cos_pitch_radians) as f32
        );

        self.front = glm::normalize(front);
    }

    pub fn get_calculation(&self) -> CameraCalculation{
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
   
        let view = to_vec(glm::ext::look_at(self.position, self.position + self.front, self.up));

        let perspective = to_vec(glm::ext::perspective(
            glm::radians(self.fov as f32), 
            self.window_width as f32 / self.window_height as f32,  
            0.1 as f32,
            100.0 as f32));

        CameraCalculation { view, model, perspective }
    }
}

fn to_vec(input: Matrix4<f32>) -> [[f32;4]; 4] {
    [
        [input.c0.x, input.c0.y, input.c0.z, input.c0.w],
        [input.c1.x, input.c1.y, input.c1.z, input.c1.w],
        [input.c2.x, input.c2.y, input.c2.z, input.c2.w],
        [input.c3.x, input.c3.y, input.c3.z, input.c3.w]
    ]
}