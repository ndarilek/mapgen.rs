//! Add exit point to the map
//!
//! This modifier will try to add exit point as far as possible from the starting point.
//! It means that starting point have to be set before this Modyfier will start.
//!

use crate::dijkstra::DijkstraMap;
use crate::geometry::Point;
use crate::Map;
use crate::MapFilter;
use rand::prelude::StdRng;
use std::f32;
use std::marker::PhantomData;

/// Add exist position to the map based on the distance from the start point.
pub struct DistantExit<D> {
    phantom: PhantomData<D>,
}

impl<D: Clone + Default> MapFilter<D> for DistantExit<D> {
    fn modify_map(&self, _: &mut StdRng, map: &Map<D>) -> Map<D> {
        self.build(map)
    }
}

impl<D: Clone + Default> DistantExit<D> {
    #[allow(dead_code)]
    pub fn new() -> Box<DistantExit<D>> {
        Box::new(DistantExit {
            phantom: PhantomData,
        })
    }

    fn build(&self, map: &Map<D>) -> Map<D> {
        let mut new_map = map.clone();

        let mut best_idx = 0;
        let mut best_value = 0.0;
        let dijkstra_map = DijkstraMap::new(map);
        for (i, &value) in dijkstra_map.tiles.iter().enumerate() {
            if value < f32::MAX && value > best_value {
                best_value = value;
                best_idx = i;
            }
        }
        let x = best_idx % map.width;
        let y = best_idx / map.width;
        new_map.exit_point = Some(Point::new(x, y));
        new_map
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::MapFilter;
    use super::*;
    use crate::geometry::Point;
    use crate::map::{Map, NoData};
    use rand::prelude::*;

    #[test]
    fn test_exit() {
        let map_str = "
        ##########
        #        #
        #  #     #
        ##########
        ";
        let mut map = Map::<NoData>::from_string(map_str);
        map.starting_point = Some(Point::new(9, 2));

        let modifier = DistantExit::new();
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.exit_point, Some(Point::new(1, 2)));
    }
}
