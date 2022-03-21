use crate::graph::Graph;
use crate::shapes::Shapes;

pub struct Builder {
    size: i32,
}

// Radius of the hexagon.
const H_P: i32 = 200;
/// Half-height of the hexagon
const H_H: i32 = 170; // TODO f32::floor((hP as f32) * f32::sqrt(3f32) / 2f32) as i32;

impl Builder {
    pub fn new(size: i32) -> Self {
        Builder { size }
    }

    pub fn build(&self) -> (Graph, Shapes) {
        let mut graph = Graph::new();

        let height = H_H * (2 * self.size + 1);
        let width = H_P * (3 * self.size - 1);

        let mut shapes = Shapes::new(false, height, width, H_P);

        let rooms_per_row = 2 * self.size - 1;

        let mut map_xy2room: Vec<i32> = vec![];
        let wall_x_offs: Vec<i32> = vec![H_P / 2, -H_P / 2, -H_P, -H_P / 2, H_P / 2, H_P];
        let wall_y_offs: Vec<i32> = vec![-H_H, -H_H, 0, H_H, H_H, 0];
        let neighbor_room_x: Vec<i32> = vec![0, -1, -1, 0, 1, 1];
        let neighbor_room_y_odd: Vec<i32> = vec![-1, 0, 1, 1, 1, 0];
        let neighbor_room_y_even: Vec<i32> = vec![-1, -1, 0, 1, 0, -1];
        // make rooms
        for x in 0..rooms_per_row {
            let is_odd = x % 2 == 1;
            for y in 0..self.size {
                // make room (topology)
                let r = graph.add_room();
                map_xy2room.push(r);
                let center = compute_room_center(x, is_odd, y);

                //LOGGER.log(Level.FINE, "addRoom " + r + " y=" + y + " x=" + x + " center=" + center);

                shapes.add_floor(r, center.0, center.1);
                //makeFloor(r, center);

                // make walls
                for w in 0..6 {
                    let w2 = (w + 1) % 6;

                    // wall endpoints
                    let x1 = center.0 + wall_x_offs[w];
                    let y1 = center.1 + wall_y_offs[w];
                    let x2 = center.0 + wall_x_offs[w2];
                    let y2 = center.1 + wall_y_offs[w2];

                    // the other room
                    let ox = x + neighbor_room_x[w];
                    let mut oy = y;
                    if is_odd {
                        oy += neighbor_room_y_odd[w];
                    } else {
                        oy += neighbor_room_y_even[w];
                    }

                    // if the other room does not exist then this is a border
                    // wall
                    if !are_room_coordinates_valid(rooms_per_row, ox, oy, self.size) {
                        shapes.add_outer_wall(x1, y1, x2, y2, -1, -1);
                    } else if w < 3 {
                        // Link only three rooms out of six. The other three
                        // walls will be linked in the from the other room
                        // (which does not exist yet).
                        let r2 = map_xy2room[(ox * self.size + oy) as usize];

                        let id = graph.add_wall(r, r2);

                        //LOGGER.log(Level.FINE,
                        //        "addWallAndShape room1=" + r + " room2=" + r2 + " y1=" + y1 + " y2=" + y2 + " x1=" + x1 + " x2=" + x2);
                        shapes.add_inner_wall(id, x1, y1, x2, y2, -1, -1);
                    }
                }
            }
        }
        graph.start_room = 0;
        graph.target_room = self.size * rooms_per_row - 1;
        (graph, shapes)
    }
}

// parameters of six walls of a hexagon
// walls numbered counterclockwise, starting at upper room

fn compute_room_center(x: i32, is_odd: bool, y: i32) -> (i32, i32) {
    // compute centre of the room
    let xc = (H_P * (2 + x * 3)) / 2;
    let mut yc = H_H * (2 * y + 1);
    if is_odd {
        yc += H_H;
    }
    (xc, yc)
}

fn are_room_coordinates_valid(rooms_per_row: i32, ox: i32, oy: i32, size: i32) -> bool {
    ox >= 0 && ox < rooms_per_row && oy >= 0 && oy < size
}
