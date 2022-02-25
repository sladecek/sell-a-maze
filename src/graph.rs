use std::panic;
use log::error;

pub struct Graph {
    rooms: Vec<Vec<i32>>,
    wall_room1: Vec<i32>,
    wall_room2: Vec<i32>,
    pub start_room: i32,
    pub target_room: i32,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            rooms: vec![],
            wall_room1: vec![],
            wall_room2: vec![],
            start_room: 0,
            target_room: 0,
        }
    }

    pub fn room_count(&self) -> i32 {
        self.rooms.len() as i32
    }

    pub fn wall_count(&self) -> i32 {
        self.wall_room1.len() as i32
    }

    pub fn add_room(&mut self) -> i32 {
        let id = self.room_count();
        self.rooms.push(vec![]);
        id
    }

    pub fn get_walls(&self, room: i32) -> &Vec<i32> {
        &self.rooms[room as usize]
    }

    pub fn add_wall(&mut self, room1: i32, room2: i32) -> i32 {
        if room1 >= self.room_count() || room2 >= self.room_count() {
            error!("cannot add wall room1={} room2={}", room1, room2);
            return 0;
        } 
        let id = self.wall_count();
        self.wall_room1.push(room1);
        self.rooms[room1 as usize].push(id);
        self.wall_room2.push(room2);
        self.rooms[room2 as usize].push(id);
        id
    }

    pub fn get_room_behind_wall(&self, room: i32, wall: i32) -> i32 {
        if wall < 0 || wall >= self.wall_room1.len() as i32 || wall >= self.wall_room2.len() as i32 {
            panic!("Unknown wall {}", wall)
        }
        let r1 = self.wall_room1[wall as usize];
        let r2 = self.wall_room2[wall as usize];
        if r1 == room {
            return r2;
        } else if r2 == room {
            return r1;
        } else {
            panic!("unknown room");
        }
    }
}

#[test]
fn graph_test() {
    let mut graph = Graph::new();
    assert_eq!(0,graph.add_room());
    assert_eq!(1,graph.add_room());
    assert_eq!(2,graph.add_room());
    assert_eq!(0,graph.add_wall(0, 1));
    assert_eq!(1,graph.add_wall(1, 2));
    assert_eq!(3, graph.room_count());
    assert_eq!(2, graph.wall_count());
    assert_eq!(1, graph.get_room_behind_wall(0, 0));
    assert_eq!(0, graph.get_room_behind_wall(1, 0));

    assert_eq!(vec![0i32,1i32], *graph.get_walls(1))
}
