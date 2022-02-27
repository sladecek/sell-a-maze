
/// Collection of printable shapes.

#[derive(Debug)]
pub struct Shapes {
    pub is_polar:bool,
    pub height:i32,
    pub width:i32,
    pub margin:i32,
    pub floors: Vec<Floor>,
    pub walls: Vec<Wall>,
    pub mapper: Mapper
}

impl Shapes {
    pub fn new(is_polar: bool, height: i32, width:i32, margin: i32) -> Self {
        Shapes { is_polar, height, width, margin, floors:vec!(), walls: vec!(), mapper: Mapper::new(is_polar, height, width, margin)}
    }

    pub fn add_floor(&mut self, room:i32, x:i32, y:i32) {
        self.floors.push(Floor{room, x,y})
    }

    pub fn add_outer_wall(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, right_face: i32, left_face: i32) {
        self.walls.push(Wall{t:WallType::Outer, wall: -1, right_face, left_face,x1, y1,x2,y2 });
    }

    pub fn add_inner_wall(&mut self, wall: i32, x1: i32, y1: i32, x2: i32, y2: i32, right_face: i32, left_face: i32) {
        self.walls.push(Wall{t:WallType::Inner, wall, right_face, left_face,x1, y1,x2,y2 });
    }
}


#[derive(Debug)]
pub struct Floor {
    pub room: i32,
    pub x: i32,
    pub y: i32
} 

#[derive(Debug)]
pub enum WallType {
    Inner, Outer
}

#[derive(Debug)]
pub struct Wall {
    pub t: WallType,
    pub wall: i32,
    pub right_face: i32,
    pub left_face: i32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32
}


#[derive(Debug)]
pub struct Mapper
{
    pub is_polar: bool,
    pub canvas_height: i32,
    pub canvas_width: i32,
    pub zero_point_x: i32,
    pub zero_point_y: i32,
}

impl Mapper
{
    pub fn new(is_polar: bool, height: i32, width: i32, margin: i32) -> Self {
        if is_polar {
            unimplemented!()
        } else {
            Mapper{is_polar, canvas_height: height + 2*margin, canvas_width: width+2*margin, zero_point_x: margin, zero_point_y: margin}
        }
    }

    pub fn map_x(&self, x: i32, _y: i32) -> i32 {
        self.zero_point_x + x
    }

    pub fn map_y(&self, _x: i32, y: i32) -> i32 {
        self.zero_point_y + y
    }
}