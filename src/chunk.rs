use std::collections::{HashMap, HashSet};

use crate::{BlockId, Sides, block::{Block}, Vertex, constants::{CHUNK_WIDTH, CHUNK_HEIGHT}};

pub struct Chunk {
   pub value: HashMap<i32, BlockId> // TODO: Find a better performing datastructure. Probably use pointers
}

impl Chunk {
    pub fn new() -> Chunk {
        // Create a chunk with a predefined pattern.

        let mut value: HashMap<i32, BlockId> = HashMap::new();

        for i in 0..(CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT) {
            if i % 3 == 0 {
                value.insert(i, BlockId::Plank);
            }
            else {
                value.insert(i, BlockId::Air);
            }
        }

        Chunk { value }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = vec![];
        let mut i = 0;

        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_WIDTH {
                    // Check what sides are visible
                    let sides = self.get_sides(i);
                    // Only render vertices that the player sees
                    let block_verteces = Block::get_vertices(x, y, -z, sides);
                    for vertex in 0..block_verteces.len() {
                        // Only render visible blocks, i.e. don't render Air blocks
                        if self.value[&i].is_visible() {
                            vertices.push(block_verteces[vertex]);
                        }
                    }
                    i = i +1;
                }
            }
        }
        return vertices;
    }

    fn get_sides(&self, i: i32) -> Sides {
        let max = CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT;

        let is_transparent = |offset: &i32| {
            return self.value[offset].is_transparent();
        };

        let show_left = || {
            // Always show the leftmost sides.
            if i % CHUNK_WIDTH == 0 {
                return true;
            }
            // Else show left side if the block to the left is transparent.
            return is_transparent(&(i - 1));
        };

        let show_right = || {
            // Always show the rightmost sides.
            if (i + 1) % CHUNK_WIDTH == 0 {
                return true;
            }
            // Else show right side if the block to the right is transparent.
            return is_transparent(&(i + 1));
        };

        let show_front = || {
            // Always show the frontmost sides
            let res = match i % (CHUNK_WIDTH * CHUNK_WIDTH) {
                0 => true,
                1 => true,
                2 => true,
                3 => true,
                _ => false
            };
            if res == true {
                return res;
            }
            // Else show front side if the block in front of it is transparent.
            return is_transparent(&(i - CHUNK_WIDTH));
        };

        let show_back = || {
            // Always show the backmost sides
            let mut cnt = 0;
            let mut val: HashSet<i32> = HashSet::new();

            while cnt < max {
                val.insert(cnt + (CHUNK_WIDTH * CHUNK_WIDTH) - CHUNK_WIDTH);
                val.insert(cnt + (CHUNK_WIDTH * CHUNK_WIDTH) - CHUNK_WIDTH + 1);
                val.insert(cnt + (CHUNK_WIDTH * CHUNK_WIDTH) - CHUNK_WIDTH + 2);
                val.insert(cnt + (CHUNK_WIDTH * CHUNK_WIDTH) - CHUNK_WIDTH + 3);
                cnt += CHUNK_WIDTH * CHUNK_WIDTH;
            }

            if val.contains(&i) {
                return true;
            }

            // Make sure the offset can't be higher than the max
            let offset = i + CHUNK_WIDTH;
            if offset >= max { 
                return false;
            }

            // Else show the backside if the block behind it is transparent.
            return is_transparent(&offset);
        };

        let show_top = || {
            // Always show the topmost blocks
            if i > (max - CHUNK_WIDTH * CHUNK_WIDTH - 1) {
                return true;
            }

            let offset = i + CHUNK_WIDTH * CHUNK_WIDTH;

            // Make sure the offset can't be higher than the max
            if offset >= max { 
                return false;
            }

            // Else show the topside if the block above it is transparent.
            return is_transparent(&offset);
        };

        let show_bottom = || {
            // Always show the bottommost blocks
            if i > max - CHUNK_WIDTH * CHUNK_WIDTH {
                return true;
            }
            
            let offset = i - CHUNK_WIDTH * CHUNK_WIDTH;

            // Make sure the offset is not negative
            if offset < 0 {
                return false;
            }
            // Else show the bottomside if the block below it is transparent
            return is_transparent(&offset);
        };

        return Sides { 
            left: show_left(), 
            right: show_right(),
            front: show_front(), 
            back: show_back(), 
            top: show_top(), 
            bottom: show_bottom(), 
        };
    }
}