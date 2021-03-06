use super::*;
pub use self::hex_grid::*;

mod hex_grid;

pub trait Grid {
    type Iter: Iterator<Item=HexPosition>;
    fn contains(&self, pos: &HexPosition) -> bool {
        self.get_index(pos).is_some()
    }
    fn array_size(&self) -> usize;
    fn get_index(&self, pos: &HexPosition) -> Option<usize>;
    fn inverse_index(&self, idx: usize) -> HexPosition;
    fn iter(&self) -> Self::Iter;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Map<T, G: Grid> {
    arr: Vec<Option<T>>,
    grid: G,
    max_count: usize,
    count: usize,
}

pub struct MapIterator<'a, T: 'a, G: Grid + 'a> {
    idx: usize,
    map: &'a Map<T, G>,
}

impl <T, G: Grid> Map<T, G> {
    pub fn grid(&self) -> &G {
        &self.grid
    }

    pub fn new(g: G) -> Map<T, G> {
        let size = g.array_size();
        let mut vec = Vec::with_capacity(size);
        for _ in 0 .. size {
            vec.push(None);
        }

        let available_count = g.iter().count();

        Map {
            arr: vec,
            grid: g,
            max_count: available_count,
            count: 0,
        }
    }

    pub fn insert(&mut self, pos: &HexPosition, value: T) -> Option<T> {
        let idx = if let Some(idx) = self.grid.get_index(pos) {
            idx
        } else {
            return None;
        };

        let slot = self.arr.get_mut(idx).unwrap();
        let ret = slot.take();
        *slot = Some(value);
        if ret.is_none() {
            self.count += 1;
        }
        return ret;
    }

    pub fn remove(&mut self, pos: &HexPosition) -> Option<T> {
        let idx = if let Some(idx) = self.grid.get_index(pos) {
            idx
        } else {
            return None;
        };

        let slot = self.arr.get_mut(idx).unwrap();
        let ret = slot.take();
        if ret.is_some() {
            self.count += 1;
        }
        return ret;
    }

    pub fn contains(&self, pos: &HexPosition) -> bool {
        self.get(pos).is_some()
    }

    pub fn could_contain(&self, pos: &HexPosition) -> bool {
        self.grid.contains(pos)
    }

    pub fn get(&self, pos: &HexPosition) -> Option<&T> {
        let idx = if let Some(idx) = self.grid.get_index(pos) {
            idx
        } else {
            return None;
        };

        self.arr.get(idx).unwrap().as_ref()
    }

    pub fn get_mut(&mut self, pos: &HexPosition) -> Option<&mut T> {
        let idx = if let Some(idx) = self.grid.get_index(pos) {
            idx
        } else {
            return None;
        };

        self.arr.get_mut(idx).unwrap().as_mut()
    }

    pub fn entry(&mut self, pos: &HexPosition) -> Option<&mut Option<T>> {
        let idx = if let Some(idx) = self.grid.get_index(pos) {
            idx
        } else {
            return None;
        };

        self.arr.get_mut(idx)
    }

    pub fn iter(&self) -> MapIterator<T, G> {
        MapIterator {
            idx: 0,
            map: self
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn is_full(&self) -> bool {
        self.count == self.max_count
    }
}

impl <'a, T: 'a, G: 'a> Iterator for MapIterator<'a, T, G>
where G: Grid {
    type Item = (HexPosition, &'a T);
    fn next(&mut self) -> Option<(HexPosition, &'a T)> {
        // Find the next filled space
        let mut i = self.idx;
        while i < self.map.arr.len() && self.map.arr[i].is_none() {
            i += 1;
        }
        self.idx = i + 1;

        if i == self.map.arr.len() {
            return None;
        }

        let ret = (
            self.map.grid.inverse_index(i),
            self.map.arr.get(i).unwrap().as_ref().unwrap()
        );
        return Some(ret);
    }
}
