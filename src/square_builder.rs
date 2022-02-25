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
        let mut shapes = Shapes{};
        /*
        final boolean isPolar = false;
        final int height = properties.getInt("height");
        final int width = properties.getInt("width");
        final int margin = properties.getInt("margin");

        // width and height in pixels
        final int h = rsp * height;
        final int w = rsp * width;

        allShapes = new Shapes(isPolar, h, w, margin);
        */

        // rooms
        for y in 0..self.height {
            for x in 0..self.width {
                let room_id = graph.add_room();
                assert!(room_id == y * self.width + x, "Inconsistent room numbering");
                /*final int x1 = x * rsp + rsp / 2;
                final int y1 = y * rsp + rsp / 2;
                final Point2DInt position = new Point2DInt(x1, y1);
                final MarkShape mark = new MarkShape(roomId, position);
                allShapes.add(mark);
                final FloorShape floor = new FloorShape(roomId, position);
                allShapes.add(floor);
                */
            }
        }

        // walls - east/west
        for y in 0..self.height {
            for x in -1..self.width {
                let roomIdWest = y * self.width + x;
                let roomIdEast = roomIdWest + 1;
                /*final Point2DInt p1 = new Point2DInt(rsp * (x + 1), rsp * y);
                final Point2DInt p2 = new Point2DInt(rsp * (x + 1), rsp * (y + 1));
*/
                if x >= self.width - 1 {
                    // east outer wall
  //                  allShapes.add(WallShape.newOuterWall(p1, p2, roomIdWest, -1));

                } else if x < 0 {
                    // west outer wall
    //                allShapes.add(WallShape.newOuterWall(p1, p2, -1, roomIdEast));
                } else {

                    // inner wall
                    let id = graph.add_wall(roomIdWest, roomIdEast);
                   // allShapes.add(WallShape.newInnerWall(id, p1, p2, roomIdWest, roomIdEast));

                }
            }
        }

        // walls - south/north
        for y in -1..self.height {
            for x in 0..self.width {
                let roomIdNorth = y * self.width + x;
                let roomIdSouth = roomIdNorth + self.width;
               // final Point2DInt p1 = new Point2DInt(rsp * x, rsp * (y + 1));
                //final Point2DInt p2 = new Point2DInt(rsp * (x + 1), rsp * (y + 1));

                if y < 0 {
                  //  allShapes.add(WallShape.newOuterWall(p1, p2, roomIdSouth, -1));
                } else if y >= self.height - 1 {
                    // south outer wall
                    //allShapes.add(WallShape.newOuterWall(p1, p2, -1, roomIdNorth));
                } else {
                    // inner wall
                    let id = graph.add_wall(roomIdNorth, roomIdSouth);
                    //allShapes.add(WallShape.newInnerWall(id, p1, p2, roomIdSouth, roomIdNorth));
                }
            }
        }
        graph.start_room = 0;
        graph.target_room = self.width * self.height - 1;
        (graph, shapes)
        
    }
}