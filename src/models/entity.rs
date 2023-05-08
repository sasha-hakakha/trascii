use std::collections::HashMap;
use uuid::Uuid;

use crate::models::Block;

pub struct Entity { 
    pub blocks: HashMap<Block>,
    pub block_type: String,
    pub id: Uuid,
}

pub struct Path {
    pub entity: Entity,
    pub path_type: String,
    pub upper_end_location: (u64, u64),
    pub lower_end_location: (u64, u64),
}

impl Path {
    fn set_entity_blocks(&mut self, blocks: HashMap<Block>){
	self.entity.blocks = blocks;
    }
}
