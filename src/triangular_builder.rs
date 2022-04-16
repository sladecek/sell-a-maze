use crate::graph::Graph;
use crate::shapes::Shapes;

const RSX: i32 = 120;
const RSY: i32 = 200;

pub struct Builder {
    size: i32,
}

impl Builder {
    pub fn new(size: i32) -> Self {
        Builder { size }
    }

    pub fn build(&self) -> (Graph, Shapes) {
        let mut graph = Graph::new();

        let height = self.size;
        let width = 2 * self.size;

        let mut shapes = Shapes::new(false, RSY * height, RSX * width, RSX);

        let mut prev_first = -1;
        let mut last_room = -1;
        let mut my_first = -1;

        // for all rows
        for y in 0..self.size + 1 {
            let rooms_in_row = 2 * y + 1;
            if y < self.size {
                let prev_room = -1;
                let mut row_builder = RowBuilder {
                    size: self.size,
                    prev_first,
                    last_room,
                    my_first,
                    y,
                    rooms_in_row,
                    prev_room,
                };
                row_builder.invoke(&mut graph, &mut shapes);
                my_first = row_builder.my_first;
                prev_first = row_builder.prev_first;
                last_room = row_builder.last_room;
            } else {
                prev_first = my_first;
            }
            // connect rooms to upper row by horizontal walls
            if prev_first >= 0 {
                let mut i = 0;
                for j in (1..rooms_in_row).step_by(2) {
                    let x = self.size + j - y - 1;
                    let mut r = -1;
                    if y < self.size {
                        r = my_first + j;
                    }
                    add_wall_and_wall_shape(&mut graph, &mut shapes, r, prev_first + i, x, x + 2, y, y);
                    i += 2;
                }
            }
        }

        graph.start_room = 0;
        graph.target_room = last_room;
        (graph, shapes)
    }
}

fn add_wall_and_wall_shape(
    graph: &mut Graph,
    shapes: &mut Shapes,
    room_left: i32,
    room_right: i32,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
) {
    let (p1x, p1y) = (RSX * x1, RSY * y1);
    let (p2x, p2y) = (RSX * x2, RSY * y2);
    //LOGGER.log(Level.FINE, "addWallAndWallShape roomRight=" + roomRight + " roomLeft=" +
    //roomLeft + " y1=" + y1 + " y2=" + y2 + " x1=" + x1 + " x2=" + x2);

    if room_right >= 0 && room_left >= 0 {
        let id = graph.add_wall(room_right, room_left);

        shapes.add_inner_wall(id, p1x, p1y, p2x, p2y, room_right, room_left);
    } else {
        shapes.add_outer_wall(p1x, p1y, p2x, p2y, room_right, room_left);
    }
}
struct RowBuilder {
    pub size: i32,
    pub y: i32,
    pub rooms_in_row: i32,
    pub prev_first: i32,
    pub last_room: i32,
    pub my_first: i32,
    pub prev_room: i32,
}

impl RowBuilder {
    fn invoke(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        // make a row of rooms  and vertical walls among them
        for j in 0..self.rooms_in_row + 1 {
            let x1 = self.size + j - self.y - 1;
            let x2 = self.size + j - self.y;
            let mut y1 = self.y;
            let mut y2 = self.y;
            let mut new_room_is_right = true;
            if j % 2 == 0 {
                y1 += 1;
            } else {
                y2 += 1;
                new_room_is_right = false;
            }

            let mut r = -1;
            if j < self.rooms_in_row {
                r = graph.add_room();
                if j == 0 {
                    self.prev_first = self.my_first;
                    self.my_first = r;
                }
                self.last_room = r;
                  shapes.add_floor(r, RSX * x2, RSY * self.y + RSY / 2);
            }

            if new_room_is_right {
                add_wall_and_wall_shape(graph,  shapes, r, self.prev_room, x1, x2, y1, y2);
            } else {
                add_wall_and_wall_shape(graph,  shapes, self.prev_room, r, x1, x2, y1, y2);
            }
            self.prev_room = r;
        }
    }
}

#[test]
fn size_is_correct() {
   let bld = Builder::new(1);
   let (_, shapes) = bld.build();
   assert_eq!(200, shapes.height);
   assert_eq!(240, shapes.width);
}