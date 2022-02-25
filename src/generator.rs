use crate::instance::Instance;
use crate::randomness::Randomness;
use crate::graph::Graph;
use std::panic;
use log::debug;

use fixedbitset::FixedBitSet;

pub struct Generator {
    //private final Random randomGenerator;
    visited_rooms: FixedBitSet,
    stack: Vec<i32>
}

// Generates random maze opening walls by random walk in a depth first order.
impl Generator {
    pub fn new() -> Self {
        Generator{ visited_rooms: FixedBitSet::new(), stack: vec!()}
    }

    pub fn generate(&mut self, graph: &Graph, randomness: &mut Randomness) -> Instance {
        let mut result = Instance::new(graph.wall_count(), graph.start_room, graph.target_room);
 
        let all_rooms_cnt = graph.room_count() as usize;
        self.visited_rooms = FixedBitSet::new();
        self.visited_rooms.grow(all_rooms_cnt);

        self.stack = vec!();
        self.visit_room(graph.start_room);
        while !self.stack.is_empty() {
 
            let room = self.stack.last().unwrap();
            debug!(" observing room: {}", room);
            if *room == graph.target_room {
                // save solution
                assert!(result.solution.is_empty(), "Maze cannot have two solutions");
                result.solution = self.stack.clone();
               
                // do not continue from the target room, instead backtrace to
                // fill gaps
                self.stack.pop();
                continue;
            }
            
            let candidates = self.find_all_possible_next_rooms(graph, &result, *room);
            if candidates.is_empty() {
                // backtrace - no way to go
                debug!("backtrace ");
                self.stack.pop();
                continue;
            }

            //debug!(" candidates: {}" , candidates.toString());

            // select next room
            let mut choice = 0;
            if candidates.len() > 1 {
                choice = randomness.get(candidates.len() as u32);
            }
            let wall = candidates[choice as usize];
            debug!(" opening wall {}", wall);
            result.set_wall_closed(wall, false);
            let otherRoom = graph.get_room_behind_wall(*room, wall);
            self.visit_room(otherRoom);
        }
        
       result
    }

    fn find_all_possible_next_rooms(&self, graph: &Graph, real: &Instance, room: i32) -> Vec<i32>  {
        let mut candidates= vec!();
        for wall in graph.get_walls(room) {
            if real.is_wall_closed(*wall) {
                let other_room = graph.get_room_behind_wall(room, *wall);
                if !self.visited_rooms[other_room as usize] {
                    candidates.push(*wall);
                }
            }
        }
            return candidates;
        
    }

    fn visit_room(&mut self, room: i32) {
        assert!(!self.visited_rooms[room as usize], "Cannot visit the same room twice");
        debug!("visiting room {}", room);
        self.visited_rooms.set(room as usize, true);
        self.stack.push(room);
    }
}