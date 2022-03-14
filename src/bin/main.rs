use num_bigint::BigUint;
use sell_a_maze::{
    cairo::CairoFiles, circular_builder, generator::Generator, hexagonal_builder,
    randomness::Randomness, rectangular_builder, svg_painter::paint_shapes, triangular_builder,
};
use std::{fs::File, io::Write, str::FromStr};

fn main() {
    let mut randomness=Randomness::new(
        BigUint::from_str(
        "31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485",
        ).unwrap(),
        BigUint::from(10u32).pow(100));

    let builder = rectangular_builder::Builder::new(20, 15);
    //let builder = triangular_builder::Builder::new(12);
    //let builder = hexagonal_builder::Builder::new(5);
    //let mut builder = circular_builder::Builder::new(6);
    let (graph, shapes) = builder.build();
    let mut generator = Generator::new();
    let is_solvable = false;
    let instance = generator.generate(&graph, &mut randomness, is_solvable );

    // paint as svg
    let with_solution = true;
    File::create("maze.svg")
        .unwrap()
        .write(paint_shapes(with_solution, &shapes, &instance).as_bytes())
        .unwrap();

    // save graph, instance, solution for cairo
    let cairo = CairoFiles::new(&graph);
    cairo.create_structure_file("maze.mas", &graph);
    cairo.create_instance_file("maze.mai", &graph, &instance);
    cairo.create_path_file("maze.map", &graph, &instance);
}
