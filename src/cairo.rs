use primes::{PrimeSet, Sieve};
use std::io::{self, Write};

use crate::{graph::Graph, instance::Instance};

pub struct CairoFiles {
    room_primes: Vec<u64>,
}

impl CairoFiles {
    pub fn new(graph: &Graph) -> Self {
        let mut room_primes: Vec<u64> = vec![];
        let mut pset = Sieve::new();

        for p in pset.iter().take(graph.room_count() as usize) {
            room_primes.push(p);
        }
        CairoFiles { room_primes }
    }

    pub fn create_structure_file(&self, graph: &Graph) -> io::Result<Vec<u8>> {
        let mut f: Vec<u8> = vec![];
        writeln!(f, "{}", graph.room_count())?;
        writeln!(f, "{}", graph.wall_count())?;
        for pr in &self.room_primes {
            writeln!(f, "{}", pr)?;
        }
        for w in 0..graph.wall_count() {
            let pw = self.room_primes[graph.wall_room1[w as usize] as usize]
                * self.room_primes[graph.wall_room2[w as usize] as usize];
            writeln!(f, "{}", pw)?;
        }
        Ok(f)
    }

    pub fn create_instance_file(&self, graph: &Graph, instance: &Instance) -> io::Result<Vec<u8>> {
        let mut f: Vec<u8> = vec![];
        for w in 0..graph.wall_count() {
            writeln!(f, "{}", if instance.is_wall_closed(w) { 1 } else { 0 })?;
        }
        Ok(f)
    }

    pub fn create_path_file(&self, graph: &Graph, instance: &Instance) -> io::Result<Vec<u8>> {
        let mut f: Vec<u8> = vec![];
        writeln!(f, "{}", instance.solution.len())?;
        for i in 0..instance.solution.len() {
            let r1 = instance.solution[i];
            writeln!(f, "{}", r1)?;
            if i < instance.solution.len() - 1 {
                let r2 = instance.solution[i + 1];
                let w = graph.get_wall_between_rooms(r1, r2);
                writeln!(f, "{}", w)?;
            }
        }
        Ok(f)
    }
}
