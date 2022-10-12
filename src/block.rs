use glium::implement_vertex;

use crate::Vertex;

pub struct Block {
    pub indices: glium::index::NoIndices,
    pub vertexes: Vec<Vertex>,
}
impl Block {
    pub fn new(x: f32, y: f32, z: f32) -> Block {
        Block {
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            vertexes: Self::get_cube(x, y, z)
        }
    }

    fn get_cube(x: f32, y: f32, z: f32) -> Vec<Vertex> {
        let n = 0.5;

        let nx = n + (x * n);
        let ny = n + (y * n);
        let nz = n + (z * n);
    
        let a = [-nx, -ny, nz];
        let b = [nx, -ny, nz];
        let c = [nx, ny, nz];
        let d = [-nx, ny, nz];
    
        let e = [-nx, -ny, -nz];
        let f = [nx, -ny, -nz];
        let g = [nx, ny, -nz];
        let h = [-nx, ny, -nz];
    
        let tex_a = [0.0, 0.0];
        let tex_b = [1.0, 0.0];
        let tex_c = [0.0, 1.0];
        let tex_e = [1.0, 1.0];
    
        return vec![
            // Front
            Vertex { position: a, tex_coords: tex_a },
            Vertex { position: b, tex_coords: tex_b },
            Vertex { position: d, tex_coords: tex_c },
    
            Vertex { position: d, tex_coords: tex_c },
            Vertex { position: c, tex_coords: tex_e },
            Vertex { position: b, tex_coords: tex_b },
    
            // Right
            Vertex { position: b, tex_coords: tex_a },
            Vertex { position: f, tex_coords: tex_b },
            Vertex { position: c, tex_coords: tex_c },
            Vertex { position: c, tex_coords: tex_c },
            Vertex { position: g, tex_coords: tex_e },
            Vertex { position: f, tex_coords: tex_b },
    
            // Back
            Vertex { position: e, tex_coords: tex_a },
            Vertex { position: f, tex_coords: tex_b },
            Vertex { position: h, tex_coords: tex_c },
            Vertex { position: h, tex_coords: tex_c },
            Vertex { position: g, tex_coords: tex_e },
            Vertex { position: f, tex_coords: tex_b },
    
            // Left
            Vertex { position: a, tex_coords: tex_a },
            Vertex { position: e, tex_coords: tex_b },
            Vertex { position: d, tex_coords: tex_c },
            Vertex { position: d, tex_coords: tex_c },
            Vertex { position: h, tex_coords: tex_e },
            Vertex { position: e, tex_coords: tex_b },
    
            // Top
            Vertex { position: d, tex_coords: tex_a },
            Vertex { position: c, tex_coords: tex_b },
            Vertex { position: h, tex_coords: tex_c },
            Vertex { position: h, tex_coords: tex_c },
            Vertex { position: g, tex_coords: tex_e },
            Vertex { position: c, tex_coords: tex_b },
    
            // Bottom
            Vertex { position: a, tex_coords: tex_a },
            Vertex { position: b, tex_coords: tex_b },
            Vertex { position: e, tex_coords: tex_c },
            Vertex { position: e, tex_coords: tex_c },
            Vertex { position: f, tex_coords: tex_e },
            Vertex { position: b, tex_coords: tex_b },
        ];
    }
}

implement_vertex!(Vertex, position, tex_coords);