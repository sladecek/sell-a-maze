use crate::graph::Graph;
use crate::shapes::Shapes;

const rsx: i32 = 120;
const rsy: i32 = 200;

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

        let mut shapes = Shapes::new(false, rsy * height, rsx * width, rsx);

        let mut prevFirst = -1;
        let mut lastRoom = -1;
        let mut myFirst = -1;

        // for all rows
        for y in 0..self.size + 1 {
            let roomsInRow = 2 * y + 1;
            if y < self.size {
                let prevRoom = -1;
                let mut rowBuilder = RowBuilder {
                    size: self.size,
                    prevFirst,
                    lastRoom,
                    myFirst,
                    y,
                    roomsInRow,
                    prevRoom,
                };
                rowBuilder.invoke(&mut graph, &mut shapes);
                myFirst = rowBuilder.myFirst;
                prevFirst = rowBuilder.prevFirst;
                lastRoom = rowBuilder.lastRoom;
            } else {
                prevFirst = myFirst;
            }
            // connect rooms to upper row by horizontal walls
            if prevFirst >= 0 {
                let mut i = 0;
                for j in (1..roomsInRow).step_by(2) {
                    let x = self.size + j - y - 1;
                    let mut r = -1;
                    if y < self.size {
                        r = myFirst + j;
                    }
                    addWallAndWallShape(&mut graph, &mut shapes, r, prevFirst + i, x, x + 2, y, y);
                    i += 2;
                }
            }
        }

        graph.start_room = 0;
        graph.target_room = lastRoom;
        (graph, shapes)
    }
}

fn addWallAndWallShape(
    graph: &mut Graph,
    shapes: &mut Shapes,
    roomLeft: i32,
    roomRight: i32,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
) {
    let (p1x, p1y) = (rsx * x1, rsy * y1);
    let (p2x, p2y) = (rsx * x2, rsy * y2);
    //LOGGER.log(Level.FINE, "addWallAndWallShape roomRight=" + roomRight + " roomLeft=" +
    //roomLeft + " y1=" + y1 + " y2=" + y2 + " x1=" + x1 + " x2=" + x2);

    if roomRight >= 0 && roomLeft >= 0 {
        let id = graph.add_wall(roomRight, roomLeft);

        shapes.add_inner_wall(id, p1x, p1y, p2x, p2y, roomRight, roomLeft);
    } else {
        shapes.add_outer_wall(p1x, p1y, p2x, p2y, roomRight, roomLeft);
    }
}
struct RowBuilder {
    pub size: i32,
    pub y: i32,
    pub roomsInRow: i32,
    pub prevFirst: i32,
    pub lastRoom: i32,
    pub myFirst: i32,
    pub prevRoom: i32,
}

impl RowBuilder {
    fn invoke(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        // make a row of rooms  and vertical walls among them
        for j in 0..self.roomsInRow + 1 {
            let x1 = self.size + j - self.y - 1;
            let x2 = self.size + j - self.y;
            let mut y1 = self.y;
            let mut y2 = self.y;
            let mut newRoomIsRight = true;
            if j % 2 == 0 {
                y1 += 1;
            } else {
                y2 += 1;
                newRoomIsRight = false;
            }

            let mut r = -1;
            if j < self.roomsInRow {
                r = graph.add_room();
                if j == 0 {
                    self.prevFirst = self.myFirst;
                    self.myFirst = r;
                }
                self.lastRoom = r;
                //LOGGER.log(Level.FINE, "addRoom " + r + " y=" + y + " j=" + j + " prevRoom=" + prevRoom +
                //      " myFirst=" + myFirst + " prevFirst=" + prevFirst + " lastRoom=" + lastRoom);

                shapes.add_floor(r, rsx * x2, rsy * self.y + rsy / 2);
                /*
                                  final Point2DInt position = new Point2DInt(r);
                                  //final MarkShape mark = new MarkShape(r, position);
                                  shapes.add (mark);
                                  final FloorShape floor = new FloorShape(r, position);
                                  allShapes.add(floor);
                */
            }

            if newRoomIsRight {
                addWallAndWallShape(graph,  shapes, r, self.prevRoom, x1, x2, y1, y2);
            } else {
                addWallAndWallShape(graph,  shapes, self.prevRoom, r, x1, x2, y1, y2);
            }
            self.prevRoom = r;
        }
    }
}
