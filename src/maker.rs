use std::{fs::File, io::Write, process::Command};

use crate::{
    cairo::CairoFiles,
    circular_builder,
    generator::Generator,
    hexagonal_builder,
    job::{Job, MazeType, Size},
    rectangular_builder,
    svg_painter::paint_shapes,
    triangular_builder, storage::Storage,
};

pub struct MazeMaker {}

impl MazeMaker {
    pub fn make(job: &Job) {
        let (graph, shapes) = match job.maze_type {
            MazeType::Rectangular => {
                let (w, h) = match job.size {
                    Size::Small => (10, 7),
                    Size::Medium => (15, 10),
                    Size::Large => (20, 15),
                    Size::Huge => (40, 20),
                };
                rectangular_builder::Builder::new(w, h).build()
            }
            MazeType::Triangular => {
                let s = match job.size {
                    Size::Small => 5,
                    Size::Medium => 7,
                    Size::Large => 10,
                    Size::Huge => 20,
                };
                triangular_builder::Builder::new(s).build()
            }
            MazeType::Circular => {
                let s = match job.size {
                    Size::Small => 5,
                    Size::Medium => 7,
                    Size::Large => 8,
                    Size::Huge => 10,
                };
                circular_builder::Builder::new(s).build()
            }
            MazeType::Hexagonal => {
                let s = match job.size {
                    Size::Small => 5,
                    Size::Medium => 7,
                    Size::Large => 10,
                    Size::Huge => 20,
                };
                hexagonal_builder::Builder::new(s).build()
            }
        };
        let mut generator = Generator::new();
        let is_solvable = false;
        let instance = generator.generate(&graph, is_solvable);


        // paint as svg
        let with_solution = false;
                let svg = paint_shapes(with_solution, &shapes, &instance);

        Storage::save_file(&job.svg, svg.as_bytes().to_vec(), "image/svg+xml" );

 
/*
        // save graph, instance, solution for cairo
        let cairo = CairoFiles::new(&graph);
        cairo
            .create_structure_file(format!("{}maze.mas", path_prefix), &graph)
            .expect("cannot write structure file");
        cairo
            .create_instance_file(format!("{}maze.mai", path_prefix), &graph, &instance)
            .expect("cannot write instance file");
        cairo
            .create_path_file(format!("{}maze.map", path_prefix), &graph, &instance)
            .expect("cannot write path file");

        let output = 
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        ;
  */
  }
}
