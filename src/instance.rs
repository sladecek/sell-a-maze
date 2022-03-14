use fixedbitset::FixedBitSet;

/// Defines concrete maze by picking opened walls among all possible walls in the maze. This also contains solution.
#[derive(Debug)]
pub struct Instance {
    wall_count: i32,
    pub start_room: i32,
    pub target_room: i32,
    is_wall_closed: FixedBitSet,
    pub solution: Vec<i32>,
}

impl Instance {
    pub fn new(wall_count: i32, start_room: i32, target_room: i32) -> Self {
        let mut is_wall_closed = FixedBitSet::new();
        is_wall_closed.grow(wall_count as usize);
        is_wall_closed.set_range(.., true);
        Instance {
            wall_count,
            start_room,
            target_room,
            is_wall_closed,
            solution: vec![],
        }
    }

    pub fn is_wall_closed(&self, wall: i32) -> bool {
        self.is_wall_closed[wall as usize]
    }

    pub fn set_wall_closed(&mut self, wall: i32, value: bool) {
        self.is_wall_closed.set(wall as usize, value);
    }
}
