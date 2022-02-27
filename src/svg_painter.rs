use crate::{
    instance::{self, Instance},
    shapes::{Mapper, Shapes, WallType},
};
use xmlwriter::{Options, XmlWriter};

const SVG_URL: &str = "http://www.w3.org/2000/svg";

fn paint_line(xml: &mut XmlWriter, mapper: &Mapper, x1: i32, y1: i32, x2: i32, y2: i32, style: &str) {
    xml.start_element("line");
    xml.write_attribute("xmlns", SVG_URL);
    xml.write_attribute("x1", &mapper.map_x(x1, y1));
    xml.write_attribute("y1", &mapper.map_y(x1, y1));
    xml.write_attribute("x2", &mapper.map_x(x2, y2));
    xml.write_attribute("y2", &mapper.map_y(x2, y2));
    xml.write_attribute("style", style);
    xml.end_element();
}

fn paint_walls(xml: &mut XmlWriter, shapes: &Shapes, instance: &Instance) {
    for wall in &shapes.walls {
        let style = match wall.t {
            WallType::Inner => "stroke:black;stroke-width:1",
            WallType::Outer => "stroke:black;stroke-width:3",
        };
        if (wall.wall < 0) || instance.is_wall_closed(wall.wall) {
            paint_line(xml, &shapes.mapper, wall.x1, wall.y1, wall.x2, wall.y2, style);
        }
        /*        String style = "";

        switch (wallType) {
            case outerWall:
                style = "stroke:" + printStyle.getOuterWallColor().toSvg() + ";stroke-width:"
                        + printStyle.getOuterWallWidth();
                break;
            case innerWall:
                style = "stroke:" + printStyle.getInnerWallColor().toSvg() + ";stroke-width:"
                        + printStyle.getInnerWallWidth();
                break;

            case noWall:
                if (printStyle.isPrintAllWalls()) {
                    style = "stroke:" + printStyle.getDebugWallColor().toSvg() + ";stroke-width:"
                            + printStyle.getInnerWallWidth();
                }
                break;
            default:
                break;

        }
        */
        //if (!style.isEmpty()) {
        /*   if (doc.getContext().isPolarCoordinates() && p1.getY() == p2.getY()) {
            if (p1.getX() == 0 && p2.getX() == 0) {
                doc.printCircle(new Point2DInt(0, 0), "none", p1.getY(), false, style);
            } else {
                doc.printArcSegment(p1, p2, style);
            }
        } else {
            */
    }
}

pub fn paint_shapes(shapes: &Shapes, instance: &Instance) -> String {
    let mut xml = XmlWriter::new(Options::default());
    xml.start_element("svg");
    xml.write_attribute("xmlns", SVG_URL);
    xml.write_attribute_fmt(
        "viewBox",
        format_args!(
            "{} {} {} {}",
            0, 0, shapes.mapper.canvas_width, shapes.mapper.canvas_height
        ),
    );
    //w.start_element("text");
    // We can write any object that implements `fmt::Display`.
    //w.write_attribute("x", &10);
    //w.write_attribute("y", &20);
    //w.write_text_fmt(format_args!("length is {}", 5));
    paint_walls(&mut xml, &shapes, instance);
    let result = xml.end_document();
    result
}
