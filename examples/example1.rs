use mapgen::{
    filter::{
        AreaStartingPosition, CellularAutomata, CullUnreachable, NoiseGenerator, XStart, YStart,
    },
    MapBuilder, NoData,
};

fn main() {
    let map = MapBuilder::<NoData>::new(20, 20)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .build();

    println!("{:}", &map);
}
