use super::*;
use super::super::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HexGrid {
    radius: u32,
    min_r: i32,
    min_u: i32,
    max_r: i32,
    max_u: i32,
}

fn replace_if<T, F>(target: &mut Option<T>, other: T, f: F) where F: FnOnce(&T, &T) -> bool {
    if target.is_some() {
        let cur = target.as_mut().unwrap();
        if f(cur, &other) {
            *cur = other;
        }
    } else {
        *target = Some(other);
    }
}

impl HexGrid {
    pub fn new(radius: u32) -> HexGrid {
        let mut min_r = None;
        let mut min_u = None;

        let mut max_r = None;
        let mut max_u = None;

        for pos in HexPosition::origin().spiral_to_radius(radius) {
            let (r, u) = pos.as_axial();
            replace_if(&mut min_r, r, |c, r| r < c);
            replace_if(&mut min_u, u, |c, r| r < c);

            replace_if(&mut max_r, r, |c, r| r > c);
            replace_if(&mut max_u, u, |c, r| r > c);
        }

        HexGrid {
            min_r: min_r.unwrap_or(0),
            min_u: min_u.unwrap_or(0),
            max_r: max_r.unwrap_or(0),
            max_u: max_u.unwrap_or(0),
            radius: radius
        }
    }
}

impl Grid for HexGrid {
    type Iter = ::SpiralIterator;
    fn array_size(&self) -> usize {
        let delta_r = (self.max_r - self.min_r) as usize;
        let delta_u = (self.max_u - self.min_u) as usize;

        (delta_r + 1) * (delta_u + 1)
    }

    fn contains(&self, pos: &HexPosition) -> bool {
        HexPosition::origin().distance_to(pos) <= self.radius
    }

    fn get_index(&self, pos: &HexPosition) -> Option<usize> {
        let (r, u) = pos.as_axial();
        let delta_r = (self.max_r - self.min_r) as usize;

        let r = (r - self.min_r) as usize;
        let u = (u - self.min_u) as usize;


        if self.contains(pos) {
            Some(r + u * (delta_r + 1))
        } else {
            None
        }
    }

    fn iter(&self) -> Self::Iter {
        HexPosition::origin().spiral_to_radius(self.radius)
    }
}
