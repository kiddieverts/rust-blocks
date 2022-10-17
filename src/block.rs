use glium::implement_vertex;

use crate::{Vertex, Sides};

pub struct Block {
    pub vertices: Vec<Vertex>,
}

impl Block {
    pub fn get_vertices(_x: i32, _y: i32, _z: i32, sides: Sides) -> Vec<Vertex> {
        let w: f32 = 0.5;

        let x = _x as f32 * w;
        let y = _y as f32 * w;
        let z = _z as f32 * w;

        let a = [x, y, z];
        let b = [x+w, y, z];
        let c = [x+w, y+w, z];
        let d = [x, y+w, z];

        let e = [x, y, z-w];
        let f = [x+w, y, z-w];
        let g = [x+w, y+w, z-w];
        let h = [x, y+w, z-w];

        let tex_a = [0.0, 0.0];
        let tex_b = [1.0, 0.0];
        let tex_c = [0.0, 1.0];
        let tex_e = [1.0, 1.0];

        let _back: f32 = 0.6;
        let _front: f32 = 0.8;
        let _left: f32 = 0.65;
        let _right: f32 = 0.75;
        let _bottom: f32 = 0.45;
        let _top: f32 = 0.95;

        let mut vec: Vec<Vertex> = vec![];

        // Front
        if sides.front {
            vec.push(Vertex { position: a, tex_coords: tex_a, brightness: _front });
            vec.push(Vertex { position: b, tex_coords: tex_b, brightness: _front });
            vec.push(Vertex { position: d, tex_coords: tex_c, brightness: _front });
            vec.push(Vertex { position: d, tex_coords: tex_c, brightness: _front });
            vec.push(Vertex { position: c, tex_coords: tex_e, brightness: _front });
            vec.push(Vertex { position: b, tex_coords: tex_b, brightness: _front });
        }

        // Right
        if sides.right {
            vec.push(Vertex { position: b, tex_coords: tex_a, brightness: _right });
            vec.push(Vertex { position: f, tex_coords: tex_b, brightness: _right });
            vec.push(Vertex { position: c, tex_coords: tex_c, brightness: _right });
            vec.push(Vertex { position: c, tex_coords: tex_c, brightness: _right });
            vec.push(Vertex { position: g, tex_coords: tex_e, brightness: _right });
            vec.push(Vertex { position: f, tex_coords: tex_b, brightness: _right });
        }

        // Back
        if sides.back {
            vec.push(Vertex { position: e, tex_coords: tex_a, brightness: _back });
            vec.push(Vertex { position: f, tex_coords: tex_b, brightness: _back });
            vec.push(Vertex { position: h, tex_coords: tex_c, brightness: _back });
            vec.push(Vertex { position: h, tex_coords: tex_c, brightness: _back });
            vec.push(Vertex { position: g, tex_coords: tex_e, brightness: _back });
            vec.push(Vertex { position: f, tex_coords: tex_b, brightness: _back });
        }

        // Left
        if sides.left {
            vec.push(Vertex { position: a, tex_coords: tex_a, brightness: _left });
            vec.push(Vertex { position: e, tex_coords: tex_b, brightness: _left });
            vec.push(Vertex { position: d, tex_coords: tex_c, brightness: _left });
            vec.push(Vertex { position: d, tex_coords: tex_c, brightness: _left });
            vec.push(Vertex { position: h, tex_coords: tex_e, brightness: _left });
            vec.push(Vertex { position: e, tex_coords: tex_b, brightness: _left });
        }

        // Top
        if sides.top {
            vec.push(Vertex { position: d, tex_coords: tex_a, brightness: _top });
            vec.push(Vertex { position: c, tex_coords: tex_b, brightness: _top });
            vec.push(Vertex { position: h, tex_coords: tex_c, brightness: _top });
            vec.push(Vertex { position: h, tex_coords: tex_c, brightness: _top });
            vec.push(Vertex { position: g, tex_coords: tex_e, brightness: _top });
            vec.push(Vertex { position: c, tex_coords: tex_b, brightness: _top });
        }

        // Bottom
        if sides.bottom {
            vec.push(Vertex { position: a, tex_coords: tex_a, brightness: _bottom });
            vec.push(Vertex { position: b, tex_coords: tex_b, brightness: _bottom });
            vec.push(Vertex { position: e, tex_coords: tex_c, brightness: _bottom });
            vec.push(Vertex { position: e, tex_coords: tex_c, brightness: _bottom });
            vec.push(Vertex { position: f, tex_coords: tex_e, brightness: _bottom });
            vec.push(Vertex { position: b, tex_coords: tex_b, brightness: _bottom });
        }

        return vec;
    }
}

implement_vertex!(Vertex, position, tex_coords, brightness);
