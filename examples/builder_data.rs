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

impl MapFilter<MyData> for IncrementData {
    fn modify_map(
        &self,
        _rng: &mut rand::prelude::StdRng,
        map: &mapgen::Map<MyData>,
    ) -> mapgen::Map<MyData> {
        let mut map = map.clone();
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
        .with(Box::new(IncrementData))
        .build();

    println!("{:}\n{}", map, map.data.value);
}
