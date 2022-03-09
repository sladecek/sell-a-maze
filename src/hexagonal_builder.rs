use crate::graph::Graph;
use crate::shapes::Shapes;


pub struct Builder {
    size: i32
}

 // Radius of the hexagon.
const hP: i32 = 20;
 /// Half-height of the hexagon
const hH: i32 = 17; // TODO f32::floor((hP as f32) * f32::sqrt(3f32) / 2f32) as i32;


impl  Builder {
    pub fn new(size: i32) -> Self {
        Builder { size }
    }

    pub fn build(&self) -> (Graph, Shapes) {

        let mut graph = Graph::new();
       
        let height = hH * (2 * self.size + 1);
        let width = hP * (3 * self.size - 1);
        
        let mut shapes = Shapes::new(false, height, width, hP);        
        
        let roomsPerRow = 2 * self.size - 1;

        let mut mapXY2room: Vec<i32> = vec!();
        let wallXOffs: Vec<i32> = vec!(hP / 2, -hP / 2, -hP, -hP / 2, hP / 2, hP);
        let wallYOffs : Vec<i32> = vec!(-hH, -hH, 0, hH, hH, 0);
        let neighborRoomX : Vec<i32> = vec!(0, -1, -1, 0, 1, 1);
        let neighborRoomYOdd : Vec<i32> = vec!(-1, 0, 1, 1, 1, 0);
        let neighborRoomYEven : Vec<i32> = vec!(-1, -1, 0, 1, 0, -1);
        // make rooms
        for  x in 0 .. roomsPerRow {
            let isOdd = x % 2 == 1;
            for  y in  0 .. self.size {

                // make room (topology)
                let r = graph.add_room();
                mapXY2room.push(r);
                let center = computeRoomCenter(x, isOdd, y);

                //LOGGER.log(Level.FINE, "addRoom " + r + " y=" + y + " x=" + x + " center=" + center);

                //makeFloor(r, center);

                // make walls
                for  w  in  0..6 {
                    let w2 = (w + 1) % 6;

                    // wall endpoints
                    let x1 = center.0 + wallXOffs[w];
                    let y1 = center.1 + wallYOffs[w];
                    let x2 = center.0 + wallXOffs[w2];
                    let y2 = center.1 + wallYOffs[w2];

                    // the other room
                    let ox = x + neighborRoomX[w];
                    let mut oy = y;
                    if isOdd {
                        oy += neighborRoomYOdd[w];
                    } else {
                        oy += neighborRoomYEven[w];
                    }

                    // if the other room does not exist then this is a border
                    // wall
                    if !areRoomCoordinatesValid(roomsPerRow, ox, oy, self.size) {
                        shapes.add_outer_wall( x1, y1,x2, y2, -1, -1);
                    } else if w < 3 {
                        // Link only three rooms out of six. The other three
                        // walls will be linked in the from the other room
                        // (which does not exist yet).
                        let r2 = mapXY2room[(ox * self.size + oy) as usize];
                        
                        let id = graph.add_wall(r, r2);

                        //LOGGER.log(Level.FINE,
                        //        "addWallAndShape room1=" + r + " room2=" + r2 + " y1=" + y1 + " y2=" + y2 + " x1=" + x1 + " x2=" + x2);
                        shapes.add_inner_wall(id, x1, y1,x2, y2, -1, -1);
                    }

                }

            }
        }
        graph.start_room = 0;
        graph.target_room = self.size * roomsPerRow - 1;
        (graph, shapes)
        
    }



}


    // parameters of six walls of a hexagon
    // walls numbered counterclockwise, starting at upper room
 

    fn computeRoomCenter(x: i32, isOdd: bool, y: i32)->(i32,i32) {
        // compute centre of the room
        let xc = (hP * (2 + x * 3)) / 2;
        let mut yc = hH * (2 * y + 1);
        if isOdd {
            yc += hH;
        }
        (xc, yc)
    }


    fn areRoomCoordinatesValid(roomsPerRow: i32, ox: i32, oy: i32, size: i32) -> bool {
        ox >= 0 && ox < roomsPerRow && oy >= 0 && oy < size
    }
