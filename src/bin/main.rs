use hash2maze::{generator::Generator, randomness::Randomness, square_builder::Builder, svg_painter::paint_shapes};
use num_bigint::BigUint;
use std::{str::FromStr, fs::File, io::Write};

fn main() {
    let mut randomness=Randomness::new(
        BigUint::from_str(
        "31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485",
        ).unwrap(),
        BigUint::from(10u32).pow(100));

    let builder = Builder::new(4, 4);
    let (graph, shapes) = builder.build();
    let mut generator = Generator::new();
    let instance = generator.generate(&graph, &mut randomness);
    print!("instance {:?}", instance);
    // save graph, instance, solution for cairo
    // paint as svg

    File::create("maze.svg").unwrap().write(paint_shapes(&shapes, &instance).as_bytes());
}
