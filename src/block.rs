use glium::implement_vertex;

use crate::{Vertex, Sides};

pub struct Block {
    pub vertices: Vec<Vertex>,
}

impl Block {
    pub fn get_vertices(_x: i32, _y: i32, _z: i32, sides: Sides) -> Vec<Vertex> {
        let w: f32 = 0.5; // The width of the block

        let x = _x as f32 * w;
        let y = _y as f32 * w;
        let z = _z as f32 * w;
        
        let xw = x + w;
        let yw = y + w;
        let zw = z - w;

        let a = [x, y, z];
        let b = [xw, y, z];
        let c = [xw, yw, z];
        let d = [x, yw, z];

        let e = [x, y, zw];
        let f = [xw, y, zw];
        let g = [xw, yw, zw];
        let h = [x, yw, zw];

        // Texture coordinates. [0.0, 1.0] is left corner top, [1.0, 0.0] is right corner bottom.
        let tex_a = [0.0, 0.0];
        let tex_b = [1.0, 0.0];
        let tex_c = [0.0, 1.0];
        let tex_e = [1.0, 1.0];

        // Each side has a different brightness value
        let back_brightness: f32 = 0.6;
        let front_brightness: f32 = 0.8;
        let left_brightness: f32 = 0.65;
        let right_brightness: f32 = 0.75;
        let bottom_brightness: f32 = 0.45;
        let top_brightness: f32 = 0.95;

        let mut vertices: Vec<Vertex> = vec![];

        // Front
        if sides.front {
            vertices.push(Vertex { position: a, tex_coords: tex_a, brightness: front_brightness });
            vertices.push(Vertex { position: b, tex_coords: tex_b, brightness: front_brightness });
            vertices.push(Vertex { position: d, tex_coords: tex_c, brightness: front_brightness });
            vertices.push(Vertex { position: d, tex_coords: tex_c, brightness: front_brightness });
            vertices.push(Vertex { position: c, tex_coords: tex_e, brightness: front_brightness });
            vertices.push(Vertex { position: b, tex_coords: tex_b, brightness: front_brightness });
        }

        // Right
        if sides.right {
            vertices.push(Vertex { position: b, tex_coords: tex_a, brightness: right_brightness });
            vertices.push(Vertex { position: f, tex_coords: tex_b, brightness: right_brightness });
            vertices.push(Vertex { position: c, tex_coords: tex_c, brightness: right_brightness });
            vertices.push(Vertex { position: c, tex_coords: tex_c, brightness: right_brightness });
            vertices.push(Vertex { position: g, tex_coords: tex_e, brightness: right_brightness });
            vertices.push(Vertex { position: f, tex_coords: tex_b, brightness: right_brightness });
        }

        // Back
        if sides.back {
            vertices.push(Vertex { position: e, tex_coords: tex_a, brightness: back_brightness });
            vertices.push(Vertex { position: f, tex_coords: tex_b, brightness: back_brightness });
            vertices.push(Vertex { position: h, tex_coords: tex_c, brightness: back_brightness });
            vertices.push(Vertex { position: h, tex_coords: tex_c, brightness: back_brightness });
            vertices.push(Vertex { position: g, tex_coords: tex_e, brightness: back_brightness });
            vertices.push(Vertex { position: f, tex_coords: tex_b, brightness: back_brightness });
        }

        // Left
        if sides.left {
            vertices.push(Vertex { position: a, tex_coords: tex_a, brightness: left_brightness });
            vertices.push(Vertex { position: e, tex_coords: tex_b, brightness: left_brightness });
            vertices.push(Vertex { position: d, tex_coords: tex_c, brightness: left_brightness });
            vertices.push(Vertex { position: d, tex_coords: tex_c, brightness: left_brightness });
            vertices.push(Vertex { position: h, tex_coords: tex_e, brightness: left_brightness });
            vertices.push(Vertex { position: e, tex_coords: tex_b, brightness: left_brightness });
        }

        // Top
        if sides.top {
            vertices.push(Vertex { position: d, tex_coords: tex_a, brightness: top_brightness });
            vertices.push(Vertex { position: c, tex_coords: tex_b, brightness: top_brightness });
            vertices.push(Vertex { position: h, tex_coords: tex_c, brightness: top_brightness });
            vertices.push(Vertex { position: h, tex_coords: tex_c, brightness: top_brightness });
            vertices.push(Vertex { position: g, tex_coords: tex_e, brightness: top_brightness });
            vertices.push(Vertex { position: c, tex_coords: tex_b, brightness: top_brightness });
        }

        // Bottom
        if sides.bottom {
            vertices.push(Vertex { position: a, tex_coords: tex_a, brightness: bottom_brightness });
            vertices.push(Vertex { position: b, tex_coords: tex_b, brightness: bottom_brightness });
            vertices.push(Vertex { position: e, tex_coords: tex_c, brightness: bottom_brightness });
            vertices.push(Vertex { position: e, tex_coords: tex_c, brightness: bottom_brightness });
            vertices.push(Vertex { position: f, tex_coords: tex_e, brightness: bottom_brightness });
            vertices.push(Vertex { position: b, tex_coords: tex_b, brightness: bottom_brightness });
        }

        return vertices;
    }
}

implement_vertex!(Vertex, position, tex_coords, brightness);
