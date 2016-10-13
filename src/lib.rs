use std::ops::{Sub, Add};

pub mod screen;
pub mod grid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HexPosition {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vector {
    inner: HexPosition
}

impl HexPosition {
    // Constructs a new position at the origin (0, 0, 0)
    pub fn origin() -> HexPosition {
        HexPosition {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    // Constructs a new position.
    // Panics if (x + y + z) != 0
    pub fn new(x: i32, y: i32, z: i32) -> HexPosition {
        assert!(x + y + z == 0, "z + y + z must equal 0, got ({} {} {})", x, y, z);

        HexPosition {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn from_axial(x: i32, z: i32) -> HexPosition {
        let y = 0 - (x + z);
        HexPosition::new(x, y, z)
    }

    pub fn as_axial(&self) -> (i32, i32) {
        (self.x, self.z)
    }

    pub fn neighbor(&self, n: u32) -> HexPosition {
        let n = n % 6;
        self.neighbors()[n as usize]
    }

    pub fn neighbors(&self) -> [HexPosition; 6] {
        let HexPosition {x, y, z} = *self;
        [
            HexPosition::new(x+1, y-1, z),
            HexPosition::new(x+1, y,   z-1),

            HexPosition::new(x,   y+1, z-1),
            HexPosition::new(x-1, y+1, z),

            HexPosition::new(x-1, y,   z+1),
            HexPosition::new(x,   y-1, z+1),
        ]
    }

    pub fn distance_to(&self, other: &HexPosition) -> u32 {
        fn abs(i: i32) -> i32 { if i < 0 { -i } else { i } }

        let HexPosition{x: ax, y: ay, z: az} = *self;
        let HexPosition{x: bx, y: by, z: bz} = *other;

        ((abs(ax - bx) + abs(ay - by) + abs(az - bz)) / 2) as u32
    }

    pub fn ring_with_radius(&self, radius: u32) -> RingIterator {
        let vec_to_start = HexPosition::origin().neighbors()[4] - HexPosition::origin();
        let vec_to_start = vec_to_start.scale(radius as i32);
        let pos_to_start = HexPosition::origin() + vec_to_start;

        RingIterator {
            radius: radius,
            next: pos_to_start,
            spent: false,
            i: 0,
            j: 0,
            offset: *self - HexPosition::origin(),
        }
    }

    pub fn spiral(&self) -> SpiralIterator {
        SpiralIterator {
            start_pos: *self,
            cur_radius: 0,
            max_radius: None,
            iter: self.ring_with_radius(0),
        }
    }

    pub fn spiral_to_radius(&self, radius: u32) -> SpiralIterator {
        SpiralIterator {
            start_pos: *self,
            cur_radius: 0,
            max_radius: Some(radius),
            iter: self.ring_with_radius(0),
        }
    }
    pub fn rays(&self) -> [RayIterator; 6] {
        let directions = HexPosition::origin().neighbors();
        [
            RayIterator {
                next: *self,
                vector: Vector{inner: directions[0]},
            },
            RayIterator {
                next: *self,
                vector: Vector{inner: directions[1]},
            },
            RayIterator {
                next: *self,
                vector: Vector{inner: directions[2]},
            },
            RayIterator {
                next: *self,
                vector: Vector{inner: directions[3]},
            },
            RayIterator {
                next: *self,
                vector: Vector{inner: directions[4]},
            },
            RayIterator {
                next: *self,
                vector: Vector{inner: directions[5]},
            },
        ]
    }

	pub fn ray(&self, direction: u32) -> RayIterator {
        let direction = direction % 6;
        let direction = HexPosition::origin().neighbors()[direction as usize];
		RayIterator {
			next: *self,
            vector: Vector {inner: direction}
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RayIterator {
	next: HexPosition,
	vector: Vector,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpiralIterator {
    start_pos: HexPosition,
    cur_radius: u32,
    max_radius: Option<u32>,
    iter: RingIterator,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RingIterator {
    radius: u32,
    next: HexPosition,
    spent: bool,
    i: u32,
    j: u32,
    offset: Vector,
}

impl Iterator for RayIterator {
	type Item = HexPosition;

	fn next(&mut self) -> Option<HexPosition> {
		let ret = self.next;
		self.next = self.next + self.vector;
		Some(ret)
	}
}

impl Iterator for SpiralIterator {
    type Item = HexPosition;
    fn next(&mut self) -> Option<HexPosition> {
        if let Some(next) = self.iter.next() {
            return Some(next);
        } else {
            if Some(self.cur_radius) == self.max_radius {
                return None;
            }

            self.cur_radius += 1;
            self.iter = self.start_pos.ring_with_radius(self.cur_radius);
            return self.next();
        }
    }
}

impl Iterator for RingIterator {
    type Item = HexPosition;
    fn next(&mut self) -> Option<HexPosition> {
        if self.spent || self.i >= 6 {
            return None;
        }

        if self.radius == 0 {
            self.spent = true;
            return Some(self.next + self.offset);
        }

        if self.j < self.radius {
            self.j += 1;
            self.next = self.next.neighbors()[self.i as usize];
            return Some(self.next); // + self.offset);
        } else {
            self.j = 0;
            self.i += 1;
            return self.next();
        }
    }
}

// for i in 0 .. 6
//   for j in 0 .. self.radius
//     yield self.next
//     self.next = self.next + self.offset

impl Vector {
    // Constructs a new vector.
    // (x + y + z) must equal 0.
    pub fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector{ inner: HexPosition::new(x, y, z) }
    }

    // Scales a vector in all directions by a constant factor.
    pub fn scale(&self, f: i32) -> Vector {
        let Vector{inner: HexPosition{ x, y, z }} = *self;
        Vector {
            inner: HexPosition {
                x: x * f,
                y: y * f,
                z: z * f,
            }
        }
    }
}

impl Sub for HexPosition {
    type Output = Vector;
    fn sub(self, HexPosition{x: bx, y: by, z: bz}: HexPosition) -> Vector {
        let HexPosition{x: ax, y: ay, z: az} = self;
        Vector {
            inner: HexPosition {
                x: ax - bx,
                y: ay - by,
                z: az - bz,
            }
        }
    }
}

impl Add<Vector> for HexPosition {
    type Output = HexPosition;
    fn add(self, Vector{inner: HexPosition{x: bx, y: by, z: bz}}: Vector) -> HexPosition {
        let HexPosition{x: ax, y: ay, z: az} = self;
        HexPosition {
            x: ax + bx,
            y: ay + by,
            z: az + bz,
        }
    }
}

impl Sub<Vector> for HexPosition {
    type Output = HexPosition;
    fn sub(self, Vector{inner: HexPosition{x: bx, y: by, z: bz}}: Vector) -> HexPosition {
        let HexPosition{x: ax, y: ay, z: az} = self;
        HexPosition {
            x: ax - bx,
            y: ay - by,
            z: az - bz,
        }
    }
}
