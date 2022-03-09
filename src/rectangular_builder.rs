use crate::graph::Graph;
use crate::shapes::Shapes;

pub struct Builder {
    width: i32,
    height: i32
}

impl  Builder {
    pub fn new(width: i32, height: i32) -> Self {
        Builder { width, height}
    }

    pub fn build(&self) -> (Graph, Shapes) {

        let mut graph = Graph::new();
       
        let rsp = 10;
        let mut shapes = Shapes::new(false, rsp*self.height, rsp*self.width, rsp);
       
       
        // rooms
        for y in 0..self.height {
            for x in 0..self.width {
                let room_id = graph.add_room();
                assert!(room_id == y * self.width + x, "Inconsistent room numbering");
                let x1 = x * rsp + rsp / 2;
                let y1 = y * rsp + rsp / 2;
                shapes.add_floor(room_id, x1, y1);

            }
        }

        // walls - east/west
        for y in 0..self.height {
            for x in -1..self.width {
                let room_west = y * self.width + x;
                let room_east = room_west + 1;
                let p1x = rsp * (x + 1);
                let p1y = rsp * y;
                let p2x = p1x;
                let p2y = rsp * (y + 1);

                if x >= self.width - 1 {
                    // east outer wall
                    shapes.add_outer_wall(p1x, p1y,  p2x, p2y, room_west, -1);

                } else if x < 0 {
                    // west outer wall
                    shapes.add_outer_wall(p1x, p1y,  p2x, p2y, -1, room_east);
                } else {

                    // inner wall
                    let id = graph.add_wall(room_west, room_east);
                    shapes.add_inner_wall(id, p1x, p1y,  p2x, p2y, room_west, room_east);

                }
            }
        }

        // walls - south/north
        for y in -1..self.height {
            for x in 0..self.width {
                let room_north = y * self.width + x;
                let room_south = room_north + self.width;
                let p1x = rsp * x;
                let p1y = rsp * (y + 1);
                let p2x = rsp * (x + 1);
                let p2y = rsp * (y + 1);
                if y < 0 {
                  shapes.add_outer_wall(p1x, p1y,  p2x, p2y, room_south, -1);
                } else if y >= self.height - 1 {
                    // south outer wall
                    shapes.add_outer_wall(p1x, p1y,  p2x, p2y,  -1, room_north);
                } else {
                    // inner wall
                    let id = graph.add_wall(room_north, room_south);
                    shapes.add_inner_wall(id, p1x, p1y,  p2x, p2y,  room_south, room_north);
                }
            }
        }
        graph.start_room = 0;
        graph.target_room = self.width * self.height - 1;
        (graph, shapes)
        
    }
}