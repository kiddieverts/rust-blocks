use glium::implement_vertex;

use crate::Vertex;

pub struct Block {
    pub indices: glium::index::NoIndices,
    pub vertexes: Vec<Vertex>,
}
impl Block {
    pub fn new() -> Block {
        Block {
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            vertexes: Self::get_cube()
        }
    }

    fn get_cube() -> Vec<Vertex> {
        let n = 0.5;
    
        let _a = [-n, -n, n];
        let _b = [n, -n, n];
        let _c = [n, n, n];
        let _d = [-n, n, n];
    
        let _e = [-n, -n, -n];
        let _f = [n, -n, -n];
        let _g = [n, n, -n];
        let _h = [-n, n, -n];
    
        let _tex_a = [0.0, 0.0];
        let _tex_b = [1.0, 0.0];
        let _tex_c = [0.0, 1.0];
        let _tex_e = [1.0, 1.0];
    
        return vec![
            // Front
            Vertex { position: _a, tex_coords: _tex_a },
            Vertex { position: _b, tex_coords: _tex_b },
            Vertex { position: _d, tex_coords: _tex_c },
    
            Vertex { position: _d, tex_coords: _tex_c },
            Vertex { position: _c, tex_coords: _tex_e },
            Vertex { position: _b, tex_coords: _tex_b },
    
            // Right
            Vertex { position: _b, tex_coords: _tex_a },
            Vertex { position: _f, tex_coords: _tex_b },
            Vertex { position: _c, tex_coords: _tex_c },
            Vertex { position: _c, tex_coords: _tex_c },
            Vertex { position: _g, tex_coords: _tex_e },
            Vertex { position: _f, tex_coords: _tex_b },
    
            // Back
            Vertex { position: _e, tex_coords: _tex_a },
            Vertex { position: _f, tex_coords: _tex_b },
            Vertex { position: _h, tex_coords: _tex_c },
            Vertex { position: _h, tex_coords: _tex_c },
            Vertex { position: _g, tex_coords: _tex_e },
            Vertex { position: _f, tex_coords: _tex_b },
    
            // Left
            Vertex { position: _a, tex_coords: _tex_a },
            Vertex { position: _e, tex_coords: _tex_b },
            Vertex { position: _d, tex_coords: _tex_c },
            Vertex { position: _d, tex_coords: _tex_c },
            Vertex { position: _h, tex_coords: _tex_e },
            Vertex { position: _e, tex_coords: _tex_b },
    
            // Top
            Vertex { position: _d, tex_coords: _tex_a },
            Vertex { position: _c, tex_coords: _tex_b },
            Vertex { position: _h, tex_coords: _tex_c },
            Vertex { position: _h, tex_coords: _tex_c },
            Vertex { position: _g, tex_coords: _tex_e },
            Vertex { position: _c, tex_coords: _tex_b },
    
            // Bottom
            Vertex { position: _a, tex_coords: _tex_a },
            Vertex { position: _b, tex_coords: _tex_b },
            Vertex { position: _e, tex_coords: _tex_c },
            Vertex { position: _e, tex_coords: _tex_c },
            Vertex { position: _f, tex_coords: _tex_e },
            Vertex { position: _b, tex_coords: _tex_b },
        ];
    }
}

implement_vertex!(Vertex, position, tex_coords);