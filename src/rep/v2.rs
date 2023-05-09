/// This is the internal representation for 2-space vectors, and related helper
/// functions

// use std::math::sqrt;

use super::Coor;

// TODO does rust have traits for + and - and whatnot?

/// Two dimensional vector
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct V2 {
    pub x: Coor,
    pub y: Coor,
}

/// These all create new instances and don't modify the inputs
impl V2 {
    pub fn new(x: Coor, y: Coor) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn dot(&self, other: &Self) -> Coor {
        self.x * other.x + self.y * other.y
    }

    pub fn len_sqr(&self) -> Coor {
        self.x*self.x + self.y*self.y
    }


    pub fn scale(&self, s: Coor) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }

    // we can't do normalize / make unit if we are sticking to integer
    // coordinates

//     pub fn len(&self) -> Coor {
//         self.len_sqr().sqrt()
//     }
}

