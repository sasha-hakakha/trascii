/// This is the internal representation of a line based on 2 space vectors

use super::Coor;
use super::v2::V2;

/// Two dimmensional line
#[derive(PartialEq, Eq)]        // note that eq cares about start vs end
pub struct Line {
    pub start: V2,
    pub end: V2,
}

impl Line {
    pub fn new(start: V2, end: V2) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn len_sqr(&self) -> Coor {
        self.end.sub(&self.start).len_sqr()
    }

    /// Iterate over the integer squares for the Line (segment) using
    /// Bresenham's alg
    pub fn as_iter(&self) -> LineIter {
        let (mut left, mut right) = (self.start, self.end);
        let mut vert_flip = false;
        let mut recip_flip = false;
        if self.end.x < self.start.x {
            // need to flip horizontally
            left = self.end;
            right = self.start;
        }
        if right.y < left.y {
            // need to flip vertically
            left.y *= -1;
            right.y *= -1;
            vert_flip = true;
        }
        let mut slope = (right.y - left.y) as f32 /
            (right.x - left.x) as f32;
        if slope > 1.0 {
            slope = 1.0 / slope;
            recip_flip = true;
        }
        // println!("slope: {}, {}, {}", slope, vert_flip, recip_flip);
        LineIter {
            slope,
            cur: left,
            error: 0.5,
            final_x: right.x,
            vert_flip,
            recip_flip,
        }
    }
}

pub struct LineIter {
    slope: f32,
    cur: V2,
    error: f32,
    final_x: Coor,
    vert_flip: bool,            // flips happen in this order
    recip_flip: bool,
}

impl<'a> Iterator for LineIter {
    type Item = V2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.x == self.final_x {
            None
        } else {
            let mut hold = self.cur;
            let expected: f32 = self.slope + self.error;

            self.cur = V2 {
                x: hold.x + 1,
                y: if expected > 0.5 {
                    // println!("{}, {}, adjusted to new {}", self.error, expected, expected - 1.0);
                    self.error = expected - 1.0;
                    hold.y + 1
                } else {
                    self.error = expected;
                    hold.y
                },
            };

            if self.vert_flip {
                hold.y *= -1
            }
            if self.recip_flip {
                let h = hold.x;
                hold.x = hold.y;
                hold.y = h;
            }

            // print!("{}, ", self.error);
            Some(hold)
        }
    }
}
