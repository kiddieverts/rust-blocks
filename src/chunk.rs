use std::collections::{HashMap, HashSet};

use crate::{BlockId, Sides, block, Vertex};

pub struct Chunk {
   pub value: HashMap<i32, BlockId>,
}

impl Chunk {
    fn get_chunk_width() -> i32 { 4 }
    fn get_chunk_height() -> i32 { 8 }

    pub fn new() -> Chunk {
        let mut value: HashMap<i32, BlockId> = HashMap::new();

        let w = Chunk::get_chunk_width();
        let h = Chunk::get_chunk_height();

        for i in 0..(w * w * h) {
            if i % 3 == 0 {
                value.insert(i, BlockId::Plank);
            }
            else {
                value.insert(i, BlockId::Air);
            }
        }

        return Chunk {
            value
        }
    }

    pub fn get_vertexes( chunk: Chunk) -> Vec<Vertex> {
        let mut v: Vec<Vertex> = vec![];
        let mut i = 0;

        let chunk_weight: i32 = Chunk::get_chunk_width();
        let chunk_height: i32 = Chunk::get_chunk_height();
    
        for y in 0..chunk_height {
            for z in 0..chunk_weight {
                for x in 0..chunk_weight {
                    let sides = Chunk::get_sides(i, &chunk);
                    let block = block::Block::get_cube(x, y, -z, sides);
                    for item in 0..block.len() {
                        if chunk.value[&i].is_visible() {
                            v.push(block[item]);
                        }
                    }
                    i = i +1;
                }
            }
        }
        return v;
    }

    fn get_sides(i: i32, chunk: &Chunk) -> Sides {
        let chunk_width: i32 = Chunk::get_chunk_width();
        let chunk_height: i32 = Chunk::get_chunk_height();

        let max = chunk_width * chunk_width * chunk_height;

        let get_left = || {
            if i % chunk_width == 0 {
                return true;
            }
            return chunk.value[&(i -1)].is_transparent();
        };

        let get_right = || {
            if (i+1) % chunk_width == 0 {
                return true;
            }
            return chunk.value[&(i +1)].is_transparent();
        };

        let get_front = || {
            let res = match i % (chunk_width * chunk_width) {
                0 => true,
                1 => true,
                2 => true,
                3 => true,
                _ => false
            };
            if res == true {
                return res;
            }
            return chunk.value[&(i - chunk_width)].is_transparent();
        };

        let get_back = || {
            let mut cnt = 0;
            let mut val: HashSet<i32> = HashSet::new();

            while cnt < max {
                val.insert(cnt + (chunk_width * chunk_width) - chunk_width);
                val.insert(cnt + (chunk_width * chunk_width) - chunk_width + 1);
                val.insert(cnt + (chunk_width * chunk_width) - chunk_width + 2);
                val.insert(cnt + (chunk_width * chunk_width) - chunk_width + 3);
                cnt += chunk_width * chunk_width;
            }

            if val.contains(&i) {
                return true;
            }

            if i + chunk_width >= max { 
                return false;
            }

            return chunk.value[&(i + chunk_width)].is_transparent();
        };

        let get_top = || {
            if i > (max - chunk_width * chunk_width - 1) {
                return true;
            }

            if (i + chunk_width * chunk_width) >= max { 
                return false;
            }
        
            return chunk.value[&(i + chunk_width * chunk_width)].is_transparent();
        };

        let get_bottom = || {
            if i > max - chunk_width * chunk_width {
                return true;
            }

            if i - chunk_width * chunk_width < 0 {
                return false;
            }

            return chunk.value[&(i - chunk_width * chunk_width)].is_transparent();
        };

        return Sides { 
            top: get_top(), 
            bottom: get_bottom(), 
            front: get_front(), 
            back: get_back(), 
            left: get_left(), 
            right: get_right() 
        };
    }
}