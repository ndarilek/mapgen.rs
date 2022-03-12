use mapgen::*;
use rand::prelude::*;

fn main() {
    let mut rng = StdRng::seed_from_u64(907647352);
    let gen = BspInterior::<NoData>::new();
    let map = gen.modify_map(&mut rng, &Map::new(20, 10));
    println!("{:}", &map);
}
