use crate::types::Room;
use image;
use image::{ImageBuffer, Rgb};
use std::io::Write;
pub fn render(dungeon_width:u32, dungeon_height:u32, rooms:&mut Vec<Room>, filename: String, central_points: &Vec<delaunator::Point>)
{
    let TILE_SIZE_PX = 5;
    println!("rendering {0} rooms", rooms.len());


    let mut image:ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(dungeon_width * TILE_SIZE_PX, dungeon_height * TILE_SIZE_PX);
    for pixel_x in 0..image.width()
    {
        for pixel_y in 0..image.height()
        {
            let mut in_room:bool = false;
            let dung_x = pixel_x / TILE_SIZE_PX;
            let dung_y = pixel_y / TILE_SIZE_PX;

            for room in &mut *rooms
            {
                in_room = in_room || (*room).contains((dung_x, dung_y));
            }
            if in_room
            {
                image.put_pixel(pixel_x, pixel_y, Rgb([255, 255, 255]));
            }
            for point in central_points
            {
                if dung_x as f64 == point.x && dung_y as f64 == point.y
                { 
                    image.put_pixel(pixel_x, pixel_y, Rgb([255, 0, 0]));
                }
            }
        }
    }
    image.save(filename).unwrap();
}

pub fn draw_triangulation(dungeon_width:u32, dungeon_height:u32, points:&Vec<delaunator::Point>, triangulation: &delaunator::Triangulation)
{  
    const LINE_COLOR: &str = "chartreuse";
    const POINT_COLOR: &str = "red";
    const POINT_SIZE: f64 = 0.5;
    const LINE_WIDTH: f64 = 0.25;
    const HULL_POINT_COLOR: &str = "red";
    let svg_circles = points
        .iter()
        .enumerate()
        .fold(String::new(), |acc, (i, p)| {
            let color = if triangulation.hull.contains(&i) {
                HULL_POINT_COLOR
            } else {
                POINT_COLOR
            };
            acc + &format!(
                r#"<circle cx="{x}" cy="{y}" r="{size}" fill="{color}"/>"#,
                x = p.x,
                y = p.y,
                size = POINT_SIZE,
                color = color
            )
        });


    // generate SVG
    let contents = format!(
        r#"
<svg viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
<rect width="100%" height="100%" fill="white" />
    {circles}
    {lines}
</svg>"#,
        width = dungeon_width+10,
        height = dungeon_height+10,
        circles = svg_circles,
        lines = (0..triangulation.triangles.len()).fold(String::new(), |acc, e| {
            if e > triangulation.halfedges[e] || triangulation.halfedges[e] == delaunator::EMPTY {
                let start = &points[triangulation.triangles[e]];
                let end = &points[triangulation.triangles[delaunator::next_halfedge(e)]];
                let color = LINE_COLOR;
                acc + &format!(r#"<line x1="{x0}" y1="{y0}" x2="{x1}" y2="{y1}" style="stroke:{color};stroke-width:{width}" />"#, x0 = start.x, y0 = start.y, x1=end.x, y1=end.y, width = LINE_WIDTH, color = color)
            } else {
                acc
            }
        })
    );
    let create_file_result = std::fs::File::create("triangulation.svg");
    let mut svg_file = match create_file_result
    {
        Ok(file) => 
        {
            file
        },
        Err(error) => 
        {
            panic!("failed to create file: {:?}", error);
        }
    };
    svg_file.write_all(contents.as_bytes());
}