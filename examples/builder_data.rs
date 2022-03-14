use mapgen::{
    filter::{
        AreaStartingPosition, CellularAutomata, CullUnreachable, NoiseGenerator, XStart, YStart,
    },
    MapBuilder, MapFilter,
};

#[derive(Clone, Default)]
struct MyData {
    value: usize,
}

struct IncrementData;

impl<D: Clone + Default> MapFilter<D> for IncrementData {
    fn modify_map(&self, rng: &mut rand::prelude::StdRng, map: &mapgen::Map<D>) -> mapgen::Map<D> {
        let map = map.clone();
        map.data.value += 1;
        map
    }
}

fn main() {
    let map = MapBuilder::<MyData>::new(20, 20)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .with(Box::new(IncrementData::<MyData>))
        .build();

    println!("{:}", &map);
}
