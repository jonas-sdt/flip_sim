use crate::units::*;
use heapless::{FnvIndexMap, Vec};
use itertools::Itertools;

type HashOutput = usize;

pub(crate) struct HashGrid<'a, T: HasPosition, const Len: usize> {
    hashmap: FnvIndexMap<(usize, usize), Vec<&'a T, Len>, Len>,
    spacing: f64,
}

impl<'a, T: HasPosition, const Len: usize> HashGrid<'a, T, Len> {
    pub(crate) fn new(mut array: [&'a T; Len], spacing: f64) -> Self {
        // sort into grid
        let mut hashmap: FnvIndexMap<(usize, usize), Vec<&'a T, Len>, Len> = FnvIndexMap::new();
        array.into_iter().for_each(|item| {
            hashmap[&Self::position_to_cell_coordinate(&spacing, &item.get_pos())].push(item);
        });

        Self { hashmap, spacing }
    }

    pub fn find(&mut self, pos: &Position, radius: f64) -> Vec<&'a T, Len> {
        let x = &pos.x;
        let y = &pos.y;
        let min_cell_x = ((x - radius) / self.spacing).floor() as usize;
        let max_cell_x = ((x + radius) / self.spacing).floor() as usize;
        let min_cell_y = ((y - radius) / self.spacing).floor() as usize;
        let max_cell_y = ((y + radius) / self.spacing).floor() as usize;

        let mut results = Vec::new();

        for cx in min_cell_x..=max_cell_x {
            for cy in min_cell_y..=max_cell_y {
                if let Some(objects) = self.hashmap.remove(&(cx, cy)) {
                    for obj in objects.into_iter() {
                        let dist_sq = (obj.get_pos().x - x).powi(2) + (obj.get_pos().y - y).powi(2);
                        if dist_sq <= radius.powi(2) {
                            results.push(obj);
                        }
                    }
                }
            }
        }

        results
    }

    fn position_to_cell_coordinate(spacing: &f64, pos: &Position) -> (usize, usize) {
        (
            (pos.x / spacing).floor() as usize,
            (pos.y / spacing).floor() as usize,
        )
    }
}
