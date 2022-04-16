use crate::graph::{Graph};
use crate::instance::Instance;
use rand::Rng;
use log::debug;

use fixedbitset::FixedBitSet;

pub struct Generator {
    visited_rooms: FixedBitSet,
    stack: Vec<i32>,
}

// Generates random maze opening walls by random walk in a depth first order.
impl Generator {
    pub fn new() -> Self {
        Generator {
            visited_rooms: FixedBitSet::new(),
            stack: vec![],
        }
    }

    pub fn generate(
        &mut self,
        graph: &Graph,
        is_solvable: bool,
    ) -> Instance {
        let mut rng = rand::thread_rng();
        let mut result = Instance::new(graph.wall_count(), graph.start_room, graph.target_room);

        let all_rooms_cnt = graph.room_count() as usize;
        self.visited_rooms = FixedBitSet::new();
        self.visited_rooms.grow(all_rooms_cnt);

        self.stack = vec![];
        self.visit_room(graph.start_room);
        while !self.stack.is_empty() {
            let room = *self.stack.last().unwrap();
            debug!(" observing room: {}", room);
            if room == graph.target_room {
                // save solution
                assert!(result.solution.is_empty(), "Maze cannot have two solutions");
                result.solution = self.stack.clone();

                // do not continue from the target room, instead backtrace to
                // fill gaps
                self.stack.pop();
                continue;
            }

            let candidates = self.find_all_possible_next_rooms(graph, &result, room);
            if candidates.is_empty() {
                // backtrace - no way to go
                debug!("backtrace ");
                self.stack.pop();
                continue;
            }
            // select next room
            let mut choice = 0;
            if candidates.len() > 1 {
                choice = rng.gen_range(0..candidates.len() as u32);
            }
            let wall = candidates[choice as usize];
            debug!(" opening wall {}", wall);
            result.set_wall_closed(wall, false);
            self.visit_room(graph.get_room_behind_wall(room, wall));
        }

        if !is_solvable {
            self.destroy_solution(&mut result, graph);
        }
        result
    }

    fn find_all_possible_next_rooms(&self, graph: &Graph, real: &Instance, room: i32) -> Vec<i32> {
        let mut candidates = vec![];
        for wall in graph.get_walls(room) {
            if real.is_wall_closed(*wall) {
                let other_room = graph.get_room_behind_wall(room, *wall);
                if !self.visited_rooms[other_room as usize] {
                    candidates.push(*wall);
                }
            }
        }
        candidates
    }

    fn visit_room(&mut self, room: i32) {
        assert!(
            !self.visited_rooms[room as usize],
            "Cannot visit the same room twice"
        );
        debug!("visiting room {}", room);
        self.visited_rooms.set(room as usize, true);
        self.stack.push(room);
    }

    fn destroy_solution(&self, result: &mut Instance, graph: &Graph) {
        let half = result.solution.len() / 2;
        if half+1 >= result.solution.len()
        {
            return
        }         
        let room1 = result.solution[half];
        let room2 = result.solution[half+1];
        let wall = graph.get_wall_between_rooms(room1, room2);
        result.set_wall_closed(wall, true);
        
    }
}
