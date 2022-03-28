use log::info;
use rand::Rng;
use std::{fs::File, io::Write, process::Command};

use crate::{
    cairo::CairoFiles,
    circular_builder,
    generator::Generator,
    hexagonal_builder,
    job::{Job, MazeType, Size},
    rectangular_builder,
    storage::Storage,
    svg_painter::paint_shapes,
    triangular_builder,
};

pub struct MazeMaker {}

impl MazeMaker {
    pub fn make(job: &Job) {
        let (graph, shapes) = match job.maze_type {
            MazeType::Rectangular => {
                let (w, h) = match job.size {
                    Size::Small => (10, 6),
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
        let mut is_solvable = true;
        if !job.guaranteed {
            if rand::thread_rng().gen_range(1..7) <= 1 {
                is_solvable = false;
            }
        }
        let instance = generator.generate(&graph, is_solvable);

        // paint as svg
        let with_solution = false;
        let svg = paint_shapes(with_solution, &shapes, &instance);

        Storage::save_file(&job.svg, svg.as_bytes().to_vec(), "image/svg+xml");

        let pdf = svg2pdf::convert_str(&svg, svg2pdf::Options::default()).unwrap();
        Storage::save_file(&job.pdf, pdf, "application/pdf");

        if !job.guaranteed {
            return;
        }

        // save graph, instance, solution for cairo
        let cairo = CairoFiles::new(&graph);
        let maze_structure = cairo
            .create_structure_file(&graph)
            .expect("cannot write structure file");

        {
            let mut fms = File::create("work/maze.mas").unwrap();
            fms.write(&maze_structure);
        }

        Storage::save_file(
            &job.maze_structure,
            maze_structure,
            "application/octet-stream",
        );

        let maze_instance = cairo
            .create_instance_file(&graph, &instance)
            .expect("cannot write instance file");

        {
            let mut fmi = File::create("work/maze.mai").unwrap();
            fmi.write(&maze_instance);
        }

        Storage::save_file(
            &job.maze_instance,
            maze_instance,
            "application/octet-stream",
        );

        let maze_path = cairo
            .create_path_file(&graph, &instance)
            .expect("cannot write path file");

        {
            let mut fmp = File::create("work/maze.map").unwrap();
            fmp.write(&maze_path);
        }
        // Path is not stored. It is used localy to make proof.

        let output = Command::new("bash")
            .arg("-c")
            .arg("work/validate.sh")
            .output()
            .expect("failed to execute 'work/validate.sh'");

        

        let mut protocol = format!("{:?}", output);
        log::info!("{:?}", protocol);
        if output.status.success() {
            protocol = String::from_utf8(output.stdout).unwrap();
        }

        Storage::save_file(
            &job.protocol,
            protocol.as_bytes().to_vec(),
            "text/plain",
        );
        
    }
}
