use std::collections::HashMap;
use uuid::Uuid;

use crate::models::Block;

pub struct Entity {
    pub blocks: HashMap<(i64, i64), Block>,
    pub block_type: String,
    pub id: Uuid,
}

pub struct Path {
    pub entity: Entity,
    pub path_type: String,
    pub upper_end_location: (i64, i64),
    pub lower_end_location: (i64, i64),
}

impl Path {
    fn set_entity_blocks(&mut self, blocks: HashMap<(i64, i64), Block>) {
        self.entity.blocks = blocks;
    }
    pub fn bresenham_line(
        x0: i64,
        y0: i64,
        x1: i64,
        y1: i64,
        material: String,
    ) -> HashMap<(i64, i64), Block> {
        let mut points = HashMap::new();

        debug_assert!(x1 >= x0, "x1 should be greater than or equal to x0");
        debug_assert!(y1 >= y0, "y1 should be greater than or equal to y0");
        let dx: i64 = x1 - x0;
        let dy: i64 = y1 - y0;
        let mut D: i64 = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            points.insert(
                (x, y),
                Block {
                    material: material.clone(),
                    x_cord: x,
                    y_cord: y,
                },
            );
            if (D > 0) {
                y = y + 1;
                D = D - 2 * x;
            }
        }

        points
    }
}
