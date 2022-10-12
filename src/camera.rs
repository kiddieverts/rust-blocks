use glium::{glutin::{event::VirtualKeyCode, self}};
use glm::{Vector3, cos, sin, Matrix4};

extern crate glm;

pub struct Camera {
    pub position: Vector3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    first_mouse: bool,
    yaw: f64,
    pitch: f64,
    last_x: f64,
    last_y: f64,
    camera_speed: Vector3<f32>,
    fov: f64
}

pub struct CameraCalculation {
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
            position: glm::vec3(1.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            fov: 45.0,
            camera_speed: glm::vec3(x, x, x) // TODO: <--
        };
    }

    pub fn process_keyboard(&mut self, virtual_keycode: Option<VirtualKeyCode>) 
    {
        let key = match virtual_keycode {
            Some(key) => key,
            None => return
        };
        match key {
            glutin::event::VirtualKeyCode::W => self.position = self.position + self.camera_speed * self.front,
            glutin::event::VirtualKeyCode::S => self.position = self.position - self.camera_speed * self.front,
            glutin::event::VirtualKeyCode::A => self.position = self.position - glm::normalize(glm::cross(self.front, self.up)) * self.camera_speed,
            glutin::event::VirtualKeyCode::D => self.position = self.position + glm::normalize(glm::cross(self.front, self.up)) * self.camera_speed,
            glutin::event::VirtualKeyCode::Space =>  self.position = glm::vec3(self.position.x, self.position.y + self.camera_speed.y, self.position.z),
            glutin::event::VirtualKeyCode::LShift => self.position = glm::vec3(self.position.x, self.position.y - self.camera_speed.y, self.position.z),
            _ => ()
        }
    }

    pub fn process_mouse(&mut self, xpos: f64, ypos: f64) {
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
            1024.0 as f32 / 720.0 as f32, // TODO: Aspect ratio should not be hardcoded
            0.1 as f32,
            100.0 as f32));

        CameraCalculation {
            view: view,
            model: model,
            perspective: perspective
        }
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